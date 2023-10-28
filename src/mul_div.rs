use crate::alpha::AlphaMulDiv;
use crate::{error, CpuExtensions};
use crate::{ImageView, ImageViewMut};

/// Methods of this structure used to multiply or divide color-channels (RGB or Luma)
/// by alpha-channel. Supported pixel types: U8x2, U8x4, U16x2 and U16x4.
///
/// By default, instance of `MulDiv` created with best CPU-extensions provided by your CPU.
/// You can change this by use method [MulDiv::set_cpu_extensions].
///
/// # Examples
///
/// ```
/// use std::num::NonZeroU32;
/// use fer::{Image, MulDiv, U8x4};
/// unsafe {
/// let width = NonZeroU32::new(10).unwrap();
/// let height = NonZeroU32::new(7).unwrap();
/// let src_image = Image::<U8x4>::new(width, height);
/// let mut dst_image = Image::<U8x4>::new(width, height);
///
/// let mul_div = MulDiv::default();
/// mul_div.multiply_alpha(&src_image.view(), &mut dst_image.view_mut());
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct MulDiv {
    cpu_extensions: CpuExtensions,
}

impl MulDiv {
    #[inline(always)]
    pub fn cpu_extensions(&self) -> CpuExtensions {
        self.cpu_extensions
    }

    /// # Safety
    /// This is unsafe because this method allows you to set a CPU-extensions
    /// that are not actually supported by your CPU.
    pub unsafe fn set_cpu_extensions(&mut self, extensions: CpuExtensions) {
        self.cpu_extensions = extensions;
    }

    /// Multiplies color-channels (RGB or Luma) of source image by alpha-channel and store
    /// result into destination image.
    pub unsafe fn multiply_alpha<P: AlphaMulDiv>(
        &self,
        src_image: &ImageView<'_, P>,
        dst_image: &mut ImageViewMut<'_, P>,
    ) {
        let cpu_extensions = self.cpu_extensions;
        if src_image.width() != dst_image.width() || src_image.height() != dst_image.height() {
            error!();
        }
        P::multiply_alpha(src_image, dst_image, cpu_extensions);
    }

    /// Multiplies color-channels (RGB or Luma) of image by alpha-channel inplace.
    pub fn multiply_alpha_inplace<P: AlphaMulDiv>(&self, image: &mut ImageViewMut<'_, P>) {
        let cpu_extensions = self.cpu_extensions;
        P::multiply_alpha_inplace(image, cpu_extensions);
    }

    /// Divides color-channels (RGB or Luma) of source image by alpha-channel and store
    /// result into destination image.
    pub unsafe fn divide_alpha<P: AlphaMulDiv>(
        &self,
        src_image: &ImageView<'_, P>,
        dst_image: &mut ImageViewMut<'_, P>,
    ) {
        let cpu_extensions = self.cpu_extensions;
        if src_image.width() != dst_image.width() || src_image.height() != dst_image.height() {
            error!();
        }
        P::divide_alpha(src_image, dst_image, cpu_extensions);
    }

    /// Divides color-channels (RGB or Luma) of image by alpha-channel inplace.
    pub fn divide_alpha_inplace<P: AlphaMulDiv>(&self, image: &mut ImageViewMut<'_, P>) {
        let cpu_extensions = self.cpu_extensions;
        P::divide_alpha_inplace(image, cpu_extensions);
    }
}
