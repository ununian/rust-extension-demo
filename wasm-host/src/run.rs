use anyhow::Context;
use exports::share::walker::ast_walker::AstType;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

wasmtime::component::bindgen!({
    path: "../share/wit/ast-walker.wit",
    world: "ast-handle",
    async: true
});

pub struct WasmInstance {
    instance: AstHandle,
    store: Store<()>,
}

impl WasmInstance {
    pub async fn create(path: &str) -> anyhow::Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let linker = Linker::new(&engine);

        let mut store = Store::new(&engine, ());
        let component = Component::from_file(&engine, path).context("Component file not found")?;

        let (instance, _) = AstHandle::instantiate_async(&mut store, &component, &linker)
            .await
            .context("Failed to instantiate the ast-handle world")?;

        Ok(WasmInstance { instance, store })
    }

    pub async fn run(self: &mut Self, ast: AstType) -> wasmtime::Result<AstType> {
        self.instance
            .share_walker_ast_walker()
            .call_walk(&mut self.store, ast)
            .await
            .context("Failed to call add function")
    }
}
