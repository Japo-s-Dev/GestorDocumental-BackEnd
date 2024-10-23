use crate::core::ctx::Ctx;
use crate::rpc::error::{Error, Result};

pub fn check_permission(ctx: &Ctx, required_permission: i64) -> Result<()> {
	if ctx.permissions().contains(&required_permission) {
		Ok(())
	} else {
		Err(Error::NotAllowed)
	}
}
