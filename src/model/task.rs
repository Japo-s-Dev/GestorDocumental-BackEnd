use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

// region: --- Task Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Fields, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
}
// endregion : --- Task Types
