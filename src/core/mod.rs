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
use crate::core::image::Image;

mod image;

use anyhow::{anyhow, Ok, Result};
use display_info::DisplayInfo;


#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub display_info: DisplayInfo,
}

impl Screen {
    pub fn new(display_info: DisplayInfo) -> Self {
        Screen {
            display_info,
        }
    }

    pub fn capture(&self) -> Result<Image> {
        capture_screen(&self.display_info)
    }
}