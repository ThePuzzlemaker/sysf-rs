use crate::ast::core::{Term, Ty};
use pretty::{Arena, BuildDoc, DocAllocator, DocBuilder};

type Builder<'a> = DocBuilder<'a, Arena<'a>, ()>;

pub fn pp_core_term<'a>(term: Term, arena: &'a Arena<'a>) -> Builder<'a> {
    match term {
        Term::Var(idx) => arena.text(format!("{}", idx)),
        Term::Bool(b) => arena.text(format!("{}", b)),
        Term::Unit => arena.text("()"),
        Term::Lambda(body) => arena
            .text("\\ _")
            .append(arena.softline())
            .append(pp_core_term(*body, arena))
            .nest(2)
            .parens(),
        Term::Appl(func, arg) => arena
            .intersperse(
                [
                    arena.text("$"),
                    pp_core_term(*func, arena),
                    pp_core_term(*arg, arena),
                ],
                arena.softline(),
            )
            .nest(2)
            .parens(),
        Term::Ann(term, ty) => arena
            .intersperse(
                [
                    arena.text(":"),
                    pp_core_term(*term, arena).nest(2),
                    pp_core_ty(ty, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
        Term::TypeAppl(term, ty) => arena
            .intersperse(
                [
                    arena.text("[]"),
                    pp_core_term(*term, arena).nest(2),
                    pp_core_ty(ty, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
    }
}

pub fn pp_core_ty<'a>(ty: Ty, arena: &'a Arena<'a>) -> Builder<'a> {
    match ty {
        Ty::Bool => arena.text("bool"),
        Ty::Unit => arena.text("unit"),
        Ty::Var(idx) => arena.text(format!("{}", idx)),
        Ty::Arrow(inp, out) => arena
            .intersperse(
                [
                    pp_core_ty(*inp, arena),
                    arena.text("->").nest(2),
                    pp_core_ty(*out, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
        Ty::Forall(body) => arena
            .intersperse(
                [
                    arena.text("_"),
                    arena.text("=>").nest(2),
                    pp_core_ty(*body, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
        Ty::ExstVar(idx) => arena.text(format!("'__exst{}", idx)),
    }
}
