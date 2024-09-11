use windows::{
  core::{
    Interface, 
    GUID, 
    HSTRING, 
    PCWSTR, 
    VARIANT
  }, 
  Win32::System::{
    Com::{
      CoCreateInstance, 
      IDispatch, 
      CLSCTX_INPROC_SERVER, 
      CLSCTX_LOCAL_SERVER, 
      DISPATCH_FLAGS,
      DISPATCH_PROPERTYGET, 
      DISPATCH_PROPERTYPUT, 
      DISPATCH_METHOD,
      DISPPARAMS
    }, 
    Ole::DISPID_PROPERTYPUT
  }
};
use crate::Result;
use super::{
  constants, 
  ClassID, 
  Variant
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispatch(IDispatch);

impl From<IDispatch> for Dispatch { fn from(value: IDispatch) -> Self { Self(value) } }
impl From<&IDispatch> for Dispatch { fn from(value: &IDispatch) -> Self { Self(value.clone()) } }
impl TryFrom<Variant> for Dispatch {
  type Error = crate::error::Error;

  fn try_from(from: Variant) -> Result<Self> {
    Ok(from.into_dispatch()?)
  }
}

impl Dispatch {
  pub fn as_raw(&self) -> &IDispatch { &self.0 }
  pub fn as_raw_mut(&mut self) -> &mut IDispatch { &mut self.0 }
  pub fn into_raw(self) -> IDispatch { self.0 }

  pub fn from_class_name(class_name: &str) -> Result<Self> {
    let class_id = ClassID::new(class_name)?;
    Self::from_class_id(&class_id)
  }

  pub fn from_class_id(class_id: &ClassID) -> Result<Self> {
    log::trace!("dispatch: loading class {class_id}");
    let instance = unsafe {
      CoCreateInstance(class_id.as_guid(), None, CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER)?
    };
    Ok(Self(instance))
  }

  pub fn is_null(&self) -> bool {
    self.0.as_raw() as *const _ == std::ptr::null()
  }

  pub fn get(&self, name: &str) -> Result<Variant> {
    self.invoke(DISPATCH_PROPERTYGET, name, vec![])
  }

  pub fn set(&self, name: &str, value: Variant) -> Result<()> {
    self.invoke(DISPATCH_PROPERTYPUT, name, vec![value])?;
    Ok(())
  }

  pub fn call(&self, name: &str, args: Option<Vec<Variant>>) -> Result<Variant> {
    let args = args.unwrap_or_default();
    self.invoke(DISPATCH_METHOD, name, args)
  }

  fn invoke(&self, flags: DISPATCH_FLAGS, name: &str, mut args: Vec<Variant>) -> Result<Variant> {
    unsafe {
      let mut disp_id = 0;
      self
        .0
        .GetIDsOfNames(
          &GUID::default(), 
          &PCWSTR::from_raw(HSTRING::from(name).as_ptr()),
          1, 
          constants::LOCALE_USER_DEFAULT, 
          &mut disp_id
        )?;
      let mut dp = DISPPARAMS::default();
      let mut disp_id_named = DISPID_PROPERTYPUT;
      if !args.is_empty() {
        args.reverse();
        dp.cArgs = args.len() as u32;
        dp.rgvarg = args.as_mut_ptr() as *mut VARIANT;
        if flags & DISPATCH_PROPERTYPUT != DISPATCH_FLAGS(0) {
          dp.cNamedArgs = 1;
          dp.rgdispidNamedArgs = &mut disp_id_named;
        }
      }
      let mut result = VARIANT::default();
      self
        .0
        .Invoke(
          disp_id, 
          &GUID::default(), 
          constants::LOCALE_SYSTEM_DEFAULT, 
          flags, 
          &dp, 
          Some(&mut result), 
          None, 
          None
        )?;
      Ok(Variant::from(result))
    }
  }
}

unsafe impl Send for Dispatch {}
unsafe impl Sync for Dispatch {}