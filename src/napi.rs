use std::{fs, thread};
use std::time::Instant;
use crate::core::Screen;

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
pub fn get_display_infos() -> Vec<DisplayInfo> {
    let infos = display_info::DisplayInfo::all().unwrap();
    let infos = infos.into_iter().map(|info| {
        DisplayInfo {
            id: info.id,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            rotation: info.rotation as f64,
            scale_factor: info.scale_factor as f64,
            is_primary: info.is_primary,
        }
    }).collect();
    infos
}

#[napi]
pub fn screen_shots() -> () {
    let start = Instant::now();
    let displays = display_info::DisplayInfo::all().unwrap();
    let count = displays.len();
    let mut handles = vec![];
    for display_info in displays {
        let handle = thread::spawn(move || {
            let id = display_info.id;
            let screen = Screen::new(display_info);
            let image = screen.capture().unwrap();
            // println!("buffer: {:?}", &image.buffer()[..100]);
            let buffer = image.to_png().unwrap();
            fs::write(format!("target/{}.png", screen.display_info.id), buffer).unwrap();
            println!("display_id: {:?} thread id: {:?}，screenshot finish,cost: {:?}", id, thread::current().id(), start.elapsed());
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
    let screen = new_screen(display_info);
    let image = screen.capture().unwrap();
    let buffer = image.buffer();
    fs::write(screen_shot.image_path, buffer).unwrap();
}

#[napi]
pub fn get_screen_shot_byte_by_display_info(screen_shot: ScreenShot) -> Vec<u8> {
    let display_info = screen_shot.display_info;
    let screen = new_screen(display_info);
    let image = screen.capture().unwrap();
    image.into()
}

fn new_screen(info: DisplayInfo) -> Screen {
    let screen = Screen::new(display_info::DisplayInfo {
        id: info.id,
        x: info.x,
        y: info.y,
        width: info.width,
        height: info.height,
        rotation: info.rotation as f32,
        scale_factor: info.scale_factor as f32,
        is_primary: info.is_primary,
    });
    screen
}

#[napi]
fn get_nums() -> Vec<u32> {
    vec![1, 1, 2, 3, 5, 8]
}

#[test]
fn test_screen_shots() {
    screen_shots()
}