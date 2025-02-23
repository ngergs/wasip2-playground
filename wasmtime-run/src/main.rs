use anyhow::Result;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiImpl, WasiView};

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
            ctx: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        },
    );
    let linker = Linker::new(&engine);
    // not needed for our example but shows how to explicitly add certain capabilities to the linker
    // see implementation of wasmtime_wasi::add_to_linker_sync
    // let closure = type_annotate::<MyState, _>(|t| WasiImpl(IoImpl(t)));
    // wasmtime_wasi::bindings::sync::cli::environment::add_to_linker_get_host(&mut linker, closure)?;
    // alternative, contains potentially dangerous things like creating sockets
    // wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let instance = Math::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_div(&mut store, Arguments { x: 2, y: 3 })?;
    println!("{result:?}");
    let result = instance.call_div(&mut store, Arguments { x: 2, y: 0 })?;
    println!("{result:?}");
    Ok(())
}

#[allow(dead_code)] // for some of the commented out examples above
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
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
