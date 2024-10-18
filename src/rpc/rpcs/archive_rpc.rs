use crate::core::ctx::Ctx;
use crate::core::model::archive::{
	Archive, ArchiveBmc, ArchiveFilter, ArchiveForCreate, ArchiveForUpdate,
};
use crate::core::model::base::ListResult;
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;

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

pub async fn list_archives(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<ArchiveFilter>,
) -> Result<ListResult<Archive>> {
	let archives =
		ArchiveBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

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
