#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<I = String> {
    Appl(Box<Term<I>>, Box<Term<I>>),
    Lambda(I, Box<Term<I>>),
    Bool(bool),
    Var(I),
    Ann(Box<Term<I>>, Ty<I>),
    PolyAppl(Box<Term<I>>, Ty<I>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty<I = String> {
    Bool,
    Arrow(Box<Ty<I>>, Box<Ty<I>>),
    Forall(I, Box<Ty<I>>),
    Var(I),
}

// TODO: actually good binding
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum CtxVar {
//     Term(isize),
//     Type(isize),

// }
// pub fn ty_to_debruijn(stack: &mut Vec<String>, ty: Ty) -> Ty<isize> {
//     match ty {
//         Ty::Bool => Ty::Bool,
//         Ty::Arrow(t1, t2) => Ty::Arrow(
//             Box::new(ty_to_debruijn(stack, *t1)),
//             Box::new(ty_to_debruijn(stack, *t2)),
//         ),
//         Ty::PolyArrow(pv, pty) => {
//             stack.push(pv);
//         }
//     }
// }
