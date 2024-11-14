use crate::core::ctx::Ctx;
use crate::core::model::archive_comment::{
	ArchiveComment, ArchiveCommentBmc, ArchiveCommentFilter, ArchiveCommentForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ArchiveCommentForOp>,
) -> Result<ArchiveComment> {
	let ParamsForCreate { data } = params;

	let id = ArchiveCommentBmc::create(&ctx, &mm, data).await?;
	let comment = ArchiveCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}

pub async fn list_comments(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<ArchiveCommentFilter>,
) -> Result<Vec<ArchiveComment>> {
	let comments =
		ArchiveCommentBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(comments)
}

pub async fn get_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<ArchiveComment> {
	let ParamsIded { id } = params;

	let comment = ArchiveCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ctx::Ctx;
    use crate::core::model::ModelManager;
    use crate::rpc::params::{ParamsForCreate, ParamsIded, ParamsList};
    use crate::core::model::archive_comment::{ArchiveComment, ArchiveCommentForOp, ArchiveCommentFilter};

    // Mock data to directly use in tests
    fn get_mock_ctx() -> Ctx {
        Ctx::root_ctx()  // Replace with actual initializer or mock if available
    }

    fn get_mock_model_manager() -> ModelManager {
        todo!()  // Replace with actual initializer or mock if available
    }

    fn get_mock_params_ided() -> ParamsIded {
        ParamsIded { id: 1 }  // Simple mock data
    }

    fn get_mock_params_list() -> ParamsList<ArchiveCommentFilter> {
		ParamsList {
			filters: None,
			list_options: None,
		}  // Simple mock data
	}

	fn get_mock_params_for_create() -> ParamsForCreate<ArchiveCommentForOp> {
		ParamsForCreate {
			data: ArchiveCommentForOp {
				archive_id: 1,
				text: "Test comment".to_string(),
			}
		}  // Simple mock data
	}

	#[tokio::test]
    async fn test_create_comment() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_for_create();

        // Assuming the create_comment function exists and operates on these mocks correctly
        let result = create_comment(ctx, mm, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_comments() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_list();

        // Assuming the list_comments function exists and operates on these mocks correctly
        let result = list_comments(ctx, mm, params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_comment() {
        let ctx = get_mock_ctx();
        let mm = get_mock_model_manager();
        let params = get_mock_params_ided();

        // Assuming the get_comment function exists and operates on these mocks correctly
        let result = get_comment(ctx, mm, params).await;
        assert!(result.is_ok());
    }
}
