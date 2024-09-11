use std::fmt::Display;
use windows::{
  core::{
    HSTRING, 
    PCWSTR,
    GUID
  }, 
  Win32::System::Com::CLSIDFromProgID
};
use crate::Result;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ClassID(GUID);

impl ClassID {
  pub fn new(clsid: &str) -> Result<Self> {
    unsafe {
      Ok(Self(CLSIDFromProgID(PCWSTR::from_raw(HSTRING::from(clsid).as_ptr()))?))
    }
  }

  pub fn as_guid(&self) -> &GUID { &self.0 }
}

impl From<GUID> for ClassID {
  fn from(guid: GUID) -> Self { Self(guid) }
}

impl From<&GUID> for ClassID {
  fn from(guid: &GUID) -> Self { Self(guid.clone()) }
}

impl TryFrom<&str> for ClassID {
  type Error = crate::error::Error;
  fn try_from(s: &str) -> std::result::Result<Self, Self::Error> { Self::new(s) }
}

impl TryFrom<String> for ClassID {
  type Error = crate::error::Error;
  fn try_from(s: String) -> std::result::Result<Self, Self::Error> { Self::new(&s) }
}

impl AsRef<GUID> for ClassID {
  fn as_ref(&self) -> &GUID { &self.0 }
}

impl Display for ClassID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{{{:?}}}", self.0) }
}