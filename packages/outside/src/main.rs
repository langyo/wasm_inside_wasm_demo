use wasi_cap_std_sync::WasiCtxBuilder;
use wasmi::{Config, Engine, Extern, Linker, Module, Store};
use wasmi_wasi::{add_to_linker, WasiCtx};

fn main() {
    let wasm = include_bytes!("../../inside/target/wasm32-wasi/debug/inside.wasm");
    let config = Config::default();
    let engine = Engine::new(&config);
    let module = Module::new(&engine, &wasm[..]).unwrap();
    let mut linker = <Linker<WasiCtx>>::new(&engine);

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    add_to_linker(&mut linker, |ctx| ctx).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let f = instance
        .get_export(&store, "_start")
        .and_then(Extern::into_func)
        .unwrap();
    let mut result = [];
    f.call(&mut store, &[], &mut result).unwrap();
}
