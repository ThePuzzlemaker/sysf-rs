#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(String),
    App(Box<Term>, Box<Term>),
    PolyApp(Box<Term>, Ty),
    Abs(String, Box<Term>),
    PolyAbs(String, Box<Term>),
    Bool(bool),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Ann(Box<Term>, Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Var(String),
    PolyLam(String, Box<Ty>),
    Lam(Box<Ty>, Box<Ty>),
}
