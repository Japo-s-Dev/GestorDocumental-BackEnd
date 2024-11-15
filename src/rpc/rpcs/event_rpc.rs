use crate::core::ctx::Ctx;
use crate::core::model::event::{EventBmc, EventFilter, EventWithUsername};
use crate::core::model::ModelManager;
use crate::rpc::params::ParamsList;
use crate::rpc::Result;

pub async fn list_events(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<EventFilter>,
) -> Result<Vec<EventWithUsername>> {
	let archive_events =
		EventBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(archive_events)
}
