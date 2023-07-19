/*
#![deny(clippy::all)] 是 Rust 编译器的一个指令，它启用了 Clippy Lint 工具并设置其报告所有的 Lint 警告。
Clippy 是一个 Lint 工具集，用于检查 Rust 代码中的常见错误和潜在问题，可以帮助提高 Rust 代码的质量和性能。

通过使用 #![deny(clippy::all)]，如果任何 Clippy Lint 报告了警告，则 Rust 编译器将无法构建代码。
这是一个良好的实践，以确保您的代码没有常见问题并遵循最佳实践。
但是，它也可能过于严格并产生误报警告，因此重要的是了解警告并根据需要选择性地禁用它们。
 */
#![deny(clippy::all)]

pub mod core;
pub mod napi;

#[macro_use]
extern crate napi_derive;

// cbindgen --config cbindgen.toml --crate napi_screenshot --output napi_screenshot.h