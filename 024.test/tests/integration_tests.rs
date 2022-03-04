// 集成测试
use adder;
mod common; // 需要在common中定义setup函数 文件名称要是mod.rs

#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}