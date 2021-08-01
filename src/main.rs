#![allow(unused)]

mod ast;
mod grammar;
mod typeck;

use std::{collections::HashMap, io::prelude::*};

use ast::{Term, Ty};
use typeck::infer;

#[macro_export]
macro_rules! trace {
    ($($tt:tt)*) => {
        #[cfg(feature = "trace")]
        {
            ::tracing::trace!($($tt)*)
        }
    };
}

fn builtins() -> HashMap<String, Option<Ty>> {
    let mut parser = grammar::TypeParser::new();
    [
        ("id", "'x => 'x -> 'x"),
        ("const", "'x => 'x -> 'x -> 'x"),
        ("cond", "'x => bool -> 'x -> 'x -> 'x"),
    ]
    .iter()
    .map(|&(n, i)| (n.to_string(), Some(parser.parse(i).unwrap())))
    .collect()
}

fn main() {
    #[cfg(feature = "trace")]
    {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .compact()
            .init();
    }
    let stdin = std::io::stdin();
    let mut parser = grammar::TermParser::new();
    let builtins = builtins();
    let mut ctx = HashMap::new();
    loop {
        ctx.extend(builtins.clone().into_iter());
        // let contents = String::from(r#"(\x.\y.x : 'x => 'y => 'x -> 'y -> 'x)$true"#);
        let mut contents = String::new();
        stdin.lock().read_line(&mut contents).expect("io");

        let parsed = parser.parse(&contents).unwrap();
        println!("parsed: {:?}", parsed);
        println!("inferred type: {:?}", infer(&mut ctx, &parsed));
        println!("new context: {:?}", ctx);

        ctx.clear();
    }
}
