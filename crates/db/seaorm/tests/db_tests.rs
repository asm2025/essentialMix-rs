#[cfg(test)]
mod tests {
    use emixseaorm::{
        repositories::{ClosureFilter, DirectCondition, ModelWithRelated, Pagination, ResultSet},
        schema::Merge,
    };
    use sea_orm::Condition;
    use serde::{Deserialize, Serialize};

    // Simple types for testing ModelWithRelated and Merge
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestModel {
        id: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestRelatedModel {
        id: i32,
        value: String,
    }

    #[derive(Debug, Clone)]
    struct TestActiveModel {
        id: Option<i32>,
        name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct UpdateDto {
        name: Option<String>,
    }

    impl Merge<TestActiveModel> for UpdateDto {
        fn merge(&self, model: &mut TestActiveModel) -> bool {
            let mut changed = false;

            if let Some(ref name) = self.name {
                model.name = Some(name.clone());
                changed = true;
            }

            changed
        }
    }

    #[test]
    fn test_pagination_default() {
        let pagination = Pagination::default();
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.page_size, 10);
    }

    #[test]
    fn test_pagination_custom() {
        let pagination = Pagination {
            page: 2,
            page_size: 25,
        };
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.page_size, 25);
    }

    #[test]
    fn test_pagination_equality() {
        let pagination1 = Pagination {
            page: 1,
            page_size: 10,
        };
        let pagination2 = Pagination {
            page: 1,
            page_size: 10,
        };
        assert_eq!(pagination1, pagination2);
    }

    #[test]
    fn test_pagination_inequality() {
        let pagination1 = Pagination {
            page: 1,
            page_size: 10,
        };
        let pagination2 = Pagination {
            page: 2,
            page_size: 10,
        };
        assert_ne!(pagination1, pagination2);
    }

    #[test]
    fn test_pagination_serialization() {
        let pagination = Pagination {
            page: 2,
            page_size: 20,
        };
        let json = serde_json::to_string(&pagination).unwrap();
        let deserialized: Pagination = serde_json::from_str(&json).unwrap();
        assert_eq!(pagination, deserialized);
    }

    #[test]
    fn test_pagination_deserialization() {
        let json = r#"{"page":3,"page_size":15}"#;
        let pagination: Pagination = serde_json::from_str(json).unwrap();
        assert_eq!(pagination.page, 3);
        assert_eq!(pagination.page_size, 15);
    }

    #[test]
    fn test_pagination_clone() {
        let pagination = Pagination {
            page: 3,
            page_size: 15,
        };
        let cloned = pagination.clone();
        assert_eq!(pagination, cloned);
    }

    #[test]
    fn test_pagination_debug() {
        let pagination = Pagination {
            page: 1,
            page_size: 10,
        };
        let debug_str = format!("{:?}", pagination);
        assert!(debug_str.contains("Pagination"));
        assert!(debug_str.contains("page"));
        assert!(debug_str.contains("page_size"));
    }

    #[test]
    fn test_result_set_default() {
        let result_set: ResultSet<()> = ResultSet::default();
        assert_eq!(result_set.data.len(), 0);
        assert_eq!(result_set.total, 0);
        assert_eq!(result_set.pagination, None);
    }

    #[test]
    fn test_result_set_with_data() {
        let result_set = ResultSet {
            data: vec![1, 2, 3],
            total: 3,
            pagination: Some(Pagination::default()),
        };
        assert_eq!(result_set.data.len(), 3);
        assert_eq!(result_set.total, 3);
        assert!(result_set.pagination.is_some());
    }

    #[test]
    fn test_result_set_without_pagination() {
        let result_set = ResultSet {
            data: vec!["test1".to_string(), "test2".to_string()],
            total: 2,
            pagination: None,
        };
        assert_eq!(result_set.data.len(), 2);
        assert_eq!(result_set.total, 2);
        assert!(result_set.pagination.is_none());
    }

    #[test]
    fn test_result_set_serialization() {
        let result_set = ResultSet {
            data: vec!["test1".to_string(), "test2".to_string()],
            total: 2,
            pagination: Some(Pagination {
                page: 1,
                page_size: 10,
            }),
        };
        let json = serde_json::to_string(&result_set).unwrap();
        let deserialized: ResultSet<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(result_set.data, deserialized.data);
        assert_eq!(result_set.total, deserialized.total);
        assert_eq!(result_set.pagination, deserialized.pagination);
    }

    #[test]
    fn test_result_set_clone() {
        let result_set = ResultSet {
            data: vec![1, 2, 3],
            total: 3,
            pagination: Some(Pagination::default()),
        };
        let cloned = result_set.clone();
        assert_eq!(result_set.data, cloned.data);
        assert_eq!(result_set.total, cloned.total);
        assert_eq!(result_set.pagination, cloned.pagination);
    }

    #[test]
    fn test_result_set_debug() {
        let result_set: ResultSet<i32> = ResultSet {
            data: vec![1, 2, 3],
            total: 3,
            pagination: None,
        };
        let debug_str = format!("{:?}", result_set);
        assert!(debug_str.contains("ResultSet"));
        assert!(debug_str.contains("data"));
        assert!(debug_str.contains("total"));
    }

    #[test]
    fn test_model_with_related() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let related = vec![
            TestRelatedModel {
                id: 1,
                value: "Related1".to_string(),
            },
            TestRelatedModel {
                id: 2,
                value: "Related2".to_string(),
            },
        ];
        let model_with_related = ModelWithRelated {
            item: model.clone(),
            related: related.clone(),
        };
        assert_eq!(model_with_related.item, model);
        assert_eq!(model_with_related.related, related);
    }

    #[test]
    fn test_model_with_related_empty_related() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let model_with_related = ModelWithRelated {
            item: model.clone(),
            related: vec![] as Vec<TestRelatedModel>,
        };
        assert_eq!(model_with_related.item, model);
        assert_eq!(model_with_related.related.len(), 0);
    }

    #[test]
    fn test_model_with_related_serialization() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let related = vec![TestRelatedModel {
            id: 1,
            value: "Related1".to_string(),
        }];
        let model_with_related = ModelWithRelated {
            item: model.clone(),
            related: related.clone(),
        };
        let json = serde_json::to_string(&model_with_related).unwrap();
        let deserialized: ModelWithRelated<TestModel, TestRelatedModel> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.item, model);
        assert_eq!(deserialized.related, related);
    }

    #[test]
    fn test_model_with_related_clone() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let related = vec![TestRelatedModel {
            id: 1,
            value: "Related".to_string(),
        }];
        let model_with_related = ModelWithRelated {
            item: model,
            related,
        };
        let cloned = model_with_related.clone();
        assert_eq!(model_with_related.item.id, cloned.item.id);
        assert_eq!(model_with_related.item.name, cloned.item.name);
        assert_eq!(model_with_related.related.len(), cloned.related.len());
    }

    #[test]
    fn test_model_with_related_debug() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let model_with_related = ModelWithRelated {
            item: model,
            related: vec![] as Vec<TestRelatedModel>,
        };
        let debug_str = format!("{:?}", model_with_related);
        assert!(debug_str.contains("ModelWithRelated"));
        assert!(debug_str.contains("item"));
        assert!(debug_str.contains("related"));
    }

    #[test]
    fn test_model_with_related_equality() {
        let model = TestModel {
            id: 1,
            name: "Test".to_string(),
        };
        let related = vec![TestRelatedModel {
            id: 1,
            value: "Related".to_string(),
        }];
        let model_with_related1 = ModelWithRelated {
            item: model.clone(),
            related: related.clone(),
        };
        let model_with_related2 = ModelWithRelated {
            item: model,
            related,
        };
        assert_eq!(model_with_related1, model_with_related2);
    }

    #[test]
    fn test_closure_filter_new() {
        let _filter = ClosureFilter::new(|| Condition::all());
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_direct_condition_creation() {
        let condition = Condition::all();
        let _direct_condition = DirectCondition(condition);
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_merge_trait() {
        let mut active_model = TestActiveModel {
            id: Some(1),
            name: Some("Old Name".to_string()),
        };
        let update_dto = UpdateDto {
            name: Some("New Name".to_string()),
        };
        update_dto.merge(&mut active_model);
        assert_eq!(active_model.name, Some("New Name".to_string()));
    }

    #[test]
    fn test_merge_trait_partial_update() {
        let mut active_model = TestActiveModel {
            id: Some(1),
            name: Some("Original Name".to_string()),
        };
        let update_dto = UpdateDto { name: None };
        update_dto.merge(&mut active_model);
        // Should not change since name is None
        assert_eq!(active_model.name, Some("Original Name".to_string()));
    }

    #[test]
    fn test_merge_trait_multiple_fields() {
        let mut active_model = TestActiveModel {
            id: Some(1),
            name: Some("Old Name".to_string()),
        };
        let update_dto = UpdateDto {
            name: Some("New Name".to_string()),
        };
        update_dto.merge(&mut active_model);
        // ID should remain unchanged
        assert_eq!(active_model.id, Some(1));
        // Name should be updated
        assert_eq!(active_model.name, Some("New Name".to_string()));
    }

    #[test]
    fn test_merge_trait_id_preserved() {
        let mut active_model = TestActiveModel {
            id: Some(42),
            name: Some("Name".to_string()),
        };
        let update_dto = UpdateDto {
            name: Some("Updated Name".to_string()),
        };
        update_dto.merge(&mut active_model);
        // ID should be preserved
        assert_eq!(active_model.id, Some(42));
    }
}
