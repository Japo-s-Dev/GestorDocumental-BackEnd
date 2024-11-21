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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ctx::Ctx;
    use crate::core::model::ModelManager;
    use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
    use crate::core::model::archive::{Archive, ArchiveForCreate, ArchiveForUpdate};

    // Mock data to directly use in tests
    fn get_mock_ctx() -> Ctx {
        Ctx::root_ctx()  // Assuming default or empty initializer is valid
    }

	fn get_mock_model_manager() -> ModelManager {
		todo!()  // Assuming default or empty initializer is valid
	}

    fn get_mock_params_ided() -> ParamsIded {
        ParamsIded { id: 1 }  // Simple mock data
    }

    fn get_mock_params_for_update() -> ParamsForUpdate<ArchiveForUpdate> {
        ParamsForUpdate {
            id: 1,
            data: ArchiveForUpdate { tag: todo!() },  // Assuming a simple default or empty initializer is valid
        }
    }

    #[tokio::test]
    async fn test_get_archive() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_ided();

        // Assuming the get_archive function exists and operates on these mocks correctly
        let result = get_archive(ctx, mm, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_archive() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_for_update();

        // Assuming the update_archive function exists and operates on these mocks correctly
        let result = update_archive(ctx, mm, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_archive() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_ided();

        // Assuming the delete_archive function exists and operates on these mocks correctly
        let result = delete_archive(ctx, mm, params).await;
        assert!(result.is_ok());
    }
}
