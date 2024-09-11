use std::path::PathBuf;

use test_context::TestContext;
use crate::{
  Application, 
  Project
};

pub struct RengaContext {
  pub app: Application
}

pub struct ProjectContext {
  pub app: Application,
  pub project: Project
}

pub fn external_file(name: &str) -> anyhow::Result<PathBuf> {
  let root = env!("CARGO_MANIFEST_DIR");
  let path = PathBuf::from(root)
    .join("tests")
    .join("data")
    .join(name);
  match path.exists() {
    true => Ok(path),
    false => anyhow::bail!("file not found: {}", path.display()),
  }
}

impl TestContext for RengaContext {
  fn setup() -> Self {
    let app = Application::new().unwrap();
    Self { app }
  }
}

impl TestContext for ProjectContext {
  fn setup() -> Self {
    let mut app = Application::new().unwrap();
    let project = app.new_project().unwrap();
    Self { app, project }
  }
}