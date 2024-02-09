use wasmi::*;

fn main() {
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))
            (func (export "hello")
                (call $host_hello (i32.const 3))
            )
        )
    "#;
    let wasm = wat::parse_str(&wat).unwrap();
    let module = Module::new(&engine, &mut &wasm[..]).unwrap();

    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    let mut linker = <Linker<HostState>>::new(&engine);
    linker.define("host", "hello", host_hello).unwrap();

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let hello = instance.get_typed_func::<(), ()>(&store, "hello").unwrap();

    hello.call(&mut store, ()).unwrap();
}
