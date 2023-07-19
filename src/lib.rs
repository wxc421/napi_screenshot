/*
#![deny(clippy::all)] 是 Rust 编译器的一个指令，它启用了 Clippy Lint 工具并设置其报告所有的 Lint 警告。
Clippy 是一个 Lint 工具集，用于检查 Rust 代码中的常见错误和潜在问题，可以帮助提高 Rust 代码的质量和性能。

通过使用 #![deny(clippy::all)]，如果任何 Clippy Lint 报告了警告，则 Rust 编译器将无法构建代码。
这是一个良好的实践，以确保您的代码没有常见问题并遵循最佳实践。
但是，它也可能过于严格并产生误报警告，因此重要的是了解警告并根据需要选择性地禁用它们。
 */
#![deny(clippy::all)]

mod core;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ScreenShot {
    pub display_info: DisplayInfo,
    pub image_path: String,
}

#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub rotation: f64,
    pub scale_factor: f64,
    pub is_primary: bool,
}


#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub fn get_screen_shots() -> Vec<ScreenShot> {
    let displays = display_info::DisplayInfo::all().unwrap();
    let mut screenshots: Vec<ScreenShot> = Vec::with_capacity(displays.len());
    for display in &displays {
        let screen_shot = ScreenShot {
            display_info: DisplayInfo {
                id: display.id,
                x: display.x,
                y: display.y,
                width: display.width,
                height: display.height,
                rotation: display.rotation as f64,
                scale_factor: display.scale_factor as f64,
                is_primary: display.is_primary,
            },
            image_path: String::from("Hello, world!"),
        };
        screenshots.push(screen_shot);
    }
    screenshots
}

#[napi]
fn get_nums() -> Vec<u32> {
    vec![1, 1, 2, 3, 5, 8]
}
// cbindgen --config cbindgen.toml --crate napi_screenshot --output napi_screenshot.h