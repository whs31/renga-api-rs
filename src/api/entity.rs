use std::fmt::{Debug, Display};
use crate::{
  native::Dispatch, 
  Result,
  Error
};
use super::UUID;

#[derive(Debug, Clone)]
pub struct Entity {
  pub id: i32,
  pub name: String,
  pub type_id: UUID,
  pub unique_id: UUID,
  _handle: Dispatch
}

impl Default for Entity {
  fn default() -> Self {
    Self {
      id: 0,
      name: String::default(),
      type_id: UUID::default(),
      unique_id: UUID::default(),
      _handle: ().into()
    }
  }
}

impl Entity {
  pub fn new(handle: Dispatch) -> Result<Self> { 
    if handle.is_null() {
      return Err(Error::Internal("Entity handle is null".to_owned()));
    }
    Ok(Self { 
      id: handle
        .get("Id")?
        .as_int()?,
      name: handle
        .get("Name")?
        .into_string()?,
      type_id: handle
        .get("TypeIdS")?
        .into_string()?
        .trim_start_matches("{")
        .trim_end_matches("}")
        .parse()?,
      unique_id: handle
        .get("UniqueIdS")?
        .into_string()?
        .trim_start_matches("{")
        .trim_end_matches("}")
        .parse()?,
      _handle: handle 
    }) 
  }
}

impl Display for Entity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<entity: {}>", match self.name.as_str().is_empty() {
      true => "<unknown entity>".to_owned(),
      false => self.name.clone()
    })
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

  // pub fn contains_id(&self, id: i32) -> Result<bool> {
  //   self.handle.call("Contains", Some(vec![id.into()]))?.as_bool()
  // }

  // pub fn contains_uuid(&self, uuid: UUID) -> Result<bool> {
  //   let uuid_str = uuid.to_string();
  //   self
  //     .handle
  //     .call("ContainsUniqueIdS", Some(vec![uuid_str.into()]))?
  //     .as_bool()
  // }

  // pub fn get_by_id(&self, id: i32) -> Result<Entity> {
  //   self
  //     .handle
  //     .call("GetById", Some(vec![id.into()]))?
  //     .into_dispatch()?
  //     .try_into()
  // }

  pub fn get(&self, index: usize) -> Result<Entity> {
    let idx = index as i32;
    self
      .handle
      .call("GetByIndex", Some(vec![idx.into()]))?
      .into_dispatch()?
      .try_into()
  }

  // pub fn get_by_uuid(&self, uuid: UUID) -> Result<Entity> {
  //   let uuid_str = uuid.to_string();
  //   self
  //     .handle
  //     .call("GetByUniqueIdS", Some(vec![uuid_str.into()]))?
  //     .into_dispatch()?
  //     .try_into()
  // }

  pub fn len(&self) -> Result<usize> {
    self
      .handle
      .get("Count")?
      .as_int()
      .map(|count| count as usize)
  }

  pub fn into_vec(self) -> Result<Vec<Entity>> {
    let len = self.len()?;
    let mut vec = Vec::with_capacity(len);
    for i in 0..len {
      vec.push(self.get(i)?);
    }
    Ok(vec)
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