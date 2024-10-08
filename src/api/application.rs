use std::{
  fmt::Debug, 
  os::raw::c_void, 
  path::Path
};
use crate::{
  native::{
    Dispatch,
    runtime::ComRuntime,
    records::Version as NativeVersion
  },
  win::HWND,
  Result,
  Error
};
use super::{
  Project,
  Version
};

const CLASS_NAME: &str = "Renga.Application.1";

/// Represents entire Renga application.
/// 
/// Currently, you can obtain instance of this struct only by using [Application::new] or [Application::new_hidden] methods.
/// Accessing running instances of Renga is still in development.
/// 
/// See [Official documentation](https://help.rengabim.com/api/interface_i_application.html)
pub struct Application {
  locale: String,
  handle: Dispatch,
  _com: ComRuntime
}

impl Application {
  /// Creates new instance of Renga application[^note].
  /// 
  /// See [Application::new_hidden]
  /// 
  /// [^note]: Renga must be registered in Windows registry for this method to work.
  /// You can do it by launching powershell as administrator in Renga folder and running the following command:
  /// ```powershell
  /// ./RengaProfessional.exe /regserver
  /// ```
  /// If your Renga distribution comes from official installer, you can skip this step - Renga will be automatically registered.
  pub fn new() -> Result<Self> {
    let mut this = Self::init()?;
    log::debug!("Renga Application initialized");
    this
      .set_enabled(true)?
      .set_visible(true)?;
    Ok(this)
  }

  /// Creates new headless instance of Renga application.
  /// 
  /// The ability to launch Renga without a GUI is advantageous for testing purposes. Actually, this crate using this feature for self-testing!
  /// 
  /// See [Application::new]
  pub fn new_hidden() -> Result<Self> {
    let mut this = Self::init()?;
    log::debug!("Renga Application initialized (hidden)");
    this
      .set_enabled(true)?
      .set_visible(false)?;
    Ok(this)
  }

  /// Tries to close Renga application.
  ///
  /// See [Application::quit]
  pub fn try_quit(&mut self) -> Result<()> {
    self.handle.call("Quit", None)?;
    log::debug!("Renga Application closed");
    Ok(())
  }

  /// Closes Renga application.
  /// 
  /// This method will not fail. If any error occurs, it will be logged.
  /// 
  /// See [Application::try_quit]
  pub fn quit(&mut self) {
    if let Err(error) = self.try_quit() {
      log::error!("Renga Application close failed: {error:?}");
    }
  }

  /// Returns semantic version of Renga application.
  #[inline]
  pub fn version(&self) -> Result<Version> {
    let var = self.handle.get("Version")?;
    let record = unsafe {
      NativeVersion::from_variant(&var)?
    };
    Ok(Version::new(record.major as u64, record.minor as u64, record.build as u64))
  }

  /// Returns `true`, if user input is enabled in this instance. 
  /// 
  /// See [Application::set_enabled]
  #[inline]
  pub fn enabled(&self) -> Result<bool> {
    Ok(self.handle.get("Enabled")?.as_bool()?)
  }

  /// Returns `true`, if user interface is visible in this instance. 
  /// 
  /// See [Application::set_visible]
  #[inline]
  pub fn visible(&self) -> Result<bool> {
    Ok(self.handle.get("Visible")?.as_bool()?)
  }

  /// Sets user input to be enabled or disabled in this instance. 
  /// 
  /// See [Application::enabled]
  #[inline]
  pub fn set_enabled(&mut self, value: bool) -> Result<&mut Self> {
    self.handle.set("Enabled", value.into())?;
    Ok(self)
  }

  /// Sets user interface to be visible or hidden in this instance.
  /// 
  /// See [Application::visible]
  #[inline]
  pub fn set_visible(&mut self, value: bool) -> Result<&mut Self> {
    self.handle.set("Visible", value.into())?;
    Ok(self)
  }

  /// Returns currently active project.
  /// 
  /// Can be `None` if there is no active project opened.
  /// 
  /// You can create new project by calling [Application::new_project] or open 
  /// existing project by calling [Application::open_project].
  pub fn project(&mut self) -> Result<Option<Project>> {
    Ok(match self.get_project() {
      Ok(project) => project,
      Err(_) => None
    })
  }

  /// Creates new project.
  /// 
  /// Can return error in the following cases:
  /// - [crate::Error::AlreadyOpened]: Project is already opened;
  /// - [crate::Error::Internal]: Internal error happened;
  /// 
  /// See [Application::open_project]
  pub fn new_project(&mut self) -> Result<Project> {
    // check for unsaved changes etc
    let error = self.handle.call("CreateProject", None)?.as_int()?;
    if error != 0 {
      return Err(Error::AlreadyOpened(format!("Failed to create new project: error code {error}")));
    }
    let project = self.get_project()?;
    if let None = project {
      return Err(Error::Internal("Failed to create new project".to_owned()));
    }
    Ok(project.unwrap())
  }

  /// Opens existing project from given path.
  /// 
  /// Can return error in the following cases:
  /// - [crate::Error::NonexistentPath]: Project path is invalid;
  /// - [crate::Error::AlreadyOpened]: Project is already opened;
  /// - [crate::Error::Internal]: Internal error happened;
  /// 
  /// See [Application::new_project]
  pub fn open_project(&mut self, path: &Path) -> Result<Project> {
    if !path.exists() {
      return Err(Error::NonexistentPath(format!("Project path does not exist: {}", path.display())));
    }
    let path = path.to_string_lossy().into_owned();
    let error = self.handle.call("OpenProject", Some(vec![path.into()]))?.as_int()?;
    if error != 0 {
      return Err(Error::AlreadyOpened(format!("Failed to open project: error code {error}")));
    }
    let project = self.get_project()?;
    if let None = project {
      return Err(Error::Internal("Failed to open project".to_owned()));
    }
    Ok(project.unwrap())
  }

  /// Returns native handle of the main window.
  pub fn native_window_handle(&self) -> Result<HWND> {
    let intptr = self.handle.call("GetMainWindowHandle", None)?.as_int()?;
    if intptr == 0 {
      return Err(Error::Internal("Failed to get native window handle".to_owned()));
    }
    let ptr = intptr as *mut c_void;
    Ok(HWND(ptr))
  }

  // fn has_project(&self) -> Result<bool> {
  //   Ok(self.handle.call("HasProject", None)?.as_bool()?)
  // }

  fn init() -> Result<Self> {
    let _com = ComRuntime::new()?;
    let handle = Dispatch::from_class_name(CLASS_NAME)?;
    Ok(Self {
      locale: handle
        .call("GetCurrentLocale", None)
        .unwrap_or("C".to_owned().into())
        .into_string()
        .unwrap_or("C".to_owned()),
      handle,
      _com
    })
  }

  fn get_project(&mut self) -> Result<Option<Project>> {
    let var = self.handle.get("Project")?.into_dispatch()?;
    let proj = Project::new(self.handle.clone(), var)?;
    Ok(Some(proj))
  }

  // Properties left:
  // ActiveView       [get]
  // Selection        [get]
  // UI               [get]
  // 
  // Methods left:
  // CreateIfcExportSettings
  // CreateProjectFromTemplate
  // ImportIfcProject
  // LastError
  // SetLastError
}

impl Debug for Application {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Renga Application {{ locale: {} }}", self.locale)
  }
}

/// Drop implementation for Application. 
/// 
/// - Closes project if it exists, discarding any changes.
/// - Closes Renga application.
/// 
/// This function never fails or panics. If any error occurs, it will be logged.
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
  use test_context::test_context;
  use crate::*;

  #[test]
  fn test_send_and_sync() {
    checks::send_and_sync::<Application>();
  }

  #[test]
  fn test_consecutive_applications() -> anyhow::Result<()> {
    let app1 = Application::new_hidden()?;
    drop(app1);
    let _ = Application::new_hidden()?;

    Ok(())
  }
    
  #[test_context(RengaContext)]
  #[test]
  fn test_open_close(ctx: &mut RengaContext) -> anyhow::Result<()> {
    let version = ctx.app.version()?;
    let project = ctx.app.project()?;

    assert!(version > Version::parse("8.0.0")?);
    assert!(project.is_none());

    Ok(())
  }

  #[test_context(RengaContext)]
  #[test]
  fn test_create_project(ctx: &mut RengaContext) -> anyhow::Result<()> {
    let project = ctx.app.new_project()?;

    assert!(ctx.app.project()?.is_some());
    assert!(!project.has_unsaved_changes()?);

    Ok(())
  }
}