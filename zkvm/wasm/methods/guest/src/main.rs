#![no_main]
#![allow(unused_imports)]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};

pub fn main() {
    let engine = Engine::default();

    let wasm: Vec<u8> = env::read();
    let pub_values: Vec<u64> = env::read();

    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = Vec<u64>;

    let mut store = Store::new(&engine, pub_values);
    let require = Func::wrap(&mut store, |cond: u32| {
        if cond == 0 {
            panic!("require failed");
        }
    });
    let wasm_input = Func::wrap(&mut store, |mut caller: Caller<'_, HostState>, is_public: u32| -> u64 {
        if is_public != 1 {
            panic!("Only public value is supported");
        }

        let host_state = caller.data_mut();
        if let Some(value) = host_state.pop() {
            value
        } else {
            panic!("No more values availabe in pub_values");
        }
    });
    let mut linker = <Linker<HostState>>::new(&engine);
    linker.define("env", "require", require).expect("Failed to define require function");
    linker.define("env", "wasm_input", wasm_input).expect("Failed to define wasm_input function");

    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let zkmain = instance
        .get_typed_func::<(), ()>(&store, "zkmain")
        .expect("Failed to get typed_func");
    zkmain.call(&mut store, ()).expect("Failed to call");
}
