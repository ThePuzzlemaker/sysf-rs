use std::collections::HashMap;

use crate::ctx::{TyCtxt, TyCtxtView};

use crate::trace;
#[cfg(feature = "trace")]
use tracing::instrument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Unit,
    Appl(Box<Term>, Box<Term>),
    Lambda(Box<Term>),
    Bool(bool),
    Var(usize),
    Ann(Box<Term>, Ty),
    TypeAppl(Box<Term>, Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Unit,
    Arrow(Box<Ty>, Box<Ty>),
    Forall(Box<Ty>),
    Var(usize),
    ExstVar(usize),
}

impl Ty {
    pub fn subst_ctx(self, ctx: &TyCtxt) -> Ty {
        let solved = ctx.get_solved();
        let mut zelf = self;
        loop {
            if !zelf.contains_evars(&solved) {
                break zelf;
            }
            zelf = zelf.subst_ctx_once(ctx, &solved);
        }
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
    fn subst_ctx_once(self, ctx: &TyCtxt, solved: &HashMap<usize, &Ty>) -> Ty {
        trace!("ty/subst_ctx_once/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::Var(_) => self,
            Ty::Forall(body) => Ty::Forall(Box::new(body.subst_ctx_once(ctx, solved))),
            Ty::Arrow(inp, out) => Ty::Arrow(
                Box::new(inp.subst_ctx_once(ctx, solved)),
                Box::new(out.subst_ctx_once(ctx, solved)),
            ),
            Ty::ExstVar(evar) => solved.get(&evar).copied().cloned().unwrap_or(self),
        };
        trace!(?res, "ty/subst_ctx_once/leave");
        res
    }

    /// Substitute the bare body of a forall,
    /// potentially with remaining unbound Var(0)s.
    pub fn subst_uvar0_bare(self, with: &Ty) -> Ty {
        self.subst_uvar0_bare_(with, 0)
    }

    pub fn subst_uvar0(self, with: &Ty) -> Ty {
        self.subst_uvar0_(with, 0)
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace"))]
    fn subst_uvar0_bare_(self, with: &Ty, depth: usize) -> Ty {
        trace!("ty/subst_uvar0_bare/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::ExstVar(_) => self,
            Ty::Forall(body) => Ty::Forall(Box::new(body.subst_uvar0_bare_(with, depth + 1))),
            Ty::Arrow(inp, out) => Ty::Arrow(
                Box::new(inp.subst_uvar0_bare_(with, depth)),
                Box::new(out.subst_uvar0_bare_(with, depth)),
            ),
            Ty::Var(idx) => {
                if idx == depth {
                    with.clone()
                } else {
                    self
                }
            }
        };
        trace!(?res, "ty/subst_uvar0_bare/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace"))]
    fn subst_uvar0_(self, with: &Ty, depth: usize) -> Ty {
        trace!("ty/subst_uvar0/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::ExstVar(_) => self,
            Ty::Forall(body) => {
                let subst = body.subst_uvar0_(with, depth + 1);
                if depth == 0 {
                    subst
                } else {
                    Ty::Forall(Box::new(subst))
                }
            }
            Ty::Arrow(inp, out) => Ty::Arrow(
                Box::new(inp.subst_uvar0_(with, depth)),
                Box::new(out.subst_uvar0_(with, depth)),
            ),
            Ty::Var(idx) => {
                if idx + 1 == depth {
                    with.clone()
                } else {
                    self
                }
            }
        };
        trace!(?res, "ty/subst_uvar0/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace"))]
    pub fn contains_evar(&self, idx: usize) -> bool {
        trace!("ty/contains_evar/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::Var(_) => false,
            Ty::ExstVar(eidx) => idx == *eidx,
            Ty::Arrow(inp, out) => inp.contains_evar(idx) || out.contains_evar(idx),
            Ty::Forall(body) => body.contains_evar(idx),
        };
        trace!(%res, "ty/contains_evar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace"))]
    pub fn contains_evars(&self, evars: &HashMap<usize, &Ty>) -> bool {
        trace!("ty/contains_evars/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::Var(_) => false,
            Ty::ExstVar(eidx) => evars.contains_key(eidx),
            Ty::Arrow(inp, out) => inp.contains_evars(evars) || out.contains_evars(evars),
            Ty::Forall(body) => body.contains_evars(evars),
        };
        trace!(%res, "ty/contains_evars/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
    pub fn is_mono_wellformed_in(&self, ctx: TyCtxtView) -> bool {
        trace!("ty/is_mono_wellformed_in/enter");
        let res = match self {
            Ty::Bool | Ty::Unit => true,
            Ty::Var(idx) => ctx.contains_uvar(*idx),
            Ty::Forall(_) => false,
            Ty::ExstVar(eidx) => ctx.contains_evar(*eidx),
            Ty::Arrow(inp, out) => inp.is_mono_wellformed_in(ctx) && out.is_mono_wellformed_in(ctx),
        };
        trace!(%res, "ty/is_mono_wellformed_in/leave");
        res
    }

    pub fn is_wellformed_in(&self, ctx: &TyCtxt) -> bool {
        self.is_wellformed_in_(ctx, 0)
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
    fn is_wellformed_in_(&self, ctx: &TyCtxt, depth: usize) -> bool {
        trace!("ty/is_wellformed_in/enter");
        let res = match self {
            Ty::Bool | Ty::Unit => true,
            Ty::Var(idx) => *idx < depth,
            Ty::Forall(body) => body.is_wellformed_in_(ctx, depth + 1),
            Ty::ExstVar(eidx) => ctx.contains_evar(*eidx),
            Ty::Arrow(inp, out) => {
                inp.is_wellformed_in_(ctx, depth) && out.is_wellformed_in_(ctx, depth)
            }
        };
        trace!(%res, "ty/is_wellformed_in/leave");
        res
    }
}
