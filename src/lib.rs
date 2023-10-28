#![doc = include_str!("../README.md")]

pub use convolution::{FilterType, Convolution};
pub use image_view::{change_type_of_pixel_components, CropBox, ImageView, ImageViewMut};
pub use mul_div::MulDiv;
pub use pixels::*;
pub use resizer::{CpuExtensions, ResizeAlg, Resizer};
pub use alpha::AlphaMulDiv;

pub use crate::image::Image;

macro_rules! error {
    () => {
        unsafe { std::hint::unreachable_unchecked() }
    };
}
use error;

#[macro_use]
mod utils;

mod alpha;
mod convolution;
mod image;
mod image_view;
mod mul_div;
#[cfg(target_arch = "aarch64")]
mod neon_utils;
pub mod pixels;
mod resizer;
#[cfg(target_arch = "x86_64")]
mod simd_utils;
#[cfg(target_arch = "wasm32")]
mod wasm32_utils;
