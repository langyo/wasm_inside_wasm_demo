# Running WASM inside WASM

这是一个简单的例程，展示如何在 WebAssembly 模块中嵌套运行 WebAssembly 模块。

This is a simple example of running a WebAssembly module inside another WebAssembly module.

## How to run

> **注意**：请先安装 `cargo-make`。
>
> **Note**: Please install `cargo-make` first.

```bash
cargo make dev
```

## Why

这种措施能够进一步提升 WebAssembly 黑箱的破解门槛，通过嵌套虚拟机与适当的数据加密措施，产物通常需要花费更多的时间提取实际数据。

This measure can further improve the cracking threshold of WebAssembly black box. By nesting virtual machines and appropriate data encryption measures, the product usually requires more time to extract actual data.

需要注意的是，`wasmi-wasi` 库无法编译至 WASI，这意味着被嵌套的 WebAssembly 模块暂时无法直接调用 WASI 相关能力，包括且不限于时钟接口、标准输入输出接口等。

It should be noted that the `wasmi-wasi` library cannot be compiled to WASI, which means that the nested WebAssembly module cannot directly call WASI-related capabilities, including but not limited to clock interfaces, standard I/O interfaces, etc.
