#![allow(unused)]

mod ast;
mod ctx;
mod grammar;
mod pp;
mod typeck;

use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use ast::parse::{Term, Ty};
use ctx::TyCtxt;

#[macro_export]
macro_rules! trace {
    ($($tt:tt)*) => {
        #[cfg(feature = "trace")]
        {
            ::tracing::trace!($($tt)*)
        }
    };
}

// fn builtins() -> HashMap<String, Option<Ty>> {
//     let mut parser = grammar::TypeParser::new();
//     [
//         ("id", "'x => 'x -> 'x"),
//         ("const", "'x => 'x -> 'x -> 'x"),
//         ("cond", "'x => bool -> 'x -> 'x -> 'x"),
//     ]
//     .iter()
//     .map(|&(n, i)| (n.to_string(), Some(parser.parse(i).unwrap())))
//     .collect()
// }

fn main() {
    #[cfg(feature = "trace")]
    {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            // .compact()
            .pretty()
            .init();
    }
    /*let stdin = std::io::stdin();
    let mut parser = grammar::TermParser::new();
    let arena = pretty::Arena::new();
    // let builtins = builtins();
    // let mut ctx = HashMap::new();
    loop {
        // ctx.extend(builtins.clone().into_iter());
        let mut contents = String::new();
        stdin.lock().read_line(&mut contents).expect("io");

        let parsed = parser.parse(&contents).unwrap();
        let pp = pp::pp_parse_term(*parsed.clone(), &arena).into_doc();
        println!("=== Parsed ===\n\n{}\n", pp.pretty(80));
        let core = parsed.into_core().unwrap();
        let pp = pp::pp_core_term(core.clone(), &arena).into_doc();
        println!("=== Resolved ===\n\n{}\n", pp.pretty(80));
        // println!("parsed: {:?}", parsed);
        // println!("inferred type: {:?}", infer(&mut ctx, &parsed));
        // println!("new context: {:?}", ctx);

        // ctx.clear();
    }*/

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut parser = grammar::TermParser::new();
    let arena = pretty::Arena::new();
    let mut ctx = TyCtxt::default();

    loop {
        let mut contents = String::new();
        print!(">>> ");
        stdout.lock().flush().expect("io");
        stdin.lock().read_line(&mut contents).expect("io");

        let parsed = parser.parse(&contents).unwrap();
        let pp = pp::pp_parse_term(*parsed.clone(), &arena).into_doc();
        println!("=== Parsed ===\n\n{}\n", pp.pretty(80));

        let core = parsed.into_core().unwrap();
        let pp = pp::pp_core_term(core.clone(), &arena).into_doc();
        println!("=== Resolved ===\n\n{}\n", pp.pretty(80));

        let inferred = typeck::infer(&mut ctx, &core);
        match inferred {
            None => println!(
                "=== Inferred ===\n\nUninferrable.\n\n=== Context ===\n\n{:?}",
                ctx
            ),
            Some(inf) => {
                let inf = inf.subst_ctx(&ctx);
                let pp = pp::pp_core_ty(inf, &arena).into_doc();
                println!(
                    "=== Inferred ===\n\n{}\n\n=== Context ===\n\n{:?}",
                    pp.pretty(80),
                    ctx
                );
            }
        }
        ctx.clear();
    }
}
