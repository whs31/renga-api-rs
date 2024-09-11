use std::{
  ffi::c_void, 
  fmt::Display
};
use windows::{
  core::{
    BSTR,
    VARIANT
  },
  Win32::System::Com::IDispatch
};
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant(VARIANT);

impl Display for Variant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<VARIANT> for Variant { fn from(value: VARIANT) -> Self { Self(value) } }
impl From<&VARIANT> for Variant { fn from(value: &VARIANT) -> Self { Self(value.clone()) } }
impl From<Variant> for VARIANT { fn from(value: Variant) -> Self { value.0 } }

impl From<()> for Variant { fn from(_: ()) -> Self { Self(VARIANT::default()) } }
impl From<bool> for Variant { fn from(value: bool) -> Self { Self(VARIANT::from(value)) } }
impl From<u8> for Variant { fn from(value: u8) -> Self { Self(VARIANT::from(value)) } }
impl From<u16> for Variant { fn from(value: u16) -> Self { Self(VARIANT::from(value)) } }
impl From<u32> for Variant { fn from(value: u32) -> Self { Self(VARIANT::from(value)) } }
impl From<u64> for Variant { fn from(value: u64) -> Self { Self(VARIANT::from(value)) } }
impl From<i8> for Variant { fn from(value: i8) -> Self { Self(VARIANT::from(value)) } }
impl From<i16> for Variant { fn from(value: i16) -> Self { Self(VARIANT::from(value)) } }
impl From<i32> for Variant { fn from(value: i32) -> Self { Self(VARIANT::from(value)) } }
impl From<i64> for Variant { fn from(value: i64) -> Self { Self(VARIANT::from(value)) } }
impl From<f32> for Variant { fn from(value: f32) -> Self { Self(VARIANT::from(value)) } }
impl From<f64> for Variant { fn from(value: f64) -> Self { Self(VARIANT::from(value)) } }
impl From<&str> for Variant { fn from(value: &str) -> Self { Self(BSTR::from(value).into()) } }
impl From<&String> for Variant { fn from(value: &String) -> Self { Self(BSTR::from(value).into()) } }
impl From<String> for Variant { fn from(value: String) -> Self { Self(BSTR::from(value).into()) } }

impl Variant {
  pub fn as_bool(&self) -> Result<bool> { Ok(bool::try_from(&self.0)?) }
  pub fn as_u16(&self) -> Result<u16> { Ok(u16::try_from(&self.0)?) }
  pub fn as_u32(&self) -> Result<u32> { Ok(u32::try_from(&self.0)?) }
  pub fn as_u64(&self) -> Result<u64> { Ok(u64::try_from(&self.0)?) }
  pub fn as_i16(&self) -> Result<i16> { Ok(i16::try_from(&self.0)?) }
  pub fn as_i32(&self) -> Result<i32> { Ok(i32::try_from(&self.0)?) }
  pub fn as_i64(&self) -> Result<i64> { Ok(i64::try_from(&self.0)?) }
  pub fn as_f64(&self) -> Result<f64> { Ok(f64::try_from(&self.0)?) }
  pub fn as_int(&self) -> Result<i32> { Ok(i32::try_from(&self.0)?) }
  pub fn as_unsigned(&self) -> Result<u32> { Ok(u32::try_from(&self.0)?) }
  pub fn as_double(&self) -> Result<f64> { Ok(f64::try_from(&self.0)?) }

  pub unsafe fn as_record_unchecked(&self) -> Result<(*mut c_void, *mut c_void)> {
    Ok((
      self.0.as_raw().Anonymous.Anonymous.Anonymous.Anonymous.pvRecord,
      self.0.as_raw().Anonymous.Anonymous.Anonymous.Anonymous.pRecInfo
    ))
  }
  
  pub fn into_string(&self) -> Result<String> { Ok(BSTR::try_from(&self.0)?.to_string()) }
  pub fn into_dispatch(&self) -> Result<super::Dispatch> {
    Ok(super::Dispatch::from(IDispatch::try_from(&self.0)?))
  }

  pub unsafe fn vt(&self) -> u16 {
    self.0.as_raw().Anonymous.Anonymous.vt
  }
}


// #[allow(non_camel_case_types)]
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// enum VariantType {
//   VT_EMPTY = 0,
//   VT_NULL = 1,
//   VT_I2 = 2,
//   VT_I4 = 3,
//   VT_R4 = 4,
//   VT_R8 = 5,
//   VT_CY = 6,
//   VT_DATE = 7,
//   VT_BSTR = 8,
//   VT_DISPATCH = 9,
//   VT_ERROR = 10,
//   VT_BOOL = 11,
//   VT_VARIANT = 12,
//   VT_UNKNOWN = 13,
//   VT_DECIMAL = 14,
//   VT_I1 = 16,
//   VT_UI1 = 17,
//   VT_UI2 = 18,
//   VT_UI4 = 19,
//   VT_I8 = 20,
//   VT_UI8 = 21,
//   VT_INT = 22,
//   VT_UINT = 23,
//   VT_VOID = 24,
//   VT_HRESULT = 25,
//   VT_PTR = 26,
//   VT_SAFEARRAY = 27,
//   VT_CARRAY = 28,
//   VT_USERDEFINED = 29,
//   VT_LPSTR = 30,
//   VT_LPWSTR = 31,
//   VT_RECORD = 36,
//   VT_INT_PTR = 37,
//   VT_UINT_PTR = 38
// }

// const VT_BYREF: u16 = 16384;

// impl VariantType {
//   pub fn by_ref(self) -> u16 { self as u16 | VT_BYREF }
// }