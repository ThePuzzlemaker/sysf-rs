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
    PolyAppl(Box<Term>, Ty),
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
    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
    pub fn subst_ctx(self, ctx: &mut TyCtxt) -> Ty {
        trace!("ty/subst_ctx/enter");
        let solved = ctx.get_solved();
        trace!(?solved, "ty/subst_ctx/solved");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::Var(_) => self,
            Ty::Forall(body) => Ty::Forall(Box::new(body.subst_ctx(ctx))),
            Ty::Arrow(inp, out) => {
                Ty::Arrow(Box::new(inp.subst_ctx(ctx)), Box::new(out.subst_ctx(ctx)))
            }
            Ty::ExstVar(evar) => solved.get(&evar).copied().cloned().unwrap_or(self),
        };
        trace!(?res, "ty/subst_ctx/leave");
        res
    }

    /// Substitute the bare body of a forall,
    /// potentially with remaining unbound Var(0)s.
    pub fn subst_uvar0(self, with: &Ty) -> Ty {
        self.subst_uvar0_(with, 0)
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace"))]
    fn subst_uvar0_(self, with: &Ty, depth: usize) -> Ty {
        trace!("ty/subst_uvar0/enter");
        let res = match self {
            Ty::Bool | Ty::Unit | Ty::ExstVar(_) => self,
            Ty::Forall(body) => Ty::Forall(Box::new(body.subst_uvar0_(with, depth + 1))),
            Ty::Arrow(inp, out) => Ty::Arrow(
                Box::new(inp.subst_uvar0_(with, depth)),
                Box::new(out.subst_uvar0_(with, depth)),
            ),
            Ty::Var(idx) => {
                if idx == depth {
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
            Ty::Arrow(inp, out) => inp.contains_evar(idx) && out.contains_evar(idx),
            Ty::Forall(body) => body.contains_evar(idx),
        };
        trace!(%res, "ty/contains_evar/leave");
        res
    }

    #[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
    pub fn is_mono_wellformed_in(&self, ctx: TyCtxtView) -> bool {
        trace!("ty/is_mono_wellformed_in/enter");
        let res = match self {
            Ty::Bool | Ty::Unit => true,
            Ty::Var(_) | Ty::Forall(_) => false,
            Ty::ExstVar(eidx) => ctx.contains_evar(*eidx),
            Ty::Arrow(inp, out) => inp.is_mono_wellformed_in(ctx) && out.is_mono_wellformed_in(ctx),
        };
        trace!(%res, "ty/is_mono_wellformed_in/leave");
        res
    }
}
