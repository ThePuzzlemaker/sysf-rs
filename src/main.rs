#![allow(unused)]

mod ast;
mod ctx;
mod grammar;
mod pp;
mod typeck;

use std::{collections::HashMap, io::prelude::*};

use ast::parse::{Term, Ty};
use ctx::TyCtxt;
use typeck::subtyping;
// use typeck::infer;

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

    let mut stdin = std::io::stdin();
    let mut parser = grammar::TypeParser::new();
    let arena = pretty::Arena::new();
    let mut ctx = TyCtxt::default();

    loop {
        let mut contents = String::new();
        stdin.lock().read_line(&mut contents).expect("io");

        let parsed = parser.parse(&contents).unwrap();
        let pp = pp::pp_parse_ty(parsed.clone(), &arena).into_doc();
        println!("=== Parsed ===\n\n{}\n", pp.pretty(80));
        let core1 = parsed.into_core().unwrap();
        let pp = pp::pp_core_ty(core1.clone(), &arena).into_doc();
        println!("=== Resolved ===\n\n{}\n", pp.pretty(80));

        // let mut contents = String::new();
        // stdin.lock().read_line(&mut contents).expect("io");

        // let parsed = parser.parse(&contents).unwrap();
        // let pp = pp::pp_parse_ty(parsed.clone(), &arena).into_doc();
        // println!("=== Parsed ===\n\n{}\n", pp.pretty(80));
        // let core2 = parsed.into_core().unwrap();
        // let pp = pp::pp_core_ty(core2.clone(), &arena).into_doc();
        // println!("=== Resolved ===\n\n{}\n", pp.pretty(80));

        // let new = core1.subst_outer_uvar(&core2);
        // let evarl = ctx.fresh_evar();
        let evarr = ctx.fresh_evar();
        // ctx.add_unsolved(evarl);
        ctx.add_unsolved(evarr);
        // let new1 = subtyping::inst_left(&mut ctx, evarl, &core1).is_some();
        let new2 = subtyping::inst_right(&mut ctx, &core1, evarr).is_some();
        // println!("InstL: {}", new1);
        // println!("\nNew Context: {:?}", ctx);
        println!("\nInstR: {}", new2);
        // println!("=== Subst === \n\n{}\n", pp.pretty(80));
        println!("\nNew Context: {:?}", ctx);
        ctx.clear();
    }
}
