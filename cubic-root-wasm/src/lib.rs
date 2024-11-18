#[allow(warnings)]
mod bindings;

use bindings::exports::share::walker::ast_walker::{AstType, Guest};

struct Component;

fn integer_cube_root(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut low = 0;
    let mut high = n;

    while low <= high {
        let mid = low + (high - low) / 2;
        let cube = mid.saturating_mul(mid).saturating_mul(mid);

        if cube == n {
            return mid;
        } else if cube < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    high
}

impl Guest for Component {
    fn walk(mut ast: AstType) -> AstType {
        ast.content = integer_cube_root(ast.content);
        return ast;
    }
}

bindings::export!(Component with_types_in bindings);
