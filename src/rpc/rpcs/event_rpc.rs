use crate::core::ctx::Ctx;
use crate::core::model::event::{Event, EventBmc, EventFilter};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn list_events(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<EventFilter>,
) -> Result<Vec<Event>> {
	let events =
		EventBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(events)
}
