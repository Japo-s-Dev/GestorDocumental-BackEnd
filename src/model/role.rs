use crate::model::base::{self, DbBmc};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Role {
	pub role_id: i64,
	pub role_name: String,
}

#[derive(Clone, Fields, FromRow, Deserialize)]
pub struct RoleForInsert {
	pub role_name: String,
}

pub trait RoleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RoleBy for Role {}
impl RoleBy for RoleForInsert {}

pub struct RoleBMC;
