use std::collections::HashMap;

use crate::ast::{Term, Ty};

// Typing context
pub type Context = HashMap<String, Option<Ty>>;

pub fn infer(ctx: &mut Context, term: &Term) -> Option<Ty> {
    match term {
        Term::Var(x) => ctx.get(x).cloned().flatten(),
        Term::Bool(_) => Some(Ty::Bool),
        Term::Ann(term, ty) => check(ctx, term, ty),
        Term::App(func, term) => match infer(ctx, func)? {
            Ty::Lam(ty1, ty2) => {
                check(ctx, term, &ty1)?;
                Some(*ty2)
            }
            Ty::PolyLam(var, body) => poly_infer(ctx, var.as_ref(), body.as_ref(), term),
            _ => None,
        },
        Term::PolyApp(term, ty) => {
            let ty1 = infer(ctx, term)?;
            let outermost_var = find_outermost_polyvar(&ty1)?;
            ctx.insert(outermost_var, Some(ty.clone()));
            Some(poly_subst(ctx, ty1))
        }
        _ => None,
    }
}

pub fn find_outermost_polyvar(ty: &Ty) -> Option<String> {
    match ty {
        Ty::Bool => None,
        Ty::Var(v) => Some(v.to_string()),
        Ty::Lam(t1, t2) => find_outermost_polyvar(t1).or_else(|| find_outermost_polyvar(t2)),
        Ty::PolyLam(var, _) => Some(var.to_string()),
    }
}

pub fn poly_infer(ctx: &mut Context, var: &str, body: &Ty, term: &Term) -> Option<Ty> {
    ctx.insert(var.to_string(), None);
    match body {
        Ty::Bool => Some(Ty::Bool),
        Ty::Var(v) => ctx.get(v).cloned().flatten(),
        Ty::Lam(t1, t2) => {
            if let Ty::Var(v) = t1.as_ref() {
                let inferred_t1 = infer(ctx, term)?;
                ctx.insert(v.to_string(), Some(inferred_t1));
                Some(poly_subst(ctx, *t2.clone()))
            } else {
                None
            }
        }
        Ty::PolyLam(var, body) => poly_infer(ctx, var, body, term),
    }
}

pub fn poly_subst(ctx: &mut Context, ty: Ty) -> Ty {
    match ty {
        Ty::Var(ref v) if ctx.contains_key(v) => ctx.get(v).unwrap().clone().unwrap_or(ty),
        Ty::Lam(t1, t2) => Ty::Lam(
            Box::new(poly_subst(ctx, *t1)),
            Box::new(poly_subst(ctx, *t2)),
        ),
        Ty::PolyLam(v, body) => {
            let body = poly_subst(ctx, *body);
            if ctx.contains_key(&v) {
                body
            } else {
                Ty::PolyLam(v, Box::new(body))
            }
        }
        _ => ty,
    }
}

pub fn check(ctx: &mut Context, term: &Term, ty: &Ty) -> Option<Ty> {
    if let Ty::Var(v) = ty {
        if ctx.get(v).cloned().flatten().is_none() {
            let t = infer(ctx, term)?;
            ctx.insert(v.to_string(), Some(t.clone()));
            return Some(t);
        }
    }
    match term {
        Term::Cond(cond, iftrue, iffalse) => {
            check(ctx, cond, &Ty::Bool)?;
            check(ctx, iftrue, ty)?;
            check(ctx, iffalse, ty)?;
            Some(ty.clone())
        }
        Term::Abs(x, term) => {
            if let Ty::Lam(t1, t2) = ty {
                ctx.insert(x.to_string(), Some(*t1.clone()));
                let res;
                if check(ctx, &*term, t2).is_some() {
                    res = Some(ty.clone())
                } else {
                    res = None;
                    ctx.remove(x);
                }
                res
            } else {
                None
            }
        }
        Term::PolyAbs(var, body) => {
            if let Ty::PolyLam(v2, t2) = ty {
                if v2 == var && check(ctx, &*body, t2).is_some() {
                    Some(ty.clone())
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => match infer(ctx, term) {
            Some(ty2) if ty == &ty2 => Some(ty.clone()),
            _ => None,
        },
    }
}
