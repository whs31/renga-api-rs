use std::fmt::{Debug, Display};
use crate::{
  native::Dispatch, 
  Result,
  Error
};
use super::UUID;

#[derive(Clone)]
pub struct Entity {
  handle: Dispatch
}

impl Entity {
  pub fn new(handle: Dispatch) -> Result<Self> { 
    if handle.is_null() {
      return Err(Error::Internal("Entity handle is null".to_owned()));
    }
    Ok(Self { handle }) 
  }

  pub fn id(&self) -> Result<i32> {
    self.handle.get("Id")?.as_int()
  }

  pub fn name(&self) -> Result<String> {
    self.handle.get("Name")?.into_string()
  }

  pub fn type_id(&self) -> Result<UUID> {
    self
      .handle
      .get("TypeIdS")?
      .into_string()?
      .parse()
  }

  pub fn unique_id(&self) -> Result<UUID> {
    self
      .handle
      .get("UniqueIdS")?
      .into_string()?
      .parse()
  }
}

impl Debug for Entity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Entity {{name: {}, id: {}, type: {}, uuid: {})}}", 
      self.name().unwrap_or("unknown".to_owned()),
      self.id().unwrap_or(-1),
      self.type_id().unwrap_or_default(),
      self.unique_id().unwrap_or_default()
    )
  }
}

impl Display for Entity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name().unwrap_or("<unknown entity>".to_owned()))
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct EntityCollection {
  handle: Dispatch
}

impl EntityCollection {
  pub fn new(handle: Dispatch) -> Result<Self> {
    if handle.is_null() {
      return Err(Error::Internal("EntityCollection handle is null".to_owned()));
    }
    Ok(Self { handle })
  }

  pub fn contains_id(&self, id: i32) -> Result<bool> {
    self.handle.call("Contains", Some(vec![id.into()]))?.as_bool()
  }
  pub fn contains_uuid(&self, uuid: UUID) -> Result<bool> {
    let uuid_str = uuid.to_string();
    self
      .handle
      .call("ContainsUniqueIdS", Some(vec![uuid_str.into()]))?
      .as_bool()
  }
  pub fn get_by_id(&self, id: i32) -> Result<Entity> {
    self
      .handle
      .call("GetById", Some(vec![id.into()]))?
      .into_dispatch()?
      .try_into()
  }

  pub fn get(&self, index: usize) -> Result<Entity> {
    let idx = index as i32;
    self
      .handle
      .call("GetByIndex", Some(vec![idx.into()]))?
      .into_dispatch()?
      .try_into()
  }

  pub fn get_by_uuid(&self, uuid: UUID) -> Result<Entity> {
    let uuid_str = uuid.to_string();
    self
      .handle
      .call("GetByUniqueIdS", Some(vec![uuid_str.into()]))?
      .into_dispatch()?
      .try_into()
  }

  pub fn len(&self) -> Result<usize> {
    self
      .handle
      .get("Count")?
      .as_int()
      .map(|count| count as usize)
  }
}

impl TryFrom<Dispatch> for Entity {
  type Error = Error;
  fn try_from(handle: Dispatch) -> Result<Self> {
    Self::new(handle)
  }
}

impl TryFrom<Dispatch> for EntityCollection {
  type Error = Error;
  fn try_from(handle: Dispatch) -> Result<Self> {
    Self::new(handle)
  }
}