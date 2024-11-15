use modql::filter::ListOptions;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, OneOrMany};

use crate::core::model::search_operations::Listoptions;

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	pub id: i64,
	pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	pub id: i64,
}

#[derive(Deserialize)]
pub struct IdList {
	pub ids: Vec<i64>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct ParamsList<F>
where
	F: DeserializeOwned,
{
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	pub filters: Option<Vec<F>>,
	pub list_options: Option<ListOptions>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct Paramslist<F>
where
	F: DeserializeOwned,
{
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	pub filters: Option<Vec<F>>,
	pub list_options: Option<Listoptions>,
}
