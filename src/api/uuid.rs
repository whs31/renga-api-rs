use std::{
  fmt::Display, hash::Hash, str::FromStr
};
use guid::{
  GUID,
  guid,
  guid_parts_impl
};

#[derive(Debug, Clone)]
pub struct UUID(GUID);

impl From<GUID> for UUID { fn from(guid: GUID) -> Self { Self(guid) } }
impl From<&GUID> for UUID { fn from(guid: &GUID) -> Self { Self(*guid) } }

impl UUID {
  pub fn into_inner(self) -> GUID { self.0 }
  pub fn as_guid(&self) -> &GUID { &self.0 }
}

impl Default for UUID {
  fn default() -> Self { Self(guid!("00000000-0000-0000-0000-000000000000")) }
}

impl Hash for UUID {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.Data1.hash(state);
    self.0.Data2.hash(state);
    self.0.Data3.hash(state);
    self.0.Data4.hash(state);
  }
}

impl Display for UUID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let formatted = format!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
      self.0.Data1, 
      self.0.Data2, 
      self.0.Data3, 
      self.0.Data4[0], 
      self.0.Data4[1], 
      self.0.Data4[2], 
      self.0.Data4[3], 
      self.0.Data4[4], 
      self.0.Data4[5], 
      self.0.Data4[6], 
      self.0.Data4[7]
    );
    write!(f, "{}", formatted)
  }
}

impl FromStr for UUID {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parsed = guid::parse(s);
    match parsed {
      Ok(guid) => Ok(Self(guid)),
      Err(_) => Err(crate::Error::ParseError(format!("Failed to parse UUID from string: {}", s)))
    }
  }
}