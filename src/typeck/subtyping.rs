use crate::ast::core::{Term, Ty};
use crate::ctx::{TyCtxt, TyCtxtEntry};

use crate::trace;
#[cfg(feature = "trace")]
use tracing::instrument;

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn subtype(ctx: &mut TyCtxt, ty1: &Ty, ty2: &Ty) -> Option<()> {
    trace!("subtype/enter");
    match (ty1, ty2) {
        // <:Var
        (Ty::Var(idx1), Ty::Var(idx2)) if idx1 == idx2 && ctx.contains_uvar(*idx1) => (),
        // <:Unit; <:Bool (not in paper)
        (Ty::Unit, Ty::Unit) | (Ty::Bool, Ty::Bool) => (),
        // <:Exvar
        (Ty::ExstVar(idx1), Ty::ExstVar(idx2)) if idx1 == idx2 && ctx.contains_evar(*idx1) => {}
        // <:->
        (Ty::Arrow(a1, a2), Ty::Arrow(b1, b2)) => {
            subtype(ctx, &*b1, &*a1)?;
            let a2 = a2.clone().subst_ctx(ctx);
            let b2 = b2.clone().subst_ctx(ctx);
            subtype(ctx, &a2, &b2)?;
        }
        // <:∀L
        (Ty::Forall(a), b) => {
            // Get a fresh evar index.
            let evar = ctx.fresh_evar();
            ctx.add_marker(evar);
            ctx.add_unsolved(evar);
            let a = a.clone().subst_uvar0_bare(&Ty::ExstVar(evar));
            subtype(ctx, &a, b)?;
            // Drop everything after the evar at the end.
            ctx.drop_after_marker(evar)?;
        }
        // <:∀R
        (a, Ty::Forall(b)) => {
            // Put a new uvar in context
            ctx.add_uvar();
            subtype(ctx, a, b)?;
            // Drop everything after that uvar at the end.
            ctx.drop_after_uvar(0);
        }
        // <:InstantiateL
        (Ty::ExstVar(evar), ty) => {
            if ty.contains_evar(*evar) {
                trace!("subtype/leave: <:IL: occurs");
                return None;
            }
            inst_left(ctx, *evar, ty)?;
        }
        // <:InstantiateR
        (ty, Ty::ExstVar(evar)) => {
            if ty.contains_evar(*evar) {
                trace!("subtype/leave: <:IR: occurs");
                return None;
            }
            inst_right(ctx, ty, *evar)?;
        }
        _ => {
            trace!("subtype/leave: not a subtype");
            return None;
        }
    }

    trace!("subtype/leave: ok");
    Some(())
}

// evar :<= ty
#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn inst_left(ctx: &mut TyCtxt, evar: usize, ty: &Ty) -> Option<()> {
    trace!("inst_left/enter");
    if !ctx.contains_evar(evar) {
        trace!("inst_left/leave: no evar in ctx");
        return None;
    }

    match ty {
        // InstLSolve
        ty if ty.is_mono_wellformed_in(ctx.slice_until_evar(evar)?) => {
            ctx.solve_evar(evar, ty.clone())?;
        }
        // InstLReach
        Ty::ExstVar(beta) => {
            if !ctx.contains_evar(*beta) {
                trace!("inst_left/leave: InstLReach: no beta in ctx");
                return None;
            }
            ctx.solve_evar(*beta, Ty::ExstVar(evar));
        }
        // InstLArr
        Ty::Arrow(a1, a2) => {
            let alpha2 = ctx.fresh_evar();
            let alpha1 = ctx.fresh_evar();
            ctx.insert_unsolved_before_evar(evar, alpha2)?;
            ctx.insert_unsolved_before_evar(evar, alpha1)?;
            ctx.solve_evar(
                evar,
                Ty::Arrow(Box::new(Ty::ExstVar(alpha1)), Box::new(Ty::ExstVar(alpha2))),
            )?;
            inst_right(ctx, a1, alpha1)?;
            let a2 = a2.clone().subst_ctx(ctx);
            inst_left(ctx, alpha2, &a2)?;
        }
        // InstLAllR
        Ty::Forall(body) => {
            ctx.add_uvar();
            inst_left(ctx, evar, body)?;
            ctx.drop_after_uvar(0)?;
        }
        _ => {
            trace!("inst_left/leave: not inst'able");
            return None;
        }
    }

    trace!("inst_left/leave: ok");
    Some(())
}

// ty =<: evar
#[cfg_attr(feature = "trace", instrument(level = "trace"))]
pub fn inst_right(ctx: &mut TyCtxt, ty: &Ty, evar: usize) -> Option<()> {
    trace!("inst_right/enter");
    if !ctx.contains_evar(evar) {
        trace!("inst_right/leave: no evar in ctx");
        return None;
    }

    match ty {
        // InstRSolve
        ty if ty.is_mono_wellformed_in(ctx.slice_until_evar(evar)?) => {
            ctx.solve_evar(evar, ty.clone())?;
        }
        // InstRReach
        Ty::ExstVar(beta) => {
            if !ctx.contains_evar(*beta) {
                trace!("inst_right/leave: InstRReach: no beta in ctx");
                return None;
            }
            ctx.solve_evar(*beta, Ty::ExstVar(evar));
        }
        // InstRArr
        Ty::Arrow(a1, a2) => {
            let alpha2 = ctx.fresh_evar();
            let alpha1 = ctx.fresh_evar();
            ctx.insert_unsolved_before_evar(evar, alpha2)?;
            ctx.insert_unsolved_before_evar(evar, alpha1)?;
            ctx.solve_evar(
                evar,
                Ty::Arrow(Box::new(Ty::ExstVar(alpha1)), Box::new(Ty::ExstVar(alpha2))),
            )?;
            inst_left(ctx, alpha1, a1)?;
            let a2 = a2.clone().subst_ctx(ctx);
            inst_right(ctx, &a2, alpha2)?;
        }
        // InstRAllL
        Ty::Forall(body) => {
            let beta = ctx.fresh_evar();
            ctx.add_marker(beta);
            ctx.add_unsolved(beta);
            let body = body.clone().subst_uvar0_bare(&Ty::ExstVar(beta));
            inst_right(ctx, &body, evar)?;
            ctx.drop_after_marker(beta)?;
        }
        _ => {
            trace!("inst_right/leave: not inst'able");
            return None;
        }
    }

    trace!("inst_right/leave: ok");
    Some(())
}
