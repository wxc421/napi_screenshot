use std::{fs, thread};
use std::time::Instant;

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
pub fn screen_shots() -> () {
    let start = Instant::now();
    let displays = display_info::DisplayInfo::all().unwrap();
    let count = displays.len();
    let mut handles = vec![];
    for display in displays {
        let handle = thread::spawn(move || {
            let screen = crate::core::Screen::new(crate::core::DisplayInfo {
                id: display.id,
                x: display.x,
                y: display.y,
                width: display.width,
                height: display.height,
                rotation: display.rotation as f32,
                scale_factor: display.scale_factor as f32,
                is_primary: display.is_primary,
            });
            let image = screen.capture().unwrap();
            let buffer = image.to_png().unwrap();
            fs::write(format!("target/{}.png", screen.display_info.id), buffer).unwrap();
            println!("thread id: {:?}，screenshot finish,cost: {:?}", thread::current().id(), start.elapsed());
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap(); // 等待线程结束
    }
    println!("screenshot cost: {:?},display num: {:?}", start.elapsed(), count);
}

#[napi]
pub fn get_screen_shot_by_display_info(screen_shot: ScreenShot) {
    let display_info = screen_shot.display_info;
    let screen = crate::core::Screen::new(crate::core::DisplayInfo {
        id: display_info.id,
        x: display_info.x,
        y: display_info.y,
        width: display_info.width,
        height: display_info.height,
        rotation: display_info.rotation as f32,
        scale_factor: display_info.scale_factor as f32,
        is_primary: display_info.is_primary,
    });
    let image = screen.capture().unwrap();
    let buffer = image.buffer();
    fs::write(format!("target/{}.png", screen.display_info.id), buffer).unwrap();
}

#[napi]
fn get_nums() -> Vec<u32> {
    vec![1, 1, 2, 3, 5, 8]
}

#[test]
fn test_screen_shots() {
    screen_shots()
}