// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::UnknownUnit;
use approxeq::ApproxEq;
use length::Length;
#[cfg(feature = "mint")]
use mint;
use point::{Point2D, Point3D, point2, point3};
use size::{Size2D, size2};
use scale::Scale;
use transform2d::Transform2D;
use transform3d::Transform3D;
use trig::Trig;
use Angle;
use num::*;
use num_traits::{Float, NumCast, Signed};
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::marker::PhantomData;
use core::cmp::{Eq, PartialEq};
use core::hash::{Hash};
#[cfg(feature = "serde")]
use serde;

/// A 2d Vector tagged with a unit.
#[repr(C)]
pub struct Vector2D<T, U> {
    pub x: T,
    pub y: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

mint_vec!(Vector2D[x, y] = Vector2);

impl<T: Copy, U> Copy for Vector2D<T, U> {}

impl<T: Clone, U> Clone for Vector2D<T, U> {
    fn clone(&self) -> Self {
        Vector2D {
            x: self.x.clone(),
            y: self.y.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U> serde::Deserialize<'de> for Vector2D<T, U>
    where T: serde::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let (x, y) = try!(serde::Deserialize::deserialize(deserializer));
        Ok(Vector2D { x, y, _unit: PhantomData })
    }
}

#[cfg(feature = "serde")]
impl<T, U> serde::Serialize for Vector2D<T, U>
    where T: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        (&self.x, &self.y).serialize(serializer)
    }
}

impl<T, U> Eq for Vector2D<T, U> where T: Eq {}

impl<T, U> PartialEq for Vector2D<T, U>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T, U> Hash for Vector2D<T, U>
    where T: Hash
{
    fn hash<H: ::core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

impl<T: Copy + Zero, U> Vector2D<T, U> {
    /// Constructor, setting all components to zero.
    #[inline]
    pub fn zero() -> Self {
        Vector2D::new(Zero::zero(), Zero::zero())
    }

    /// Convert into a 3d vector.
    #[inline]
    pub fn to_3d(&self) -> Vector3D<T, U> {
        vec3(self.x, self.y, Zero::zero())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Vector2D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

impl<T: fmt::Display, U> fmt::Display for Vector2D<T, U> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({},{})", self.x, self.y)
    }
}

impl<T: Default, U> Default for Vector2D<T, U> {
    fn default() -> Self {
        Vector2D::new(Default::default(), Default::default())
    }
}

impl<T, U> Vector2D<T, U> {
    /// Constructor taking scalar values directly.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Vector2D {
            x,
            y,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, U> Vector2D<T, U> {
    /// Constructor taking properly  Lengths instead of scalar values.
    #[inline]
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>) -> Self {
        vec2(x.0, y.0)
    }

    /// Create a 3d vector from this one, using the specified z value.
    #[inline]
    pub fn extend(&self, z: T) -> Vector3D<T, U> {
        vec3(self.x, self.y, z)
    }

    /// Cast this vector into a point.
    ///
    /// Equivalent to adding this vector to the origin.
    #[inline]
    pub fn to_point(&self) -> Point2D<T, U> {
        Point2D {
            x: self.x,
            y: self.y,
            _unit: PhantomData,
        }
    }

    /// Swap x and y.
    #[inline]
    pub fn yx(&self) -> Self {
        vec2(self.y, self.x)
    }

    /// Cast this vector into a size.
    #[inline]
    pub fn to_size(&self) -> Size2D<T, U> {
        size2(self.x, self.y)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Vector2D<T, UnknownUnit> {
        vec2(self.x, self.y)
    }

    /// Tag a unit-less value with units.
    #[inline]
    pub fn from_untyped(p: Vector2D<T, UnknownUnit>) -> Self {
        vec2(p.x, p.y)
    }

    /// Cast the unit
    #[inline]
    pub fn cast_unit<V>(&self) -> Vector2D<T, V> {
        vec2(self.x, self.y)
    }

    #[inline]
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T, U> Vector2D<T, U>
where
    T: Copy
        + Clone
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Sub<T, Output = T>
        + Trig
        + PartialOrd
        + One
        + Zero {
    #[inline]
    pub fn to_transform(&self) -> Transform2D<T, U, U> {
        Transform2D::create_translation(self.x, self.y)
    }
}

impl<T, U> Vector2D<T, U>
where
    T: Trig + Copy + Sub<T, Output = T>,
{
    /// Returns the angle between this vector and the x axis between -PI and PI.
    pub fn angle_from_x_axis(&self) -> Angle<T> {
        Angle::radians(Trig::fast_atan2(self.y, self.x))
    }
}

impl<T, U> Vector2D<T, U>
where
    T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T>,
{
    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Returns the norm of the cross product [self.x, self.y, 0] x [other.x, other.y, 0]..
    #[inline]
    pub fn cross(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn normalize(self) -> Self
    where
        T: Float,
    {
        self / self.length()
    }

    /// Return the normalized vector even if the length is larger than the max value of Float.
    #[inline]
    pub fn robust_normalize(self) -> Self
    where
        T: Float,
    {
        let length = self.length();
        if length.is_infinite() {
            let scaled = self / T::max_value();
            scaled / scaled.length()
        } else {
            self / length
        }
    }

    #[inline]
    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.square_length().sqrt()
    }
}

impl<T, U> Vector2D<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this vector and another vector.
    ///
    /// `t` is expected to be between zero and one.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        let one_t = T::one() - t;
        (*self) * one_t + other * t
    }
}

impl<T: Copy + Add<T, Output = T>, U> Add for Vector2D<T, U> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Copy + Add<T, Output = T>, U> AddAssign for Vector2D<T, U> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T: Copy + Sub<T, Output = T>, U> SubAssign<Vector2D<T, U>> for Vector2D<T, U> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T: Copy + Sub<T, Output = T>, U> Sub for Vector2D<T, U> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        vec2(self.x - other.x, self.y - other.y)
    }
}

impl<T: Copy + Neg<Output = T>, U> Neg for Vector2D<T, U> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        vec2(-self.x, -self.y)
    }
}

impl<T: Float, U> Vector2D<T, U> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        vec2(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        vec2(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
    pub fn clamp(&self, start: Self, end: Self) -> Self {
        self.max(start).min(end)
    }
}

impl<T: Copy + Mul<T, Output = T>, U> Mul<T> for Vector2D<T, U> {
    type Output = Self;
    #[inline]
    fn mul(self, scale: T) -> Self {
        vec2(self.x * scale, self.y * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U> Div<T> for Vector2D<T, U> {
    type Output = Self;
    #[inline]
    fn div(self, scale: T) -> Self {
        vec2(self.x / scale, self.y / scale)
    }
}

impl<T: Copy + Mul<T, Output = T>, U> MulAssign<T> for Vector2D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

impl<T: Copy + Div<T, Output = T>, U> DivAssign<T> for Vector2D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

impl<T: Copy + Mul<T, Output = T>, U1, U2> Mul<Scale<T, U1, U2>> for Vector2D<T, U1> {
    type Output = Vector2D<T, U2>;
    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        vec2(self.x * scale.get(), self.y * scale.get())
    }
}

impl<T: Copy + Div<T, Output = T>, U1, U2> Div<Scale<T, U1, U2>> for Vector2D<T, U2> {
    type Output = Vector2D<T, U1>;
    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        vec2(self.x / scale.get(), self.y / scale.get())
    }
}

impl<T: Round, U> Vector2D<T, U> {
    /// Rounds each component to the nearest integer value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    /// For example `{ -0.1, -0.8 }.round() == { 0.0, -1.0 }`.
    #[inline]
    #[must_use]
    pub fn round(&self) -> Self {
        vec2(self.x.round(), self.y.round())
    }
}

impl<T: Ceil, U> Vector2D<T, U> {
    /// Rounds each component to the smallest integer equal or greater than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    /// For example `{ -0.1, -0.8 }.ceil() == { 0.0, 0.0 }`.
    #[inline]
    #[must_use]
    pub fn ceil(&self) -> Self {
        vec2(self.x.ceil(), self.y.ceil())
    }
}

impl<T: Floor, U> Vector2D<T, U> {
    /// Rounds each component to the biggest integer equal or lower than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    /// For example `{ -0.1, -0.8 }.floor() == { -1.0, -1.0 }`.
    #[inline]
    #[must_use]
    pub fn floor(&self) -> Self {
        vec2(self.x.floor(), self.y.floor())
    }
}

impl<T: NumCast + Copy, U> Vector2D<T, U> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast + Copy>(&self) -> Vector2D<NewT, U> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn try_cast<NewT: NumCast + Copy>(&self) -> Option<Vector2D<NewT, U>> {
        match (NumCast::from(self.x), NumCast::from(self.y)) {
            (Some(x), Some(y)) => Some(Vector2D::new(x, y)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(&self) -> Vector2D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(&self) -> Vector2D<f64, U> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(&self) -> Vector2D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(&self) -> Vector2D<u32, U> {
        self.cast()
    }

    /// Cast into an i32 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(&self) -> Vector2D<i32, U> {
        self.cast()
    }

    /// Cast into an i64 vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(&self) -> Vector2D<i64, U> {
        self.cast()
    }
}

impl<T: Copy + ApproxEq<T>, U> ApproxEq<Vector2D<T, U>> for Vector2D<T, U> {
    #[inline]
    fn approx_epsilon() -> Self {
        vec2(T::approx_epsilon(), T::approx_epsilon())
    }

    #[inline]
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
    }
}

impl<T: Copy, U> Into<[T; 2]> for Vector2D<T, U> {
    fn into(self) -> [T; 2] {
        self.to_array()
    }
}

impl<T: Copy, U> From<[T; 2]> for Vector2D<T, U> {
    fn from(array: [T; 2]) -> Self {
        vec2(array[0], array[1])
    }
}

impl<T: Copy, U> Into<(T, T)> for Vector2D<T, U> {
    fn into(self) -> (T, T) {
        self.to_tuple()
    }
}

impl<T: Copy, U> From<(T, T)> for Vector2D<T, U> {
    fn from(tuple: (T, T)) -> Self {
        vec2(tuple.0, tuple.1)
    }
}

impl<T: Copy, U> From<Size2D<T, U>> for Vector2D<T, U> {
    fn from(size: Size2D<T, U>) -> Self {
        size.to_vector()
    }
}

impl<T, U> Vector2D<T, U>
where
    T: Signed,
{
    pub fn abs(&self) -> Self {
        vec2(self.x.abs(), self.y.abs())
    }
}

/// A 3d Vector tagged with a unit.
#[repr(C)]
pub struct Vector3D<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

mint_vec!(Vector3D[x, y, z] = Vector3);

impl<T: Copy, U> Copy for Vector3D<T, U> {}

impl<T: Clone, U> Clone for Vector3D<T, U> {
    fn clone(&self) -> Self {
        Vector3D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            _unit: PhantomData,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U> serde::Deserialize<'de> for Vector3D<T, U>
    where T: serde::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let (x, y, z) = try!(serde::Deserialize::deserialize(deserializer));
        Ok(Vector3D { x, y, z, _unit: PhantomData })
    }
}

#[cfg(feature = "serde")]
impl<T, U> serde::Serialize for Vector3D<T, U>
    where T: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        (&self.x, &self.y, &self.z).serialize(serializer)
    }
}

impl<T, U> Eq for Vector3D<T, U> where T: Eq {}

impl<T, U> PartialEq for Vector3D<T, U>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T, U> Hash for Vector3D<T, U>
    where T: Hash
{
    fn hash<H: ::core::hash::Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
        self.z.hash(h);
    }
}

impl<T: Copy + Zero, U> Vector3D<T, U> {
    /// Constructor, setting all components to zero.
    #[inline]
    pub fn zero() -> Self {
        vec3(Zero::zero(), Zero::zero(), Zero::zero())
    }

    #[inline]
    pub fn to_array_4d(&self) -> [T; 4] {
        [self.x, self.y, self.z, Zero::zero()]
    }

    #[inline]
    pub fn to_tuple_4d(&self) -> (T, T, T, T) {
        (self.x, self.y, self.z, Zero::zero())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Vector3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
    }
}

impl<T: fmt::Display, U> fmt::Display for Vector3D<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Default, U> Default for Vector3D<T, U> {
    fn default() -> Self {
        Vector3D::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T, U> Vector3D<T, U> {
    /// Constructor taking scalar values directly.
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3D {
            x,
            y,
            z,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, U> Vector3D<T, U> {
    /// Constructor taking properly  Lengths instead of scalar values.
    #[inline]
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>, z: Length<T, U>) -> Vector3D<T, U> {
        vec3(x.0, y.0, z.0)
    }

    /// Cast this vector into a point.
    ///
    /// Equivalent to adding this vector to the origin.
    #[inline]
    pub fn to_point(&self) -> Point3D<T, U> {
        point3(self.x, self.y, self.z)
    }

    /// Returns a 2d vector using this vector's x and y coordinates
    #[inline]
    pub fn xy(&self) -> Vector2D<T, U> {
        vec2(self.x, self.y)
    }

    /// Returns a 2d vector using this vector's x and z coordinates
    #[inline]
    pub fn xz(&self) -> Vector2D<T, U> {
        vec2(self.x, self.z)
    }

    /// Returns a 2d vector using this vector's x and z coordinates
    #[inline]
    pub fn yz(&self) -> Vector2D<T, U> {
        vec2(self.y, self.z)
    }

    #[inline]
    pub fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn to_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> Vector3D<T, UnknownUnit> {
        vec3(self.x, self.y, self.z)
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(p: Vector3D<T, UnknownUnit>) -> Self {
        vec3(p.x, p.y, p.z)
    }

    /// Convert into a 2d vector.
    #[inline]
    pub fn to_2d(&self) -> Vector2D<T, U> {
        self.xy()
    }
}

impl<T, U> Vector3D<T, U>
where
    T: Copy
        + Clone
        + Add<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Sub<T, Output = T>
        + Trig
        + PartialOrd
        + One
        + Zero
        + Neg<Output = T> {
    #[inline]
    pub fn to_transform(&self) -> Transform3D<T, U, U> {
        Transform3D::create_translation(self.x, self.y, self.z)
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy, U>
    Vector3D<T, U> {
    // Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    pub fn normalize(self) -> Self
    where
        T: Float,
    {
        self / self.length()
    }

    /// Return the normalized vector even if the length is larger than the max value of Float.
    #[inline]
    pub fn robust_normalize(self) -> Self
    where
        T: Float,
    {
        let length = self.length();
        if length.is_infinite() {
            let scaled = self / T::max_value();
            scaled / scaled.length()
        } else {
            self / length
        }
    }

    #[inline]
    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.square_length().sqrt()
    }
}

impl<T, U> Vector3D<T, U>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Linearly interpolate between this vector and another vector.
    ///
    /// `t` is expected to be between zero and one.
    #[inline]
    pub fn lerp(&self, other: Self, t: T) -> Self {
        let one_t = T::one() - t;
        (*self) * one_t + other * t
    }
}

impl<T: Copy + Add<T, Output = T>, U> Add for Vector3D<T, U> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        vec3(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Copy + Sub<T, Output = T>, U> Sub for Vector3D<T, U> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        vec3(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Copy + Add<T, Output = T>, U> AddAssign for Vector3D<T, U> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl<T: Copy + Sub<T, Output = T>, U> SubAssign<Vector3D<T, U>> for Vector3D<T, U> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl<T: Copy + Neg<Output = T>, U> Neg for Vector3D<T, U> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl<T: Copy + Mul<T, Output = T>, U> Mul<T> for Vector3D<T, U> {
    type Output = Self;
    #[inline]
    fn mul(self, scale: T) -> Self {
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl<T: Copy + Div<T, Output = T>, U> Div<T> for Vector3D<T, U> {
    type Output = Self;
    #[inline]
    fn div(self, scale: T) -> Self {
        Self::new(self.x / scale, self.y / scale, self.z / scale)
    }
}

impl<T: Copy + Mul<T, Output = T>, U> MulAssign<T> for Vector3D<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        *self = *self * scale
    }
}

impl<T: Copy + Div<T, Output = T>, U> DivAssign<T> for Vector3D<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        *self = *self / scale
    }
}

impl<T: Float, U> Vector3D<T, U> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        vec3(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        vec3(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    #[inline]
    pub fn clamp(&self, start: Self, end: Self) -> Self {
        self.max(start).min(end)
    }
}

impl<T: Copy + Mul<T, Output = T>, U1, U2> Mul<Scale<T, U1, U2>> for Vector3D<T, U1> {
    type Output = Vector3D<T, U2>;
    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        vec3(self.x * scale.get(), self.y * scale.get(), self.z * scale.get())
    }
}

impl<T: Copy + Div<T, Output = T>, U1, U2> Div<Scale<T, U1, U2>> for Vector3D<T, U2> {
    type Output = Vector3D<T, U1>;
    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        vec3(self.x / scale.get(), self.y / scale.get(), self.z / scale.get())
    }
}

impl<T: Round, U> Vector3D<T, U> {
    /// Rounds each component to the nearest integer value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    #[inline]
    #[must_use]
    pub fn round(&self) -> Self {
        vec3(self.x.round(), self.y.round(), self.z.round())
    }
}

impl<T: Ceil, U> Vector3D<T, U> {
    /// Rounds each component to the smallest integer equal or greater than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    #[inline]
    #[must_use]
    pub fn ceil(&self) -> Self {
        vec3(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl<T: Floor, U> Vector3D<T, U> {
    /// Rounds each component to the biggest integer equal or lower than the original value.
    ///
    /// This behavior is preserved for negative values (unlike the basic cast).
    #[inline]
    #[must_use]
    pub fn floor(&self) -> Self {
        vec3(self.x.floor(), self.y.floor(), self.z.floor())
    }
}

impl<T: NumCast + Copy, U> Vector3D<T, U> {
    /// Cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn cast<NewT: NumCast + Copy>(&self) -> Vector3D<NewT, U> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    ///
    /// When casting from floating vector to integer coordinates, the decimals are truncated
    /// as one would expect from a simple cast, but this behavior does not always make sense
    /// geometrically. Consider using `round()`, `ceil()` or `floor()` before casting.
    #[inline]
    pub fn try_cast<NewT: NumCast + Copy>(&self) -> Option<Vector3D<NewT, U>> {
        match (
            NumCast::from(self.x),
            NumCast::from(self.y),
            NumCast::from(self.z),
        ) {
            (Some(x), Some(y), Some(z)) => Some(vec3(x, y, z)),
            _ => None,
        }
    }

    // Convenience functions for common casts

    /// Cast into an `f32` vector.
    #[inline]
    pub fn to_f32(&self) -> Vector3D<f32, U> {
        self.cast()
    }

    /// Cast into an `f64` vector.
    #[inline]
    pub fn to_f64(&self) -> Vector3D<f64, U> {
        self.cast()
    }

    /// Cast into an `usize` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_usize(&self) -> Vector3D<usize, U> {
        self.cast()
    }

    /// Cast into an `u32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_u32(&self) -> Vector3D<u32, U> {
        self.cast()
    }

    /// Cast into an `i32` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i32(&self) -> Vector3D<i32, U> {
        self.cast()
    }

    /// Cast into an `i64` vector, truncating decimals if any.
    ///
    /// When casting from floating vector vectors, it is worth considering whether
    /// to `round()`, `ceil()` or `floor()` before the cast in order to obtain
    /// the desired conversion behavior.
    #[inline]
    pub fn to_i64(&self) -> Vector3D<i64, U> {
        self.cast()
    }
}

impl<T: Copy + ApproxEq<T>, U> ApproxEq<Vector3D<T, U>> for Vector3D<T, U> {
    #[inline]
    fn approx_epsilon() -> Self {
        vec3(
            T::approx_epsilon(),
            T::approx_epsilon(),
            T::approx_epsilon(),
        )
    }

    #[inline]
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x) && self.y.approx_eq(&other.y) && self.z.approx_eq(&other.z)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
            && self.z.approx_eq_eps(&other.z, &eps.z)
    }
}

impl<T: Copy, U> Into<[T; 3]> for Vector3D<T, U> {
    fn into(self) -> [T; 3] {
        self.to_array()
    }
}

impl<T: Copy, U> From<[T; 3]> for Vector3D<T, U> {
    fn from(array: [T; 3]) -> Self {
        vec3(array[0], array[1], array[2])
    }
}

impl<T: Copy, U> Into<(T, T, T)> for Vector3D<T, U> {
    fn into(self) -> (T, T, T) {
        self.to_tuple()
    }
}

impl<T: Copy, U> From<(T, T, T)> for Vector3D<T, U> {
    fn from(tuple: (T, T, T)) -> Self {
        vec3(tuple.0, tuple.1, tuple.2)
    }
}

impl<T, U> Vector3D<T, U>
where
    T: Signed,
{
    pub fn abs(&self) -> Self {
        vec3(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoolVector2D {
    pub x: bool,
    pub y: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoolVector3D {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl BoolVector2D {
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y
    }

    #[inline]
    pub fn none(&self) -> bool {
        !self.any()
    }

    #[inline]
    pub fn and(&self, other: Self) -> Self {
        BoolVector2D {
            x: self.x && other.x,
            y: self.y && other.y,
        }
    }

    #[inline]
    pub fn or(&self, other: Self) -> Self {
        BoolVector2D {
            x: self.x || other.x,
            y: self.y || other.y,
        }
    }

    #[inline]
    pub fn not(&self) -> Self {
        BoolVector2D {
            x: !self.x,
            y: !self.y,
        }
    }

    #[inline]
    pub fn select_point<T: Copy, U>(&self, a: Point2D<T, U>, b: Point2D<T, U>) -> Point2D<T, U> {
        point2(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
        )
    }

    #[inline]
    pub fn select_vector<T: Copy, U>(&self, a: Vector2D<T, U>, b: Vector2D<T, U>) -> Vector2D<T, U> {
        vec2(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
        )
    }

    #[inline]
    pub fn select_size<T: Copy, U>(&self, a: Size2D<T, U>, b: Size2D<T, U>) -> Size2D<T, U> {
        size2(
            if self.x { a.width } else { b.width },
            if self.y { a.height } else { b.height },
        )
    }
}

impl BoolVector3D {
    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y || self.z
    }

    #[inline]
    pub fn none(&self) -> bool {
        !self.any()
    }

    #[inline]
    pub fn and(&self, other: Self) -> Self {
        BoolVector3D {
            x: self.x && other.x,
            y: self.y && other.y,
            z: self.z && other.z,
        }
    }

    #[inline]
    pub fn or(&self, other: Self) -> Self {
        BoolVector3D {
            x: self.x || other.x,
            y: self.y || other.y,
            z: self.z || other.z,
        }
    }

    #[inline]
    pub fn not(&self) -> Self {
        BoolVector3D {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }


    #[inline]
    pub fn select_point<T: Copy, U>(&self, a: Point3D<T, U>, b: Point3D<T, U>) -> Point3D<T, U> {
        point3(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
            if self.z { a.z } else { b.z },
        )
    }

    #[inline]
    pub fn select_vector<T: Copy, U>(&self, a: Vector3D<T, U>, b: Vector3D<T, U>) -> Vector3D<T, U> {
        vec3(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
            if self.z { a.z } else { b.z },
        )
    }

    #[inline]
    pub fn xy(&self) -> BoolVector2D {
        BoolVector2D {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    pub fn xz(&self) -> BoolVector2D {
        BoolVector2D {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    pub fn yz(&self) -> BoolVector2D {
        BoolVector2D {
            x: self.y,
            y: self.z,
        }
    }
}

impl<T: PartialOrd, U> Vector2D<T, U> {
    #[inline]
    pub fn greater_than(&self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.x > other.x,
            y: self.y > other.y,
        }
    }

    #[inline]
    pub fn lower_than(&self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.x < other.x,
            y: self.y < other.y,
        }
    }
}


impl<T: PartialEq, U> Vector2D<T, U> {
    #[inline]
    pub fn equal(&self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.x == other.x,
            y: self.y == other.y,
        }
    }

    #[inline]
    pub fn not_equal(&self, other: Self) -> BoolVector2D {
        BoolVector2D {
            x: self.x != other.x,
            y: self.y != other.y,
        }
    }
}

impl<T: PartialOrd, U> Vector3D<T, U> {
    #[inline]
    pub fn greater_than(&self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.x > other.x,
            y: self.y > other.y,
            z: self.z > other.z,
        }
    }

    #[inline]
    pub fn lower_than(&self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.x < other.x,
            y: self.y < other.y,
            z: self.z < other.z,
        }
    }
}


impl<T: PartialEq, U> Vector3D<T, U> {
    #[inline]
    pub fn equal(&self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.x == other.x,
            y: self.y == other.y,
            z: self.z == other.z,
        }
    }

    #[inline]
    pub fn not_equal(&self, other: Self) -> BoolVector3D {
        BoolVector3D {
            x: self.x != other.x,
            y: self.y != other.y,
            z: self.z != other.z,
        }
    }
}

/// Convenience constructor.
#[inline]
pub fn vec2<T, U>(x: T, y: T) -> Vector2D<T, U> {
    Vector2D {
        x,
        y,
        _unit: PhantomData,
    }
}

/// Convenience constructor.
#[inline]
pub fn vec3<T, U>(x: T, y: T, z: T) -> Vector3D<T, U> {
    Vector3D {
        x,
        y,
        z,
        _unit: PhantomData,
    }
}

#[inline]
pub fn bvec2(x: bool, y: bool) -> BoolVector2D {
    BoolVector2D { x, y }
}

#[inline]
pub fn bvec3(x: bool, y: bool, z: bool) -> BoolVector3D {
    BoolVector3D { x, y, z }
}


#[cfg(test)]
mod vector2d {
    use {default, vec2};
    use scale::Scale;

    #[cfg(feature = "mint")]
    use mint;
    type Vec2 = default::Vector2D<f32>;

    #[test]
    pub fn test_scalar_mul() {
        let p1: Vec2 = vec2(3.0, 5.0);

        let result = p1 * 5.0;

        assert_eq!(result, Vec2::new(15.0, 25.0));
    }

    #[test]
    pub fn test_dot() {
        let p1: Vec2 = vec2(2.0, 7.0);
        let p2: Vec2 = vec2(13.0, 11.0);
        assert_eq!(p1.dot(p2), 103.0);
    }

    #[test]
    pub fn test_cross() {
        let p1: Vec2 = vec2(4.0, 7.0);
        let p2: Vec2 = vec2(13.0, 8.0);
        let r = p1.cross(p2);
        assert_eq!(r, -59.0);
    }

    #[test]
    pub fn test_normalize() {
        let p0: Vec2 = Vec2::zero();
        let p1: Vec2 = vec2(4.0, 0.0);
        let p2: Vec2 = vec2(3.0, -4.0);
        assert!(p0.normalize().x.is_nan() && p0.normalize().y.is_nan());
        assert_eq!(p1.normalize(), vec2(1.0, 0.0));
        assert_eq!(p2.normalize(), vec2(0.6, -0.8));

        let p3: Vec2 = vec2(::std::f32::MAX, ::std::f32::MAX);
        assert_ne!(p3.normalize(), vec2(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt()));
        assert_eq!(p3.robust_normalize(), vec2(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt()));
    }

    #[test]
    pub fn test_min() {
        let p1: Vec2 = vec2(1.0, 3.0);
        let p2: Vec2 = vec2(2.0, 2.0);

        let result = p1.min(p2);

        assert_eq!(result, vec2(1.0, 2.0));
    }

    #[test]
    pub fn test_max() {
        let p1: Vec2 = vec2(1.0, 3.0);
        let p2: Vec2 = vec2(2.0, 2.0);

        let result = p1.max(p2);

        assert_eq!(result, vec2(2.0, 3.0));
    }
    #[test]
    pub fn test_angle_from_x_axis() {
        use core::f32::consts::FRAC_PI_2;
        use approxeq::ApproxEq;

        let right: Vec2 = vec2(10.0, 0.0);
        let down: Vec2 = vec2(0.0, 4.0);
        let up: Vec2 = vec2(0.0, -1.0);

        assert!(right.angle_from_x_axis().get().approx_eq(&0.0));
        assert!(down.angle_from_x_axis().get().approx_eq(&FRAC_PI_2));
        assert!(up.angle_from_x_axis().get().approx_eq(&-FRAC_PI_2));
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let v1 = Vec2::new(1.0, 3.0);
        let vm: mint::Vector2<_> = v1.into();
        let v2 = Vec2::from(vm);

        assert_eq!(v1, v2);
    }

    pub enum Mm {}
    pub enum Cm {}

    pub type Vector2DMm<T> = super::Vector2D<T, Mm>;
    pub type Vector2DCm<T> = super::Vector2D<T, Cm>;

    #[test]
    pub fn test_add() {
        let p1 = Vector2DMm::new(1.0, 2.0);
        let p2 = Vector2DMm::new(3.0, 4.0);

        let result = p1 + p2;

        assert_eq!(result, vec2(4.0, 6.0));
    }

    #[test]
    pub fn test_add_assign() {
        let mut p1 = Vector2DMm::new(1.0, 2.0);
        p1 += vec2(3.0, 4.0);

        assert_eq!(p1, vec2(4.0, 6.0));
    }

    #[test]
    pub fn test_tpyed_scalar_mul() {
        let p1 = Vector2DMm::new(1.0, 2.0);
        let cm_per_mm = Scale::<f32, Mm, Cm>::new(0.1);

        let result: Vector2DCm<f32> = p1 * cm_per_mm;

        assert_eq!(result, vec2(0.1, 0.2));
    }

    #[test]
    pub fn test_swizzling() {
        let p: default::Vector2D<i32> = vec2(1, 2);
        assert_eq!(p.yx(), vec2(2, 1));
    }
}

#[cfg(test)]
mod vector3d {
    #[cfg(feature = "mint")]
    use mint;
    use {default, vec2, vec3};
    use scale::Scale;

    type Vec3 = default::Vector3D<f32>;

    #[test]
    pub fn test_dot() {
        let p1: Vec3 = vec3(7.0, 21.0, 32.0);
        let p2: Vec3 = vec3(43.0, 5.0, 16.0);
        assert_eq!(p1.dot(p2), 918.0);
    }

    #[test]
    pub fn test_cross() {
        let p1: Vec3 = vec3(4.0, 7.0, 9.0);
        let p2: Vec3 = vec3(13.0, 8.0, 3.0);
        let p3 = p1.cross(p2);
        assert_eq!(p3, vec3(-51.0, 105.0, -59.0));
    }

    #[test]
    pub fn test_normalize() {
        let p0: Vec3 = Vec3::zero();
        let p1: Vec3 = vec3(0.0, -6.0, 0.0);
        let p2: Vec3 = vec3(1.0, 2.0, -2.0);
        assert!(
            p0.normalize().x.is_nan() && p0.normalize().y.is_nan() && p0.normalize().z.is_nan()
        );
        assert_eq!(p1.normalize(), vec3(0.0, -1.0, 0.0));
        assert_eq!(p2.normalize(), vec3(1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0));

        let p3: Vec3 = vec3(::std::f32::MAX, ::std::f32::MAX, 0.0);
        assert_ne!(p3.normalize(), vec3(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0));
        assert_eq!(p3.robust_normalize(), vec3(1.0 / 2.0f32.sqrt(), 1.0 / 2.0f32.sqrt(), 0.0));
    }

    #[test]
    pub fn test_min() {
        let p1: Vec3 = vec3(1.0, 3.0, 5.0);
        let p2: Vec3 = vec3(2.0, 2.0, -1.0);

        let result = p1.min(p2);

        assert_eq!(result, vec3(1.0, 2.0, -1.0));
    }

    #[test]
    pub fn test_max() {
        let p1: Vec3 = vec3(1.0, 3.0, 5.0);
        let p2: Vec3 = vec3(2.0, 2.0, -1.0);

        let result = p1.max(p2);

        assert_eq!(result, vec3(2.0, 3.0, 5.0));
    }

    #[test]
    pub fn test_clamp() {
        let p1: Vec3 = vec3(1.0, -1.0, 5.0);
        let p2: Vec3 = vec3(2.0, 5.0, 10.0);
        let p3: Vec3 = vec3(-1.0, 2.0, 20.0);

        let result = p3.clamp(p1, p2);

        assert_eq!(result, vec3(1.0, 2.0, 10.0));
    }

    #[test]
    pub fn test_typed_scalar_mul() {
        enum Mm {}
        enum Cm {}

        let p1 = super::Vector3D::<f32, Mm>::new(1.0, 2.0, 3.0);
        let cm_per_mm = Scale::<f32, Mm, Cm>::new(0.1);

        let result: super::Vector3D<f32, Cm> = p1 * cm_per_mm;

        assert_eq!(result, vec3(0.1, 0.2, 0.3));
    }

    #[test]
    pub fn test_swizzling() {
        let p: Vec3 = vec3(1.0, 2.0, 3.0);
        assert_eq!(p.xy(), vec2(1.0, 2.0));
        assert_eq!(p.xz(), vec2(1.0, 3.0));
        assert_eq!(p.yz(), vec2(2.0, 3.0));
    }

    #[cfg(feature = "mint")]
    #[test]
    pub fn test_mint() {
        let v1 = Vec3::new(1.0, 3.0, 5.0);
        let vm: mint::Vector3<_> = v1.into();
        let v2 = Vec3::from(vm);

        assert_eq!(v1, v2);
    }
}

#[cfg(test)]
mod bool_vector {
    use default;
    use super::*;
    type Vec2 = default::Vector2D<f32>;
    type Vec3 = default::Vector3D<f32>;

    #[test]
    fn test_bvec2() {

        assert_eq!(
            Vec2::new(1.0, 2.0).greater_than(Vec2::new(2.0, 1.0)),
            bvec2(false, true),
        );

        assert_eq!(
            Vec2::new(1.0, 2.0).lower_than(Vec2::new(2.0, 1.0)),
            bvec2(true, false),
        );

        assert_eq!(
            Vec2::new(1.0, 2.0).equal(Vec2::new(1.0, 3.0)),
            bvec2(true, false),
        );

        assert_eq!(
            Vec2::new(1.0, 2.0).not_equal(Vec2::new(1.0, 3.0)),
            bvec2(false, true),
        );

        assert!(bvec2(true, true).any());
        assert!(bvec2(false, true).any());
        assert!(bvec2(true, false).any());
        assert!(!bvec2(false, false).any());
        assert!(bvec2(false, false).none());
        assert!(bvec2(true, true).all());
        assert!(!bvec2(false, true).all());
        assert!(!bvec2(true, false).all());
        assert!(!bvec2(false, false).all());

        assert_eq!(bvec2(true, false).not(), bvec2(false, true));
        assert_eq!(bvec2(true, false).and(bvec2(true, true)), bvec2(true, false));
        assert_eq!(bvec2(true, false).or(bvec2(true, true)), bvec2(true, true));

        assert_eq!(
            bvec2(true, false).select_vector(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)),
            Vec2::new(1.0, 4.0),
        );
    }

    #[test]
    fn test_bvec3() {

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).greater_than(Vec3::new(3.0, 2.0, 1.0)),
            bvec3(false, false, true),
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).lower_than(Vec3::new(3.0, 2.0, 1.0)),
            bvec3(true, false, false),
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).equal(Vec3::new(3.0, 2.0, 1.0)),
            bvec3(false, true, false),
        );

        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).not_equal(Vec3::new(3.0, 2.0, 1.0)),
            bvec3(true, false, true),
        );

        assert!(bvec3(true, true, false).any());
        assert!(bvec3(false, true, false).any());
        assert!(bvec3(true, false, false).any());
        assert!(!bvec3(false, false, false).any());
        assert!(bvec3(false, false, false).none());
        assert!(bvec3(true, true, true).all());
        assert!(!bvec3(false, true, false).all());
        assert!(!bvec3(true, false, false).all());
        assert!(!bvec3(false, false, false).all());

        assert_eq!(bvec3(true, false, true).not(), bvec3(false, true, false));
        assert_eq!(bvec3(true, false, true).and(bvec3(true, true, false)), bvec3(true, false, false));
        assert_eq!(bvec3(true, false, false).or(bvec3(true, true, false)), bvec3(true, true, false));

        assert_eq!(
            bvec3(true, false, true).select_vector(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0)),
            Vec3::new(1.0, 5.0, 3.0),
        );
    }
}
