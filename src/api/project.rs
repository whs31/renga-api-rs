use std::path::{
  Path, 
  PathBuf
};
use crate::{
  native::Dispatch,
  Result,
  Error
};
use super::{
  Category, 
  Entity, 
  EntityCollection
};

/// Represents active Renga project.
/// 
/// Provides methods to work with a Renga project. Can be obtained using [crate::Application::project] method.
/// 
/// By default, project is immutable. To achieve mutability, you must follow this steps:
/// - Create new transaction using [Project::start_transaction] method. Store it in a mutable variable.
/// - Do something with the project.
/// - Commit the transaction using [ProjectTransaction::commit] or discard it using [ProjectTransaction::rollback] methods.
/// 
/// For example:
/// ```no_run
/// use renga_api_rs as renga;
/// let mut app = renga::Application::new().unwrap();
/// let mut project = app.new_project().unwrap();
/// let mut transaction = project.start_transaction().unwrap();
/// 
/// // do something with the project
/// transaction.commit().unwrap();
/// ```
/// 
/// You can safely clone this struct and use it in multiple threads.
/// 
/// See [Official documentation](https://help.rengabim.com/api/interface_i_project.html)
pub struct Project {
  parent_handle: Dispatch,
  handle: Dispatch
}

impl Project {
  /// Creates new instance of Project from native handles.
  /// 
  /// To get an instance of this structure without using native handles, use 
  /// the [crate::Application::new_project] or [crate::Application::project] methods.
  pub fn new(parent_handle: Dispatch, handle: Dispatch) -> Result<Self> {
    if handle.is_null() || parent_handle.is_null() {
      return Err(Error::Internal(format!("Project handle is null")));
    }
    Ok(Self { parent_handle, handle })
  }

  /// Returns path to project save file or `None` if project was never saved.
  /// 
  /// See [Project::save]
  #[inline]
  pub fn path(&self) -> Result<Option<PathBuf>> {
    let path = self
      .handle
      .get("FilePath")?
      .into_string()?;
    match path.is_empty() {
      true => Ok(None),
      false => Ok(Some(Path::new(&path).to_path_buf())),
    }
  }

  /// Saves project and returns path to saved file.
  pub fn save(&self) -> Result<PathBuf> {
    let error = self
      .handle
      .call("Save", None)?
      .as_int()?;
    if error != 0 {
      return Err(Error::Internal(format!("Failed to save project: error code {error}")));
    }
    match self.path()? {
      Some(path) => Ok(path),
      None => Err(Error::Internal("Failed to get project path".to_owned())),
    }
  }

  // pub fn save_as(&self, path: &Path) -> Result<&Self> {
  //   let str = path.to_string_lossy().to_string();
  //   let error = self
  //     .handle
  //     .call("SaveAs", Some(vec![str.into()]))?
  //     .as_int()?;
  //   if error != 0 {
  //     return Err(Error::Internal(format!("Failed to save project: error code {error}")));
  //   }
  //   Ok(self)
  // }

  /// Returns `true`` if project has unsaved changes.
  #[inline]
  pub fn has_unsaved_changes(&self) -> Result<bool> {
    self
      .handle
      .call("HasUnsavedChanges", None)?
      .as_bool()
  }

  /// Closes project.
  /// 
  /// If project has unsaved changes, you can discard them using `discard_changes` parameter.
  pub fn close(&mut self, discard_changes: bool) -> Result<()> {
    let error = self.parent_handle.call("CloseProject", Some(vec![discard_changes.into()]))?.as_int()?;
    if error != 0 {
      return Err(Error::InvalidOperation(format!("Failed to close project: error code {error}")));
    }
    Ok(())
  }

  /// Creates new transaction.
  /// 
  /// See [ProjectTransaction] for more information.
  pub fn start_transaction(&mut self) -> Result<ProjectTransaction> {
    if self.has_transaction()? {
      return Err(Error::InvalidOperation("Project already has an active transaction".to_owned()));
    }
    let handle = self.handle.call("CreateOperation", None)?.into_dispatch()?;
    ProjectTransaction::new(handle)
  }

  /// Returns `true` if project has an active transaction.
  pub fn has_transaction(&self) -> Result<bool> {
    self
      .handle
      .call("HasActiveOperation", None)?
      .as_bool()
  }

  /// Returns collection of entities of given category.
  /// 
  /// You can use this method to access entities inside categories.
  /// For example, to get style template with name matching string `Pump`:
  /// ```no_run
  /// use renga_api_rs as renga;
  /// use anyhow::Result;
  /// 
  /// fn style_template() -> Result<renga::Entity> {
  ///   let mut app = renga::Application::new()?;
  ///   let mut project = app.new_project()?;
  ///   let pump_style_template = project
  ///     .category(renga::Category::Equipment)?
  ///     .into_vec()?
  ///     .into_iter()
  ///     .find(|entity| entity.name == "Pump")
  ///     .unwrap();
  ///   Ok(pump_style_template)
  /// }
  /// 
  /// let pump_style_template = style_template().unwrap();
  /// assert_eq!(
  ///   pump_style_template.name, 
  ///   "Pump"
  /// );
  /// ```
  pub fn category(&self, category: Category) -> Result<EntityCollection> {
    Ok(self
      .handle
      .get(format!("{category:?}Categories").as_str())?
      .into_dispatch()?
      .try_into()?)
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

  // CreateOperationWithUndo ([in] GUID modelId, [out, retval] IOperation **ppOperation)
  // ExportDrawingsToOpenXpsS ([in] SAFEARRAY(BSTR) drawingIds, [in] BSTR filePath, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult) 
  // ExportDrawingsToPdf ([in] SAFEARRAY(GUID) drawingIds, [in] BSTR filePath, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult)
  // ExportDrawingsToPdfS ([in] SAFEARRAY(BSTR) drawingIds, [in] BSTR filePath, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult)
  // ExportToCsv ([in] BSTR folderPath, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult)
  // ExportToIfc ([in] BSTR filePath, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult)
  // ExportToIfc2 ([in] BSTR filePath, [in] VARIANT_BOOL overwrite, [in] IIfcExportSettings *pIfcExportSettings, [out, retval] int *pResult)
  // GetEntityNumberInTopic ([in] GUID entityId, [out, retval] int *pResult)
  // GetEntityNumberInTopicS ([in] BSTR entityId, [out, retval] int *pResult)
  // GetUndoStack ([in] GUID modelId, [out, retval] IUndoStack **ppUndoStack)
  // SaveAs ([in] BSTR filePath, [in] enum ProjectType projectType, [in] VARIANT_BOOL overwrite, [out, retval] int *pResult)

  // IEntityCollection 	Assemblies [get]
  // IBeamStyleManager 	BeamStyleManager [get]
  // IEntityCollection 	BeamStyles [get]
  // IBuildingInfo 	BuildingInfo [get]
  // IColumnStyleManager 	ColumnStyleManager [get]
  // IEntityCollection 	ColumnStyles [get]
  // IDataExporter 	DataExporter [get]
  // IEntityCollection 	DoorStyles [get]
  // IDrawingCollection 	Drawings [get]
  // IEntityCollection 	Drawings2 [get]
  // IEntityCollection 	DuctAccessoryStyles [get]
  // IEntityCollection 	DuctFittingStyles [get]
  // IEntityCollection 	DuctStyles [get]
  // IEntityCollection 	ElectricalCircuitLineStyles [get]
  // IEntityCollection 	ElectricalConductorStyles [get]
  // IEntityCollection 	ElectricDistributionBoardStyles [get]
  // IEntityCollection 	ElementStyles [get]
  // IEquipmentStyleManager 	EquipmentStyleManager [get]
  // IEntityCollection 	EquipmentStyles [get]
  // IEntityCollection 	HoleStyles [get]
  // BSTR 	JournalPath [get]
  // ILandPlotInfo 	LandPlotInfo [get]
  // ILayeredMaterialManager 	LayeredMaterialManager [get]
  // IEntityCollection 	LayeredMaterials [get]
  // IEntityCollection 	LayoutStyles [get]
  // IEntityCollection 	LightingFixtureStyles [get]
  // IMaterialManager 	MaterialManager [get]
  // IEntityCollection 	Materials [get]
  // IEntityCollection 	MechanicalEquipmentStyles [get]
  // IModel 	Model [get]
  // IEntityCollection 	PageFormatStyles [get]
  // IEntityCollection 	PipeAccessoryStyles [get]
  // IEntityCollection 	PipeFittingStyles [get]
  // IEntityCollection 	PipeStyles [get]
  // IEntityCollection 	PlateStyles [get]
  // IPlumbingFixtureStyleManager 	PlumbingFixtureStyleManager [get]
  // IEntityCollection 	PlumbingFixtureStyles [get]
  // IProfileDescriptionManager 	ProfileDescriptionManager [get]
  // IEntityCollection 	Profiles [get]
  // IProjectInfo 	ProjectInfo [get]
  // ProjectType 	ProjectType [get]
  // IPropertyManager 	PropertyManager [get]
  // IEntityCollection 	RebarStyles [get]
  // IEntityCollection 	ReinforcementGrades [get]
  // IEntityCollection 	ReinforcementStyles [get]
  // IReinforcementUnitStyleManager 	ReinforcementUnitStyleManager [get]
  // IEntityCollection 	ReinforcementUnitStyles [get]
  // ISystemStyleManager 	SystemStyleManager [get]
  // IEntityCollection 	SystemStyles [get]
  // IEntityCollection 	Topics [get]
  // IEntityCollection 	WindowStyles [get]
  // IEntityCollection 	WiringAccessoryStyles [get]
}

/// Represents project transaction, created by [Project::start_transaction].
pub struct ProjectTransaction {
  handle: Dispatch
}

impl ProjectTransaction {
  /// Creates new instance of ProjectTransaction from native handle.
  /// 
  /// To get an instance of this structure without using native handles, use
  /// the [Project::start_transaction] method.
  pub fn new(handle: Dispatch) -> Result<Self> {
    log::trace!("starting transaction");
    if handle.is_null() {
      return Err(Error::Internal(format!("IOperation handle is null")));
    }
    let mut this = Self { handle };
    this.start()?;
    Ok(this)
  }

  /// Commits changes made in transaction to the project.
  pub fn commit(&mut self) -> Result<()> {
    log::trace!("committing transaction");
    self.handle.call("Apply", None)?;
    Ok(())
  }

  /// Rolls back changes made in transaction.
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
  use api::UUID;
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
    assert!(category.id > 0);
    assert!(category.type_id != UUID::default());
    assert!(category.unique_id != UUID::default());

    let entity = ctx
      .project
      .category(Category::Equipment)?
      .into_vec()?
      .iter()
      .find(|e| e.name == "asd 1")
      .unwrap()
      .clone();

    assert_eq!(entity.name, "asd 1".to_owned());
    assert_eq!(entity.unique_id, category.unique_id);

    Ok(())
  }
}