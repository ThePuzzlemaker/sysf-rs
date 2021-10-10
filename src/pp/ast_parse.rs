use crate::ast::parse::{Term, Ty};
use pretty::{DocAllocator, DocBuilder};

type Builder<'a, A> = DocBuilder<'a, A, ()>;

pub fn pp_parse_term<'a, A: DocAllocator<'a, ()>>(term: Term, arena: &'a A) -> Builder<'a, A>
where
    <A as DocAllocator<'a, ()>>::Doc: Clone,
{
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
        Term::TypeAppl(term, ty) => arena
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

pub fn pp_parse_ty<'a, A: DocAllocator<'a, ()>>(ty: Ty, arena: &'a A) -> Builder<'a, A>
where
    <A as DocAllocator<'a, ()>>::Doc: Clone,
{
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
