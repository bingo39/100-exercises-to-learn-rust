// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

/*
要求：
定义一个新的 SaturatingU16 类型。它应该：

1.内部持有一个 u16 值
2.。支持从 u16、u8、&u16、&u8 的类型转换（From trait）
3.支持加法运算，右侧操作数可以是 SaturatingU16、u16、&u16、&SaturatingU16。加法应该使用饱和运算，即溢出时停在 u16 的最大值而不是回绕
4.可以与另一个 SaturatingU16 或 u16 进行比较（PartialEq）
5.可以打印调试表示（Debug）
测试在 tests 文件夹里，注意类型和方法的可见性要设为 pub */

use std::ops::Add;
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SaturatingU16{
     value:u16
}

// u16
impl From<u16> for SaturatingU16 {
    fn from(value:u16) -> Self {
        SaturatingU16 { value }
    }
}
// u8->u16
impl From<u8> for SaturatingU16 {
     fn from(value: u8) -> Self {
        SaturatingU16 { value:u16::from(value)}
    }
}
// &u16->u16
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value}
    }
}
// &u8->u16
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 { value: *value as u16 }
    }
}

/*add */
//SaturatingU16
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16{value:self.value.saturating_add(rhs.value)}
    }  
}
//u16
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16{value:self.value.saturating_add(rhs)}
    }
}
//&u16
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16{value:self.value.saturating_add(*rhs)}
    }
}
//&SaturatingU16
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16{value:self.value.saturating_add(*&rhs.value)}
    }
}

/* PartialEq*/
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
