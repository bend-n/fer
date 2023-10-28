//! Contains types of pixels.
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::size_of;
use std::slice;
pub trait GetCount {
    fn count() -> usize;
}

/// Generic type to represent the number of components in single pixel.
pub struct Count<const N: usize>;

impl<const N: usize> GetCount for Count<N> {
    #[inline(always)]
    fn count() -> usize {
        N
    }
}

pub trait GetCountOfValues {
    fn count_of_values() -> usize;
}

/// Generic type to represent the number of available values for a single pixel component.
pub struct Values<const N: usize>;

impl<const N: usize> GetCountOfValues for Values<N> {
    fn count_of_values() -> usize {
        N
    }
}

/// Information about one component of pixel.
pub trait PixelComponent
where
    Self: Sized + Copy + Debug + PartialEq + 'static,
{
    /// Type that provides information about a count of
    /// available values of one pixel's component
    type CountOfComponentValues: GetCountOfValues;

    /// Count of available values of one pixel's component
    fn count_of_values() -> usize {
        Self::CountOfComponentValues::count_of_values()
    }
}

impl PixelComponent for u8 {
    type CountOfComponentValues = Values<256>;
}
impl PixelComponent for u16 {
    type CountOfComponentValues = Values<65536>;
}
impl PixelComponent for i32 {
    type CountOfComponentValues = Values<0>;
}
impl PixelComponent for f32 {
    type CountOfComponentValues = Values<0>;
}

/// Additional information about pixel type.
pub trait PixelExt
where
    Self: Copy + Clone + Sized + Debug + PartialEq,
{
    /// Type of pixel components
    type Component: PixelComponent;
    /// Type that provides information about a count of pixel's components
    type CountOfComponents: GetCount;

    /// Count of pixel's components
    fn count_of_components() -> usize {
        Self::CountOfComponents::count()
    }

    /// Count of available values of one pixel's component
    fn count_of_component_values() -> usize {
        Self::Component::count_of_values()
    }

    /// Size of pixel in bytes
    ///
    /// Example:
    /// ```
    /// # use fer::pixels::{U8x2, U8x3, U8, PixelExt};
    /// assert_eq!(U8x3::size(), 3);
    /// assert_eq!(U8x2::size(), 2);
    /// assert_eq!(U8::size(), 1);
    /// ```
    fn size() -> usize {
        size_of::<Self>()
    }

    /// Create slice of pixel's components from slice of pixels
    fn components(buf: &[Self]) -> &[Self::Component] {
        let size = buf.len() * Self::count_of_components();
        let components_ptr = buf.as_ptr() as *const Self::Component;
        unsafe { slice::from_raw_parts(components_ptr, size) }
    }

    /// Create mutable slice of pixel's components from mutable slice of pixels
    fn components_mut(buf: &mut [Self]) -> &mut [Self::Component] {
        let size = buf.len() * Self::count_of_components();
        let components_ptr = buf.as_mut_ptr() as *mut Self::Component;
        unsafe { slice::from_raw_parts_mut(components_ptr, size) }
    }
}

/// Generic type of pixel.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Pixel<T, C, const COUNT_OF_COMPONENTS: usize>(
    pub T,
    PhantomData<[C; COUNT_OF_COMPONENTS]>,
)
where
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent;

impl<T, C, const COUNT_OF_COMPONENTS: usize> Pixel<T, C, COUNT_OF_COMPONENTS>
where
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent,
{
    #[inline(always)]
    pub const fn new(v: T) -> Self {
        Self(v, PhantomData)
    }
}

impl<T, C, const COUNT_OF_COMPONENTS: usize> PixelExt for Pixel<T, C, COUNT_OF_COMPONENTS>
where
    Self: Debug,
    T: Sized + Copy + Clone + PartialEq + 'static,
    C: PixelComponent,
{
    type Component = C;
    type CountOfComponents = Count<COUNT_OF_COMPONENTS>;
}

macro_rules! pixel_struct {
    ($name:ident, $type:tt, $comp_type:tt, $comp_count:literal, $doc:expr) => {
        #[doc = $doc]
        pub type $name = Pixel<$type, $comp_type, $comp_count>;

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let components_ptr = self as *const _ as *const $comp_type;
                let components: &[$comp_type] =
                    unsafe { slice::from_raw_parts(components_ptr, $comp_count) };
                write!(f, "{}{:?}", stringify!($name), components)
            }
        }
    };
}

pixel_struct!(U8, u8, u8, 1, "One byte per pixel (e.g. L8)");
pixel_struct!(U8x2, u16, u8, 2, "Two bytes per pixel (e.g. LA8)");
pixel_struct!(U8x3, [u8; 3], u8, 3, "Three bytes per pixel (e.g. RGB8)");
pixel_struct!(
    U8x4,
    u32,
    u8,
    4,
    "Four bytes per pixel (RGBA8, RGBx8, CMYK8 and other)"
);
pixel_struct!(U16, u16, u16, 1, "One `u16` component per pixel (e.g. L16)");
pixel_struct!(
    U16x2,
    [u16; 2],
    u16,
    2,
    "Two `u16` components per pixel (e.g. LA16)"
);
pixel_struct!(
    U16x3,
    [u16; 3],
    u16,
    3,
    "Three `u16` components per pixel (e.g. RGB16)"
);
pixel_struct!(
    U16x4,
    [u16; 4],
    u16,
    4,
    "Four `u16` components per pixel (e.g. RGBA16)"
);
pixel_struct!(I32, i32, i32, 1, "One `i32` component per pixel");
pixel_struct!(F32, f32, f32, 1, "One `f32` component per pixel");

pub trait IntoPixelComponent<Out: PixelComponent>
where
    Self: PixelComponent,
{
    fn into_component(self) -> Out;
}

impl<C: PixelComponent> IntoPixelComponent<C> for C {
    fn into_component(self) -> C {
        self
    }
}

impl IntoPixelComponent<u8> for u16 {
    fn into_component(self) -> u8 {
        self.to_le_bytes()[1]
    }
}

impl IntoPixelComponent<u16> for u8 {
    fn into_component(self) -> u16 {
        u16::from_le_bytes([self, self])
    }
}
