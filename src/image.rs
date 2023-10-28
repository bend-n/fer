use std::marker::PhantomData;
use std::num::NonZeroU32;

use crate::pixels::PixelExt;
use crate::{error, ImageView, ImageViewMut};

#[derive(Debug)]
enum BufferContainer<'a> {
    MutU8(&'a mut [u8]),
    VecU8(Vec<u8>),
}

impl<'a> BufferContainer<'a> {
    fn as_vec(&self) -> Vec<u8> {
        match self {
            Self::MutU8(slice) => slice.to_vec(),
            Self::VecU8(vec) => vec.clone(),
        }
    }
}

/// Simple container of image data.
#[derive(Debug)]
pub struct Image<'a, P: PixelExt> {
    width: NonZeroU32,
    height: NonZeroU32,
    buffer: BufferContainer<'a>,
    pixel_type: PhantomData<P>,
}

impl<'a, P: PixelExt> Image<'a, P> {
    /// Create empty image with given dimensions and pixel type.
    pub fn new(width: NonZeroU32, height: NonZeroU32) -> Self {
        let pixels_count = (width.get() * height.get()) as usize;
        let buffer = BufferContainer::VecU8(vec![0; pixels_count * P::size()]);
        Self {
            width,
            height,
            buffer,
            pixel_type: PhantomData,
        }
    }

    pub unsafe fn from_vec_u8(width: NonZeroU32, height: NonZeroU32, buffer: Vec<u8>) -> Self {
        let size = (width.get() * height.get()) as usize * P::size();
        if buffer.len() < size {
            error!();
        }
        Self {
            width,
            height,
            buffer: BufferContainer::VecU8(buffer),
            pixel_type: PhantomData,
        }
    }

    pub unsafe fn from_slice_u8(
        width: NonZeroU32,
        height: NonZeroU32,
        buffer: &'a mut [u8],
    ) -> Self {
        let size = (width.get() * height.get()) as usize * P::size();
        if buffer.len() < size {
            error!();
        }
        Self {
            width,
            height,
            buffer: BufferContainer::MutU8(buffer),
            pixel_type: PhantomData,
        }
    }

    /// Creates a copy of the image.
    pub fn copy(&self) -> Image<'static, P> {
        Image {
            width: self.width,
            height: self.height,
            buffer: BufferContainer::VecU8(self.buffer.as_vec()),
            pixel_type: self.pixel_type,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    /// Buffer with image pixels.
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        match &self.buffer {
            BufferContainer::MutU8(p) => p,
            BufferContainer::VecU8(v) => v,
        }
    }

    /// Mutable buffer with image pixels.
    #[inline(always)]
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        match &mut self.buffer {
            BufferContainer::MutU8(p) => p,
            BufferContainer::VecU8(ref mut v) => v.as_mut_slice(),
        }
    }

    #[inline(always)]
    pub fn into_vec(self) -> Vec<u8> {
        match self.buffer {
            BufferContainer::MutU8(p) => p.into(),
            BufferContainer::VecU8(v) => v,
        }
    }

    #[inline(always)]
    pub unsafe fn view(&self) -> ImageView<P> {
        ImageView::new(self.width, self.height, self.buffer())
    }

    #[inline(always)]
    pub unsafe fn view_mut(&mut self) -> ImageViewMut<P> {
        ImageViewMut::new(self.width, self.height, self.buffer_mut())
    }
}

/// Generic image container for internal purposes.
pub(crate) struct InnerImage<'a, P>
where
    P: PixelExt,
{
    width: NonZeroU32,
    height: NonZeroU32,
    pixels: &'a mut [P],
}

impl<'a, P> InnerImage<'a, P>
where
    P: PixelExt,
{
    pub fn new(width: NonZeroU32, height: NonZeroU32, pixels: &'a mut [P]) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    #[inline(always)]
    pub unsafe fn src_view(&self) -> ImageView<P> {
        ImageView::from_pixels(self.width, self.height, self.pixels)
    }

    #[inline(always)]
    pub unsafe fn dst_view(&mut self) -> ImageViewMut<P> {
        ImageViewMut::from_pixels(self.width, self.height, self.pixels)
    }
}
