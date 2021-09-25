use crate::ast::core::{Term, Ty};
use crate::ctx::{TyCtxt, TyCtxtEntry};
use subtyping::subtype;

use crate::trace;
#[cfg(feature = "trace")]
use tracing::instrument;

pub mod subtyping;

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn infer(ctx: &mut TyCtxt, term: &Term) -> Option<Ty> {
    trace!("infer/enter");
    let res = match term {
        // Var
        Term::Var(idx) => ctx.get_term_var(*idx).cloned()?,
        // Anno
        Term::Ann(term, ty) => {
            if !ty.is_wellformed_in(ctx) {
                trace!("infer/leave: Anno: type is not well-formed");
                return None;
            }
            check(ctx, term, ty)?;
            ty.clone()
        }
        // 1I=>
        Term::Unit => Ty::Unit,
        // BoolI=> (not in paper)
        Term::Bool(_) => Ty::Bool,
        // ->I=>
        Term::Lambda(body) => {
            let alpha = ctx.fresh_evar();
            let beta = ctx.fresh_evar();
            ctx.add_unsolved(alpha);
            ctx.add_unsolved(beta);
            ctx.add_term_var(Ty::ExstVar(alpha));
            check(ctx, body, &Ty::ExstVar(beta))?;
            ctx.drop_after_term_var(0);
            Ty::Arrow(Box::new(Ty::ExstVar(alpha)), Box::new(Ty::ExstVar(beta)))
        }
        // ->E
        Term::Appl(func, arg) => {
            let fty = infer(ctx, &*func)?;
            let fty = fty.subst_ctx(ctx);
            infer_appl(ctx, &fty, &*arg)?
        }
        // TypeApp=>
        Term::TypeAppl(term, ty) => {
            let fty = infer(ctx, &*term)?;
            if !ty.is_mono_wellformed_in((&*ctx).into()) {
                trace!("infer/leave: TypeApp=>: not a monotype");
                return None;
            }
            fty.subst_uvar0(ty)
        }
        _ => {
            trace!("infer/leave: not inferrable");
            return None;
        }
    };
    trace!(?res, "infer/leave: ok");
    Some(res)
}

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn check(ctx: &mut TyCtxt, term: &Term, ty: &Ty) -> Option<()> {
    trace!("check/enter");

    match (term, ty) {
        // 1I; BoolI (not in paper)
        (_, Ty::Unit | Ty::Bool) => (),
        // ∀I
        (_, Ty::Forall(body)) => {
            ctx.add_uvar();
            check(ctx, term, &*body)?;
            ctx.drop_after_uvar(0);
        }
        // ->I
        (Term::Lambda(body), Ty::Arrow(inp, out)) => {
            ctx.add_term_var(*inp.clone());
            check(ctx, &*body, &*out)?;
            ctx.drop_after_term_var(0);
        }
        // Sub
        _ => {
            let tya = infer(ctx, term)?;
            let tya = tya.subst_ctx(ctx);
            let tyb = ty.clone().subst_ctx(ctx);
            subtype(ctx, &tya, &tyb)?;
        }
    }
    trace!("check/leave: ok");
    Some(())
}

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn infer_appl(ctx: &mut TyCtxt, ty: &Ty, term: &Term) -> Option<Ty> {
    trace!("infer_appl/enter");
    let res = match ty {
        // ∀App
        Ty::Forall(body) => {
            let evar = ctx.fresh_evar();
            ctx.add_unsolved(evar);
            let body = body.clone().subst_uvar0_bare(&Ty::ExstVar(evar));
            infer_appl(ctx, &body, term)?
        }
        // âApp
        Ty::ExstVar(evar) if ctx.contains_evar(*evar) => {
            let alpha2 = ctx.fresh_evar();
            let alpha1 = ctx.fresh_evar();
            ctx.insert_unsolved_before_evar(*evar, alpha2);
            ctx.insert_unsolved_before_evar(*evar, alpha1);
            ctx.solve_evar(
                *evar,
                Ty::Arrow(Box::new(Ty::ExstVar(alpha1)), Box::new(Ty::ExstVar(alpha2))),
            )?;
            check(ctx, term, &Ty::ExstVar(alpha1))?;
            Ty::ExstVar(alpha2)
        }
        // ->App
        Ty::Arrow(inp, out) => {
            check(ctx, term, &*inp)?;
            *out.clone()
        }
        _ => {
            trace!("infer_appl/leave: not inferrable");
            return None;
        }
    };
    trace!(?res, "infer_appl/leave: ok");
    Some(res)
}
