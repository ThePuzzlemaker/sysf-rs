use std::collections::HashMap;

use crate::ast::{Term, Ty};

use crate::trace;

#[cfg(feature = "trace")]
use tracing::instrument;

type Context = HashMap<String, Option<Ty>>;

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn infer(ctx: &mut Context, term: &Term) -> Option<Ty> {
    trace!("poly_appl/enter");
    let res = match term {
        Term::Var(x) => ctx.get(x).cloned().flatten().map(|x| poly_subst(ctx, x)),
        Term::Bool(_) => Some(Ty::Bool),
        Term::Appl(func, term) => match infer(ctx, func)? {
            Ty::Arrow(t1, t2) => {
                check(ctx, term, &t1)?;
                Some(*t2)
            }
            // When we apply something to a polymorphic lambda, there's a
            // chance we can infer the full monomorphic type based on the
            // applicand. We thus work inside the lambda ("poly-unroll" it) and
            // apply the rules for inferring polymorphic application.
            Ty::Forall(pv, pty) => poly_appl(ctx, pv, *pty, term),
            _ => None,
        },
        Term::PolyAppl(func, ty) => {
            if let Ty::Forall(pv, pty) = infer(ctx, func)? {
                trace!(%pv, ?ty, "ctx/insert");
                ctx.insert(pv.clone(), Some(ty.clone()));
                let res = poly_subst(ctx, *pty);
                trace!(%pv, "ctx/remove");
                ctx.remove(&pv);
                Some(res)
            } else {
                None
            }
        }
        Term::Ann(term, ty) => check(ctx, term, ty),
        _ => None,
    };
    trace!(?res, "infer/exit");
    res
}

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn poly_appl(ctx: &mut Context, pv: String, pty: Ty, term: &Term) -> Option<Ty> {
    trace!("poly_appl/enter");
    // We mark that the polyvar `pv` is in the scope, but not bound yet.
    trace!(%pv, "ctx/insert(none)");
    ctx.insert(pv.clone(), None);
    let res = match pty {
        Ty::Arrow(t1, t2) => {
            // We check if the lambda's input type is a variable.
            match *t1 {
                // If it is, then we can infer the polyvar!
                Ty::Var(pv2) => {
                    // We try to infer the type of the applicand.
                    let appty = infer(ctx, term)?;
                    // If we can, we add it to the context.
                    trace!(pv = %pv2, ty = ?appty, "ctx/insert");
                    ctx.insert(pv2.clone(), Some(appty));
                    // We then substitute the lambda's output.
                    let res = poly_subst(ctx, *t2);
                    // We clean up the context (as this polyvar will no longer be
                    // in scope), and return the substituted output type.
                    trace!(pv = %pv2, "ctx/remove");
                    ctx.remove(&pv2);
                    Some(res)
                }
                t1 => {
                    // In the case that the values is *not* a variable, we check
                    // the applicand with the input type given to us.
                    check(ctx, term, &t1)?;
                    // If we were able to check it, we are able to infer the
                    // type of the application to be the output type of the
                    // lambda. This is the same as the rule for application
                    // outside of polymorphic lambdas.
                    Some(*t2)
                }
            }
        }
        // If we're given nested poly-lambdas, we'll want to recursively infer
        // using poly-unrolling.
        Ty::Forall(pv, pty) => poly_appl(ctx, pv, *pty, term),
        // We can't infer anything else, as application would have no meaning
        // for any other type.
        _ => None,
    }?;

    let res = Some(if contains_pvar(&pv, &res) {
        match ctx.get(&pv) {
            // If the poly-var is not bound and the result type still has instances of
            // that variable, we'll want to poly-roll it back up.
            None => Ty::Forall(pv, Box::new(res)),
            Some(None) => {
                trace!(%pv, "ctx/remove");
                ctx.remove(&pv);
                Ty::Forall(pv, Box::new(res))
            }
            // If it was bound, however, we'll want to substitute the type
            // again, just to make sure no known variables are left.
            // We don't need to roll it back up, as we already know the type
            // bound to the polyvar.
            // This (hopefully) shouldn't happen in practice.
            Some(_) => {
                trace!(%pv, "ctx/remove");
                ctx.remove(&pv);
                poly_subst(ctx, res)
            }
        }
    } else {
        // If the result type doesn't contain any instances of the polyvar, we
        // don't need to roll it back up.
        res
    });
    trace!(?res, "poly_appl/exit");
    res
}

#[cfg_attr(feature = "trace", instrument(level = "trace"))]
pub fn contains_pvar(var: &str, ty: &Ty) -> bool {
    trace!("contains_pvar/enter");
    let res = match ty {
        Ty::Bool => false,
        Ty::Var(v) => v == var,
        Ty::Arrow(t1, t2) => contains_pvar(var, t1) || contains_pvar(var, t2),
        Ty::Forall(pv, pty) => var == pv || contains_pvar(var, pty),
    };
    trace!(res, "contains_pvar/exit");
    res
}

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn poly_subst(ctx: &mut Context, ty: Ty) -> Ty {
    trace!("poly_subst/enter");
    let res = match ty {
        Ty::Var(ref v) => ctx.get(v).cloned().flatten().unwrap_or(ty),
        Ty::Arrow(t1, t2) => Ty::Arrow(
            Box::new(poly_subst(ctx, *t1)),
            Box::new(poly_subst(ctx, *t2)),
        ),
        Ty::Forall(pv, body) => {
            let body = poly_subst(ctx, *body);
            if ctx.contains_key(&pv) {
                trace!(%pv, "ctx/remove");
                ctx.remove(&pv);
                body
            } else {
                Ty::Forall(pv, Box::new(body))
            }
        }
        _ => ty,
    };
    trace!(?res, "poly_subst/exit");
    res
}

#[cfg_attr(feature = "trace", instrument(level = "trace", skip(ctx)))]
pub fn check(ctx: &mut Context, term: &Term, ty: &Ty) -> Option<Ty> {
    trace!("check/enter");
    if let Ty::Forall(pv, pty) = ty {
        // If we're checking against a forall, we can unroll the forall and
        // check against the inner type.
        trace!(%pv, "ctx/insert(none)");
        ctx.insert(pv.clone(), None);
        check(ctx, term, pty)?;
        // We'll want to take it out of scope at the end, of course.
        trace!(%pv, "ctx/remove");
        ctx.remove(pv);
        // We'll also want to roll it back up at the end (in this case we just
        // need to return the type given to us)
        return Some(ty.clone());
    } else if let Ty::Var(v) = ty {
        // If we're checking against a variable, as long as that variable is in
        // scope, it's valid to check against.
        ctx.get(v)?;
        // We return the variable given to us.
        return Some(ty.clone());
    }

    let res = match term {
        Term::Lambda(x, term) => match ty {
            Ty::Arrow(t1, t2) => {
                trace!(%x, ty = ?t1, "ctx/insert");
                ctx.insert(x.to_string(), Some(*t1.clone()));
                let res;
                if check(ctx, &*term, t2).is_some() {
                    res = Some(ty.clone());
                } else {
                    res = None;
                }
                trace!(%x, "ctx/remove");
                ctx.remove(x);
                res
            }
            _ => None,
        },
        _ => match infer(ctx, term) {
            Some(ty2) if ty == &ty2 => Some(ty.clone()),
            _ => None,
        },
    };
    trace!(?res, "check/exit");
    res
}
