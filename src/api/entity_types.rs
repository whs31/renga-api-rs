use std::{
  fmt::Display, 
  str::FromStr
};
use super::{
  GUID,
  guid,
  guid_parts_impl
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityTypes {
  CategoryType(Category)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
  DuctAccessory,
  DuctFitting,
  ElectricDistributionBoard,
  Equipment,
  LightingFixture,
  MechanicalEquipment,
  PipeAccessory,
  PipeFitting,
  PlumbingFixture,
  WiringAccessory 
}

impl Category {
  pub fn as_guid(&self) -> GUID {
    match self {
      Self::DuctAccessory => guid!("46c07d12-8f76-4537-a473-08d52395baba"),
      Self::DuctFitting => guid!("68eff079-2b52-4e05-a51b-6875d1cdb9fc"),
      Self::ElectricDistributionBoard => guid!("d547f002-4a74-41bf-b1f0-ed8f5846098f"),
      Self::Equipment => guid!("4cd3bc4c-14da-43ca-bbc5-d7679566b8dd"),
      Self::LightingFixture => guid!("c59fd4c5-4050-47a0-b11a-f52c4799470c"),
      Self::MechanicalEquipment => guid!("d7e202ce-791c-4123-adbe-5f6357bf85e6"),
      Self::PipeAccessory => guid!("17c36f59-54dc-4440-8b78-034b0adb8716"),
      Self::PipeFitting => guid!("8b5cf8f2-a391-4701-8cb9-d6a6ba5ee46f"),
      Self::PlumbingFixture => guid!("10bc8911-5931-471a-9c0e-74ad36a7ee8a"),
      Self::WiringAccessory => guid!("2c07d135-8343-418d-a1c2-ea074d98db31")
    }
  }

  pub fn to_sanitized_string(&self) -> String {
    Self::guid_to_string(self.as_guid())
  }

  fn guid_to_string(guid: GUID) -> String {
    format!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
      guid.Data1, 
      guid.Data2, 
      guid.Data3, 
      guid.Data4[0], 
      guid.Data4[1], 
      guid.Data4[2], 
      guid.Data4[3], 
      guid.Data4[4], 
      guid.Data4[5], 
      guid.Data4[6], 
      guid.Data4[7]
    )
  }
}

impl Display for EntityTypes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Display for Category {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?} ({{{:?}}})", self, self.to_sanitized_string())
  }
}

impl From<&Category> for GUID { fn from(category: &Category) -> Self { category.as_guid() } }
impl From<Category> for GUID { fn from(category: Category) -> Self { category.as_guid() } }
impl FromStr for Category { 
  type Err = crate::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> { 
    let sanitized = s
      .trim()
      .to_lowercase()
      .trim_end_matches("_category")
      .to_owned();
    match sanitized.as_str() {
      "duct_accessory" => Ok(Self::DuctAccessory),
      "duct_fitting" => Ok(Self::DuctFitting),
      "electric_distribution_board" => Ok(Self::ElectricDistributionBoard),
      "equipment" => Ok(Self::Equipment),
      "lighting_fixture" => Ok(Self::LightingFixture),
      "mechanical_equipment" => Ok(Self::MechanicalEquipment),
      "pipe_accessory" => Ok(Self::PipeAccessory),
      "pipe_fitting" => Ok(Self::PipeFitting),
      "plumbing_fixture" => Ok(Self::PlumbingFixture),
      "wiring_accessory" => Ok(Self::WiringAccessory),
      _ => Err(crate::Error::ParseError(s.to_owned()))
    }
  } 
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_guid() {
    assert_eq!(
      Category::ElectricDistributionBoard.to_sanitized_string(), 
      "D547F002-4A74-41BF-B1F0-ED8F5846098F"
    );
  }
}