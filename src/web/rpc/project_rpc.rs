use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::project::{Project, ProjectBmc, ProjectForOp};
use crate::model::ModelManager;
use crate::web::Result;
pub async fn create_project(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ProjectForOp>,
) -> Result<Project> {
	let ParamsForCreate { data } = params;

	let id = ProjectBmc::create(&ctx, &mm, data).await?;
	let project = ProjectBmc::get(&ctx, &mm, id).await?;

	Ok(project)
}

pub async fn list_projects(ctx: Ctx, mm: ModelManager) -> Result<Vec<Project>> {
	let projects = ProjectBmc::list(&ctx, &mm).await?;

	Ok(projects)
}

pub async fn get_project(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Project> {
	let ParamsIded { id } = params;

	let project = ProjectBmc::get(&ctx, &mm, id).await?;

	Ok(project)
}

pub async fn update_project(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<ProjectForOp>,
) -> Result<Project> {
	let ParamsForUpdate { id, data } = params;

	ProjectBmc::update(&ctx, &mm, id, data).await?;

	let project = ProjectBmc::get(&ctx, &mm, id).await?;

	Ok(project)
}

pub async fn delete_project(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Project> {
	let ParamsIded { id } = params;

	let project = ProjectBmc::get(&ctx, &mm, id).await?;
	ProjectBmc::delete(&ctx, &mm, id).await?;

	Ok(project)
}