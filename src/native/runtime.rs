use std::sync::atomic::{
  AtomicBool,
  Ordering
};
use windows::Win32::System::Com::{
  CoInitialize,
  CoUninitialize
};
use crate::{
  Result,
  Error
};

static COM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ComRuntime;

impl ComRuntime {
  pub fn new() -> Result<Self> {
    Self::init()?;
    COM_INITIALIZED.store(true, Ordering::Relaxed);
    Ok(Self)
  }

  fn init() -> Result<()> {
    if COM_INITIALIZED.load(Ordering::Relaxed) {
      return Ok(());
    }
    unsafe {
      let res = CoInitialize(None);
      if res.is_err() {
        return Err(Error::ComRuntimeInitFailed(res.0));
      }
    }
    log::debug!("COM Runtime initialized");
    Ok(())
  }
}

impl Drop for ComRuntime {
  fn drop(&mut self) {
    if !COM_INITIALIZED.load(Ordering::Relaxed) {
      return;
    }
    log::trace!("uninitializing COM Runtime");
    unsafe {
      CoUninitialize();
    }
    log::debug!("COM Runtime unitialized");
  }
}