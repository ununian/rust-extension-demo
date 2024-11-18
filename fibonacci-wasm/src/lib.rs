#[allow(warnings)]
mod bindings;

use bindings::exports::share::walker::ast_walker::{AstType, Guest};

struct Component;

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

impl Guest for Component {
    fn walk(mut ast: AstType) -> AstType {
        ast.content = fibonacci_iterative(ast.end);
        return ast;
    }
}

bindings::export!(Component with_types_in bindings);
