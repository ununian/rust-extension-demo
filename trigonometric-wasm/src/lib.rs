#[allow(warnings)]
mod bindings;

use bindings::exports::share::walker::ast_walker::{AstType, Guest};

struct Component;

impl Guest for Component {
    fn walk(mut ast: AstType) -> AstType {
        let result: f64 = (ast.start..=ast.end)
            .map(|i| (i as f64).sin().cos().tan())
            .sum();
        ast.content = result.round() as u64;
        return ast;
    }
}

bindings::export!(Component with_types_in bindings);
