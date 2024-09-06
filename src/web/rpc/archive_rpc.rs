use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::archive::{
	Archive, ArchiveBmc, ArchiveBy, ArchiveForCreate, ArchiveForUpdate,
};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn create_archive(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ArchiveForCreate>,
) -> Result<Archive> {
	let ParamsForCreate { data } = params;

	let id = ArchiveBmc::create(&ctx, &mm, data).await?;
	let archive = ArchiveBmc::get(&ctx, &mm, id).await?;

	Ok(archive)
}

pub async fn list_archives(ctx: Ctx, mm: ModelManager) -> Result<Vec<Archive>> {
	let archives = ArchiveBmc::list(&ctx, &mm).await?;

	Ok(archives)
}

pub async fn get_archive(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Archive> {
	let ParamsIded { id } = params;

	let archive = ArchiveBmc::get(&ctx, &mm, id).await?;

	Ok(archive)
}

pub async fn update_archive(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<ArchiveForUpdate>,
) -> Result<Archive> {
	let ParamsForUpdate { id, data } = params;

	ArchiveBmc::update(&ctx, &mm, id, data).await?;

	let archive = ArchiveBmc::get(&ctx, &mm, id).await?;

	Ok(archive)
}

pub async fn delete_archive(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Archive> {
	let ParamsIded { id } = params;

	let archive = ArchiveBmc::get(&ctx, &mm, id).await?;
	ArchiveBmc::delete(&ctx, &mm, id).await?;

	Ok(archive)
}
