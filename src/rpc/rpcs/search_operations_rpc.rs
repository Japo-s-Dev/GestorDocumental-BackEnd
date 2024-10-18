use std::collections::HashMap;

use crate::core::ctx::Ctx;
use crate::core::model::archive::Archive;
use crate::core::model::archive_event::ArchiveEventFilter;
use crate::core::model::base::ListResult;
use crate::core::model::document::{Document, DocumentBmc};
use crate::core::model::index::IndexFilter;
use crate::core::model::search_operations::{
	ArchiveIndexFilter, EventWithUsername, IndexWithDatatype, SearchBmc,
};
use crate::core::model::separator::{Separator, SeparatorBmc};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsIded, ParamsList, Paramslist};
use crate::rpc::Result;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Serialize;

#[derive(Serialize)]
pub struct Node {
	name: String,
	id: Option<i64>,
	children: Vec<Node>,
	documents: Vec<Document>,
	r#type: String,
}

pub async fn get_project_fields(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<IndexFilter>,
) -> Result<Vec<IndexWithDatatype>> {
	let indexes = SearchBmc::get_indexes_with_filters(
		&ctx,
		&mm,
		params.filters,
		params.list_options,
	)
	.await?;

	Ok(indexes)
}

pub async fn search_archives(
	ctx: Ctx,
	mm: ModelManager,
	params: Paramslist<ArchiveIndexFilter>,
) -> Result<ListResult<Archive>> {
	let archives =
		SearchBmc::search_archives(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(archives)
}

pub async fn get_events_with_filters(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<ArchiveEventFilter>,
) -> Result<Vec<EventWithUsername>> {
	let events = SearchBmc::get_events_with_filters(
		&ctx,
		&mm,
		params.filters,
		params.list_options,
	)
	.await?;

	Ok(events)
}

pub fn build_tree(
	parent_name: String,
	parent_id: Option<i64>,
	separators_by_parent: HashMap<Option<i64>, Vec<Separator>>, // Clonamos aquí
	documents_by_separator: HashMap<Option<i64>, Vec<Document>>, // Clonamos aquí
) -> BoxFuture<'static, Result<Node>> {
	async move {
		let mut children_nodes = Vec::new();
		let mut document_nodes = Vec::new();

		// Procesar separadores
		if let Some(separators) = separators_by_parent.get(&parent_id) {
			for sep in separators {
				let child_node = build_tree(
					sep.name.clone(),
					Some(sep.id),
					separators_by_parent.clone(), // Clonamos aquí para evitar problemas de vida útil
					documents_by_separator.clone(), // Clonamos aquí también
				)
				.await?;

				children_nodes.push(child_node);
			}
		}

		// Procesar documentos
		if let Some(docs) = documents_by_separator.get(&parent_id) {
			for doc in docs {
				document_nodes.push(doc.clone());
			}
		}

		Ok(Node {
			id: parent_id, // Aquí asignamos el id del separador actual
			name: parent_name,
			children: children_nodes,
			documents: document_nodes,
			r#type: "folder".to_string(),
		})
	}
	.boxed()
}

pub async fn get_file_tree(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Node> {
	let ParamsIded { id } = params;

	let documents =
		DocumentBmc::get_documents_by_archive::<Document>(&ctx, &mm, id).await?;
	let separators =
		SeparatorBmc::get_separators_by_archive::<Separator>(&ctx, &mm, id).await?;

	let mut documents_by_separator: HashMap<Option<i64>, Vec<Document>> =
		HashMap::new();
	for doc in documents {
		documents_by_separator
			.entry(Some(doc.separator_id))
			.or_insert_with(Vec::new)
			.push(doc);
	}

	let mut separators_by_parent: HashMap<Option<i64>, Vec<Separator>> =
		HashMap::new();
	for sep in separators {
		separators_by_parent
			.entry(sep.parent_id)
			.or_insert_with(Vec::new)
			.push(sep);
	}

	let root = build_tree(
		"root".to_string(),
		None,
		separators_by_parent,
		documents_by_separator,
	)
	.await?;

	Ok(root)
}
