#[allow(warnings)]
mod bindings;

use bindings::exports::share::walker::ast_walker::{AstType, Guest};

struct Component;

impl Guest for Component {
    fn walk(mut ast: AstType) -> AstType {
        ast.content = (ast.start..=ast.end).sum();
        return ast;
    }
}

bindings::export!(Component with_types_in bindings);
