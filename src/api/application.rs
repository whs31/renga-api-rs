use crate::{
  native::{
    Dispatch,
    runtime::ComRuntime,
    records::Version as NativeVersion
  },
  Result,
  Error
};
use super::{
  Project,
  Version
};

const CLASS_NAME: &str = "Renga.Application.1";

pub struct Application {
  handle: Dispatch,
  _com: ComRuntime
}

impl Application {
  pub fn new() -> Result<Self> {
    let _com = ComRuntime::new()?;
    let mut this = Self {
      handle: Dispatch::from_class_name(CLASS_NAME)?,
      _com
    };
    log::debug!("Renga Application initialized");
    this
      .set_enabled(true)?
      .set_visible(true)?;
    Ok(this)
  }

  pub fn new_hidden() -> Result<Self> {
    let mut this = Self {
      handle: Dispatch::from_class_name(CLASS_NAME)?,
      _com: ComRuntime::new()?
    };
    log::debug!("Renga Application initialized (hidden)");
    this
      .set_enabled(true)?
      .set_visible(false)?;
    Ok(this)
  }

  pub fn try_quit(&mut self) -> Result<()> {
    self.handle.call("Quit", None)?;
    log::debug!("Renga Application closed");
    Ok(())
  }

  pub fn quit(&mut self) {
    if let Err(error) = self.try_quit() {
      log::error!("Renga Application close failed: {error:?}");
    }
  }

  #[inline(always)]
  pub fn version(&self) -> Result<Version> {
    let var = self.handle.get("Version")?;
    let record = unsafe {
      NativeVersion::from_variant(&var)?
    };
    Ok(Version::new(record.major as u64, record.minor as u64, record.build as u64))
  }

  #[inline(always)]
  pub fn enabled(&self) -> Result<bool> {
    Ok(self.handle.get("Enabled")?.as_bool()?)
  }

  #[inline(always)]
  pub fn visible(&self) -> Result<bool> {
    Ok(self.handle.get("Visible")?.as_bool()?)
  }

  #[inline(always)]
  pub fn set_enabled(&mut self, value: bool) -> Result<&mut Self> {
    self.handle.set("Enabled", value.into())?;
    Ok(self)
  }

  #[inline(always)]
  pub fn set_visible(&mut self, value: bool) -> Result<&mut Self> {
    self.handle.set("Visible", value.into())?;
    Ok(self)
  }

  pub fn project(&mut self) -> Result<Option<Project>> {
    Ok(match self.get_project() {
      Ok(project) => project,
      Err(_) => None
    })
  }

  pub fn new_project(&mut self) -> Result<Project> {
    // check for unsaved changes etc
    let error = self.handle.call("CreateProject", None)?.as_int()?;
    if error != 0 {
      return Err(Error::InvalidOperation(format!("Failed to create new project: error code {error}")));
    }
    let project = self.get_project()?;
    if let None = project {
      return Err(Error::Internal("Failed to create new project".to_owned()));
    }
    Ok(project.unwrap())
  }

  fn has_project(&self) -> Result<bool> {
    Ok(self.handle.call("HasProject", None)?.as_bool()?)
  }

  fn get_project(&mut self) -> Result<Option<Project>> {
    let var = self.handle.get("Project")?.into_dispatch()?;
    let proj = Project::new(self.handle.clone(), var)?;
    Ok(Some(proj))
  }

  // Properties left:
  // ActiveView       [get]
  // Project          [get]
  // Selection        [get]
  // UI               [get]
  // 
  // Methods left:
  // CloseProject
  // CreateIfcExportSettings
  // CreateProjectFromTemplate
  // GetCurrentLocale
  // GetMainWindowHandle
  // ImportIfcProject
  // LastError
  // OpenProject
  // SetLastError
}

impl Drop for Application {
  fn drop(&mut self) {
    let _ = self
      .project()
      .map(|pr| {
        let _ = pr
          .map(|mut p| { let _ = p
            .close(true)
            .inspect_err(|e| log::error!("failed to close project: {e:?}")
          );
        });
    });
    self.quit();
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_open_close() -> anyhow::Result<()> {
    let mut app = Application::new()?;
    let version = app.version()?;
    let project = app.project()?;

    assert!(version > Version::parse("8.0.0")?);
    assert!(project.is_none());

    Ok(())
  }

  #[test]
  fn test_create_project() -> anyhow::Result<()> {
    let mut app = Application::new()?;
    let project = app.new_project()?;

    assert!(app.project()?.is_some());
    assert!(!project.has_unsaved_changes()?);

    Ok(())
  }
}