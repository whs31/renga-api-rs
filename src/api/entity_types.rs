use std::{
  fmt::Display, 
  str::FromStr
};
use super::{
  UUID,
  guid,
  guid_parts_impl
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityTypes {
  CategoryType(Category)
}

/// Project category entity types.
/// 
/// See [Official documentation](https://help.rengabim.com/api/group___project_info_types.html) for details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
  /// Duct accessory category type. 
  DuctAccessory,

  /// Duct fitting category type.
  DuctFitting,

  /// Electric distribution board category type.
  ElectricDistributionBoard,

  /// Equipment category type.
  Equipment,

  /// Lighting fixture category type.
  LightingFixture,

  /// Mechanical equipment category type.
  MechanicalEquipment,

  /// Pipe accessory category type.
  PipeAccessory,

  /// Pipe fitting category type.
  PipeFitting,

  /// Plumbing fixture category type.
  PlumbingFixture,

  /// Wiring accessory category type.
  WiringAccessory 
}

impl Category {
  /// Returns GUID of the category.
  pub fn as_uuid(&self) -> UUID {
    match self {
      Self::DuctAccessory => UUID::from(guid!("46c07d12-8f76-4537-a473-08d52395baba")),
      Self::DuctFitting => UUID::from(guid!("68eff079-2b52-4e05-a51b-6875d1cdb9fc")),
      Self::ElectricDistributionBoard => UUID::from(guid!("d547f002-4a74-41bf-b1f0-ed8f5846098f")),
      Self::Equipment => UUID::from(guid!("4cd3bc4c-14da-43ca-bbc5-d7679566b8dd")),
      Self::LightingFixture => UUID::from(guid!("c59fd4c5-4050-47a0-b11a-f52c4799470c")),
      Self::MechanicalEquipment => UUID::from(guid!("d7e202ce-791c-4123-adbe-5f6357bf85e6")),
      Self::PipeAccessory => UUID::from(guid!("17c36f59-54dc-4440-8b78-034b0adb8716")),
      Self::PipeFitting => UUID::from(guid!("8b5cf8f2-a391-4701-8cb9-d6a6ba5ee46f")),
      Self::PlumbingFixture => UUID::from(guid!("10bc8911-5931-471a-9c0e-74ad36a7ee8a")),
      Self::WiringAccessory => UUID::from(guid!("2c07d135-8343-418d-a1c2-ea074d98db31"))
    }
  }

  /// Returns sanitized string representation of the GUID.
  /// - Braces are omitted.
  /// - Hexadecimal digits are represented in upper case.
  /// 
  /// Example: 
  /// ```rust
  /// use renga_api_rs as renga;
  /// assert_eq!(
  ///   renga::Category::ElectricDistributionBoard.to_sanitized_string(), 
  ///   "D547F002-4A74-41BF-B1F0-ED8F5846098F"
  /// );
  /// ```
  pub fn to_sanitized_string(&self) -> String {
    self.as_uuid().to_string()
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

impl From<&Category> for UUID { fn from(category: &Category) -> Self { category.as_uuid() } }
impl From<Category> for UUID { fn from(category: Category) -> Self { category.as_uuid() } }
impl FromStr for Category { 
  type Err = crate::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> { 
    let sanitized = s
      .trim()
      .to_lowercase()
      .replace(" ", "_")
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
  use crate::*;

  #[test]
  fn test_to_uuid() {
    assert_eq!(
      Category::ElectricDistributionBoard.to_sanitized_string(), 
      "D547F002-4A74-41BF-B1F0-ED8F5846098F"
    );
  }
}