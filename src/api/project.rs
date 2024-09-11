use crate::{
  native::Dispatch,
  Result,
  Error
};
use super::Category;

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
  use crate::*;

  #[test]
  fn test_close_project() -> anyhow::Result<()> {
    let mut app = Application::new()?;
    let mut project = app.new_project()?;

    assert!(app.project()?.is_some());

    project.close(false)?;

    assert!(app.project()?.is_none());
    assert!(project.close(true).is_err());

    Ok(())
  }

  #[test]
  fn test_transaction() -> anyhow::Result<()> {
    let mut app = Application::new()?;
    let mut project = app.new_project()?;

    assert!(app.project()?.is_some());

    let mut transaction = project.start_transaction()?;

    //assert!(project.has_transaction()?);

    transaction.commit()?;

    assert!(!project.has_transaction()?);
    std::thread::sleep(std::time::Duration::from_millis(5000));

    Ok(())
  }
}