#![deny(missing_docs)]
#![deny(warnings)]

//! Convert types into a Vec, avoiding copies when possible.

/// Anything convertible into a Vec with or without copies, but avoiding them if possible.
pub trait IntoVec<T> {
    /// Convert Self into a Vec
    #[inline]
    fn into_vec(self) -> Vec<T>;
}

impl<T> IntoVec<T> for Vec<T> {
    #[inline]
    fn into_vec(self) -> Vec<T> { self }
}

impl<'a, T: Clone> IntoVec<T> for &'a [T] {
    #[inline]
    fn into_vec(self) -> Vec<T> { self.to_vec() }
}

impl<T> IntoVec<T> for Box<[T]> {
    #[inline]
    fn into_vec(self) -> Vec<T> {
        use std::slice::BoxedSliceExt;
        fn boxed_into_vec<T, B: BoxedSliceExt<T>>(b: B) -> Vec<T> { b.into_vec() }

        boxed_into_vec(self)
    }
}

impl IntoVec<u8> for String {
    #[inline]
    fn into_vec(self) -> Vec<u8> { self.into_bytes() }
}

impl<'a> IntoVec<u8> for &'a str {
    #[inline]
    fn into_vec(self) -> Vec<u8> { self.as_bytes().into_vec() }
}

