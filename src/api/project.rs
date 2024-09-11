use std::path::Path;
use crate::{
  native::Dispatch,
  Result,
  Error
};
use super::{Category, Entity};

pub struct Project {
  parent_handle: Dispatch,
  handle: Dispatch
}

impl Project {
  pub fn new(parent_handle: Dispatch, handle: Dispatch) -> Result<Self> {
    if handle.is_null() || parent_handle.is_null() {
      return Err(Error::Internal(format!("Project handle is null")));
    }
    Ok(Self { parent_handle, handle })
  }

  // https://help.rengabim.com/api/interface_i_project.html
  pub fn has_unsaved_changes(&self) -> Result<bool> {
    self
      .handle
      .call("HasUnsavedChanges", None)?
      .as_bool()
  }

  pub fn close(&mut self, discard_changes: bool) -> Result<()> {
    let error = self.parent_handle.call("CloseProject", Some(vec![discard_changes.into()]))?.as_int()?;
    if error != 0 {
      return Err(Error::InvalidOperation(format!("Failed to close project: error code {error}")));
    }
    Ok(())
  }

  pub fn start_transaction(&mut self) -> Result<ProjectTransaction> {
    if self.has_transaction()? {
      return Err(Error::InvalidOperation("Project already has an active transaction".to_owned()));
    }
    let handle = self.handle.call("CreateOperation", None)?.into_dispatch()?;
    ProjectTransaction::new(handle)
  }

  pub fn has_transaction(&self) -> Result<bool> {
    self
      .handle
      .call("HasActiveOperation", None)?
      .as_bool()
  }

  // todo: return entity instead of ()
  pub fn import_category(&mut self, category: Category, path: &Path) -> Result<Entity> {
    // if !self.has_transaction()? {
    //   return Err(Error::NoActiveTransaction);
    // }
    let category_str = category.to_sanitized_string();
    let path_str: String = path.to_string_lossy().to_string();
    let entity: Entity = self
      .handle
      .call("ImportCategoryS", Some(vec![category_str.into(), path_str.into()]))?
      .into_dispatch()?
      .try_into()?;
    Ok(entity)
  }
}

pub struct ProjectTransaction {
  handle: Dispatch
}

impl ProjectTransaction {
  pub fn new(handle: Dispatch) -> Result<Self> {
    log::trace!("starting transaction");
    if handle.is_null() {
      return Err(Error::Internal(format!("IOperation handle is null")));
    }
    let mut this = Self { handle };
    this.start()?;
    Ok(this)
  }

  pub fn commit(&mut self) -> Result<()> {
    log::trace!("committing transaction");
    self.handle.call("Apply", None)?;
    Ok(())
  }

  pub fn rollback(&mut self) -> Result<()> {
    log::trace!("rolling back transaction");
    self.handle.call("Rollback", None)?;
    Ok(())
  }

  fn start(&mut self) -> Result<()> {
    self.handle.call("Start", None)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use test_context::test_context;
  use crate::*;

  #[test]
  fn test_send_and_sync() {
    checks::send_and_sync::<Application>();
  }

  #[test_context(ProjectContext)]
  #[test]
  fn test_close_project(ctx: &mut ProjectContext) -> anyhow::Result<()> {
    assert!(ctx.app.project()?.is_some());

    ctx.project.close(false)?;

    assert!(ctx.app.project()?.is_none());
    assert!(ctx.project.close(true).is_err());

    Ok(())
  }

  #[test_context(ProjectContext)]
  #[test]
  fn test_transaction(ctx: &mut ProjectContext) -> anyhow::Result<()> {
    assert!(ctx.app.project()?.is_some());

    let mut transaction = ctx.project.start_transaction()?;

    //assert!(ctx.project.has_transaction()?);

    transaction.commit()?;

    assert!(!ctx.project.has_transaction()?);

    Ok(())
  }

  #[test_context(ProjectContext)]
  #[test]
  fn test_import_category(ctx: &mut ProjectContext) -> anyhow::Result<()> {
    let file = external_file("style_category.rst")?;
    let mut transaction = ctx.project.start_transaction()?;

    let category = ctx.project.import_category(Category::Equipment, &file)?;
    transaction.commit()?;

    assert!(ctx.project.has_unsaved_changes()?);
    assert!(category.id()? > 0);

    // todo: entity is empty here for some reason

    Ok(())
  }
}