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
    let record_ptr = variant.as_record_unchecked()?.0;
    let version = std::mem::transmute::<_, &Version>(record_ptr);
    Ok(version.clone())
  }
}