use std::collections::VecDeque;

use super::core;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Unit,
    Appl(Box<Term>, Box<Term>),
    Lambda(String, Box<Term>),
    Bool(bool),
    Var(String),
    Ann(Box<Term>, Ty),
    TypeAppl(Box<Term>, Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Unit,
    Bool,
    Arrow(Box<Ty>, Box<Ty>),
    Forall(String, Box<Ty>),
    Var(String),
}

impl Term {
    pub fn into_core(self) -> Option<core::Term> {
        self.into_core_(&mut VecDeque::new())
    }

    fn into_core_(self, ctx: &mut VecDeque<String>) -> Option<core::Term> {
        Some(match self {
            Term::Appl(func, arg) => core::Term::Appl(
                Box::new(func.into_core_(ctx)?),
                Box::new(arg.into_core_(ctx)?),
            ),
            Term::Lambda(name, body) => {
                ctx.push_front(name);
                let new_body = body.into_core_(ctx);
                ctx.pop_front();
                core::Term::Lambda(Box::new(new_body?))
            }
            Term::Bool(b) => core::Term::Bool(b),
            Term::Unit => core::Term::Unit,
            Term::Var(name) => {
                let idx =
                    ctx.iter()
                        .enumerate()
                        .find_map(|(idx, s)| if s == &name { Some(idx) } else { None })?;
                core::Term::Var(idx)
            }
            Term::Ann(term, ty) => {
                core::Term::Ann(Box::new(term.into_core_(ctx)?), ty.into_core()?)
            }
            Term::TypeAppl(term, ty) => {
                core::Term::TypeAppl(Box::new(term.into_core_(ctx)?), ty.into_core()?)
            }
        })
    }
}

impl Ty {
    pub fn into_core(self) -> Option<core::Ty> {
        self.into_core_(&mut VecDeque::new())
    }

    pub fn into_core_(self, ctx: &mut VecDeque<String>) -> Option<core::Ty> {
        Some(match self {
            Ty::Bool => core::Ty::Bool,
            Ty::Unit => core::Ty::Unit,
            Ty::Arrow(inp, out) => core::Ty::Arrow(
                Box::new(inp.into_core_(ctx)?),
                Box::new(out.into_core_(ctx)?),
            ),
            Ty::Forall(name, body) => {
                ctx.push_front(name);
                let new_body = body.into_core_(ctx);
                ctx.pop_front();
                core::Ty::Forall(Box::new(new_body?))
            }
            Ty::Var(name) => {
                let idx =
                    ctx.iter()
                        .enumerate()
                        .find_map(|(idx, s)| if s == &name { Some(idx) } else { None })?;
                core::Ty::Var(idx)
            }
        })
    }
}
