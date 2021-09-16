use crate::ast::parse::{Term, Ty};
use pretty::{Arena, BuildDoc, DocAllocator, DocBuilder};

type Builder<'a> = DocBuilder<'a, Arena<'a>, ()>;

pub fn pp_parse_term<'a>(term: Term, arena: &'a Arena<'a>) -> Builder<'a> {
    match term {
        Term::Var(name) => arena.text(name),
        Term::Bool(b) => arena.text(format!("{}", b)),
        Term::Unit => arena.text("()"),
        Term::Lambda(name, body) => arena
            .text(format!("\\ {}", name))
            .append(arena.softline())
            .append(pp_parse_term(*body, arena))
            .nest(2)
            .parens(),
        Term::Appl(func, arg) => arena
            .intersperse(
                [
                    arena.text("$"),
                    pp_parse_term(*func, arena),
                    pp_parse_term(*arg, arena),
                ],
                arena.softline(),
            )
            .nest(2)
            .parens(),
        Term::Ann(term, ty) => arena
            .intersperse(
                [
                    arena.text(":"),
                    pp_parse_term(*term, arena).nest(2),
                    pp_parse_ty(ty, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
        Term::PolyAppl(term, ty) => arena
            .intersperse(
                [
                    arena.text("[]"),
                    pp_parse_term(*term, arena).nest(2),
                    pp_parse_ty(ty, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
    }
}

pub fn pp_parse_ty<'a>(ty: Ty, arena: &'a Arena<'a>) -> Builder<'a> {
    match ty {
        Ty::Bool => arena.text("bool"),
        Ty::Unit => arena.text("unit"),
        Ty::Var(name) => arena.text(name),
        Ty::Arrow(inp, out) => arena
            .intersperse(
                [
                    pp_parse_ty(*inp, arena),
                    arena.text("->").nest(2),
                    pp_parse_ty(*out, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
        Ty::Forall(name, body) => arena
            .intersperse(
                [
                    arena.text(name),
                    arena.text("=>").nest(2),
                    pp_parse_ty(*body, arena).nest(2),
                ],
                arena.softline(),
            )
            .parens(),
    }
}
