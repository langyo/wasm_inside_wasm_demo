// TODO - 实际上这段代码没有对接给 outside 模块，outside 那边暂时使用自行编译的 wat 文本进行测试

#[no_mangle]
extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}
