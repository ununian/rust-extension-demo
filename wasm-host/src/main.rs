use clap::Parser;
use run::{exports::share::walker::ast_walker::AstType, WasmInstance};
use std::env;

mod run;

#[derive(Parser)]
#[clap(name = "wasm-host", version = env!("CARGO_PKG_VERSION"))]
struct HostApp {
    end: u64,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    HostApp::parse().run().await
}

impl HostApp {
    async fn run(self) -> anyhow::Result<()> {
        // let mut moon_trigonometric_instance =
        //     WasmInstance::create("./moonbit-wasm/target/wasm/release/build/gen/gen.wasm").await?;
        let mut trigonometric_instance =
            WasmInstance::create("./target/wasm32-wasip1/release/trigonometric_wasm.wasm").await?;
        let mut cubic_root_instance =
            WasmInstance::create("./target/wasm32-wasip1/release/cubic_root_wasm.wasm").await?;
        // let mut fibnacci_instance =
        //     WasmInstance::create("./target/wasm32-wasip1/release/fibonacci_wasm.wasm").await?;
        // let mut sum_instance =
        //     WasmInstance::create("./target/wasm32-wasip1/release/sum_wasm.wasm").await?;
        let mut cubic_root_instance =
            WasmInstance::create("./target/wasm32-wasip1/release/cubic_root_wasm.wasm").await?;

        for end in 1..=self.end {
            let ast = AstType {
                start: 1,
                end,
                content: 0,
            };
            let ast = trigonometric_instance.run(ast).await?;
            let ast = cubic_root_instance.run(ast).await?;
            println!("end = {} ;root result = {}", end, ast.content);
        }

        Ok(())
    }
}
