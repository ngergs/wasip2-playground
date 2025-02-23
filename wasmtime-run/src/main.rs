use anyhow::Result;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path:"../math.wit",
    world: "math",
});

fn main() -> Result<()> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    let component = Component::from_file(
        &engine,
        "../wasip2-example/target/wasm32-wasip2/release/wasip2_example.wasm",
    )?;
    let mut store = Store::new(
        &engine,
        MyState {
            // what is allowed in terms of the runtime environment is configured here, see https://docs.wasmtime.dev/api/wasmtime_wasi/struct.WasiCtxBuilder.html
            ctx: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        },
    );
    let mut linker = Linker::new(&engine);
    // if you run this with the `MINIMAL_WASM` Taskfile option the following line can be omitted
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let instance = Math::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_div(&mut store, Arguments { x: 2, y: 3 })?;
    println!("{result:?}");
    let result = instance.call_div(&mut store, Arguments { x: 2, y: 0 })?;
    println!("{result:?}");
    Ok(())
}

// see https://docs.wasmtime.dev/api/wasmtime_wasi/fn.add_to_linker_sync.html
struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
