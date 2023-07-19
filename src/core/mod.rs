#[cfg(target_os = "macos")]
mod darwin;

#[cfg(target_os = "macos")]
use darwin::*;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "windows")]
use win32::*;

#[cfg(target_os = "linux")]
mod linux;


#[cfg(target_os = "linux")]
use linux::*;

mod image;

#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub rotation: f32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub display_info: DisplayInfo,
}

impl Screen {
    pub fn new(display_info: &DisplayInfo) -> Self {
        Screen {
            display_info: *display_info,
        }
    }

    pub fn capture(&self) -> Result<Image> {
        capture_screen(&self.display_info)
    }
}