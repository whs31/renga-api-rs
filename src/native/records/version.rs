use crate::{
  Result,
  native::Variant
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub build: i32
}

impl Version {
  pub unsafe fn from_variant(variant: &Variant) -> Result<Self> {
    variant.as_record_unchecked::<Self>()
  }
}