use crate::core::ctx::Ctx;
use crate::core::model::archive_event::{
	ArchiveEvent, ArchiveEventBmc, ArchiveEventFilter,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn list_archive_events(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<ArchiveEventFilter>,
) -> Result<Vec<ArchiveEvent>> {
	let archive_events =
		ArchiveEventBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(archive_events)
}
