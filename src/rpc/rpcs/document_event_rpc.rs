use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::document_event::{
	DocumentEvent, DocumentEventBmc, DocumentEventFilter,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn list_document_events(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<DocumentEventFilter>,
) -> Result<ListResult<DocumentEvent>> {
	let archive_events =
		DocumentEventBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(archive_events)
}
