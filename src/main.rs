use std::collections::HashMap;

pub mod ast;
pub mod grammar;
pub mod typeck;

use typeck::infer;
use grammar::TermParser;

fn main() {
    let mut ctx = HashMap::new();
    let parser = TermParser::new();
    let inputs = vec![
        "|x| |y| x.y :: (Bool -> Bool) -> Bool -> Bool",
        "(|x| |y| x :: Bool -> Bool -> Bool).true.false",
        "<'x> |x| x :: 'x => 'x -> 'x",
        "(<'x> |x| x :: 'x => 'x -> 'x)[Bool].true",
        "(<'x> |x| x :: 'x => 'x -> 'x).true",
        "(<'x> <'y> |x| |y| x :: 'x => 'y => 'x -> 'y -> 'x)",
        "(<'x> <'y> |x| |y| x :: 'x => 'y => 'x -> 'y -> 'x).true.false",
    ];
    for (idx, input) in inputs.into_iter().enumerate() {
        println!("== {} ==", idx + 1);
        println!("{}", input);
        let parsed = parser.parse(input).unwrap();
        println!("parsed: {:?}", parsed);
        println!("inferred type: {:?}", infer(&mut ctx, &parsed));
        println!("new context: {:?}", ctx);
        ctx.clear();
    }
}
