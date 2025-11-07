use emixdb::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestModel {
    id: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestRelatedModel {
    id: i32,
    description: String,
}

#[test]
fn test_model_with_related_creation() {
    let model = TestModel {
        id: 1,
        name: "Test".to_string(),
    };
    let related = vec![
        TestRelatedModel {
            id: 1,
            description: "Related 1".to_string(),
        },
        TestRelatedModel {
            id: 2,
            description: "Related 2".to_string(),
        },
    ];

    let model_with_related = ModelWithRelated {
        item: model.clone(),
        related: related.clone(),
    };

    assert_eq!(model_with_related.item, model);
    assert_eq!(model_with_related.related, related);
    assert_eq!(model_with_related.related.len(), 2);
}

#[test]
fn test_model_with_related_empty_relations() {
    let model = TestModel {
        id: 1,
        name: "Test".to_string(),
    };
    let model_with_related: ModelWithRelated<TestModel, TestRelatedModel> = ModelWithRelated {
        item: model.clone(),
        related: vec![],
    };

    assert_eq!(model_with_related.item, model);
    assert!(model_with_related.related.is_empty());
}

#[test]
fn test_model_with_related_serialization() {
    let model = TestModel {
        id: 1,
        name: "Test".to_string(),
    };
    let related = vec![TestRelatedModel {
        id: 1,
        description: "Related".to_string(),
    }];

    let model_with_related = ModelWithRelated {
        item: model,
        related,
    };

    let json = serde_json::to_string(&model_with_related).unwrap();
    let deserialized: ModelWithRelated<TestModel, TestRelatedModel> =
        serde_json::from_str(&json).unwrap();

    assert_eq!(model_with_related, deserialized);
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
        page: 5,
        page_size: 25,
    };
    assert_eq!(pagination.page, 5);
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
    let pagination3 = Pagination {
        page: 2,
        page_size: 10,
    };

    assert_eq!(pagination1, pagination2);
    assert_ne!(pagination1, pagination3);
}

#[test]
fn test_pagination_serialization() {
    let pagination = Pagination {
        page: 3,
        page_size: 20,
    };

    let json = serde_json::to_string(&pagination).unwrap();
    let deserialized: Pagination = serde_json::from_str(&json).unwrap();

    assert_eq!(pagination, deserialized);
}

#[test]
fn test_result_set_default() {
    let result_set: ResultSet<()> = ResultSet::default();
    assert!(result_set.data.is_empty());
    assert_eq!(result_set.total, 0);
    assert!(result_set.pagination.is_none());
}

#[test]
fn test_result_set_with_data() {
    let data = vec![
        TestModel {
            id: 1,
            name: "Model 1".to_string(),
        },
        TestModel {
            id: 2,
            name: "Model 2".to_string(),
        },
    ];

    let result_set = ResultSet {
        data: data.clone(),
        total: 2,
        pagination: None,
    };

    assert_eq!(result_set.data.len(), 2);
    assert_eq!(result_set.total, 2);
    assert!(result_set.pagination.is_none());
}

#[test]
fn test_result_set_with_pagination() {
    let data = vec![TestModel {
        id: 1,
        name: "Model 1".to_string(),
    }];

    let pagination = Pagination {
        page: 1,
        page_size: 10,
    };

    let result_set = ResultSet {
        data: data.clone(),
        total: 100,
        pagination: Some(pagination),
    };

    assert_eq!(result_set.data.len(), 1);
    assert_eq!(result_set.total, 100);
    assert!(result_set.pagination.is_some());
    assert_eq!(result_set.pagination.unwrap(), pagination);
}

#[test]
fn test_result_set_empty_with_pagination() {
    let result_set: ResultSet<TestModel> = ResultSet {
        data: vec![],
        total: 0,
        pagination: Some(Pagination::default()),
    };

    assert!(result_set.data.is_empty());
    assert_eq!(result_set.total, 0);
    assert!(result_set.pagination.is_some());
}

#[test]
fn test_result_set_serialization() {
    let data = vec![TestModel {
        id: 1,
        name: "Test".to_string(),
    }];

    let result_set = ResultSet {
        data,
        total: 1,
        pagination: Some(Pagination::default()),
    };

    let json = serde_json::to_string(&result_set).unwrap();
    let deserialized: ResultSet<TestModel> = serde_json::from_str(&json).unwrap();

    assert_eq!(result_set.data.len(), deserialized.data.len());
    assert_eq!(result_set.total, deserialized.total);
    assert_eq!(result_set.pagination, deserialized.pagination);
}

#[test]
fn test_pagination_copy() {
    let pagination1 = Pagination {
        page: 1,
        page_size: 10,
    };
    let pagination2 = pagination1; // Copy trait
    let pagination3 = pagination1; // Can use again because it implements Copy

    assert_eq!(pagination1, pagination2);
    assert_eq!(pagination1, pagination3);
}

#[test]
fn test_model_with_related_clone() {
    let model_with_related = ModelWithRelated {
        item: TestModel {
            id: 1,
            name: "Test".to_string(),
        },
        related: vec![TestRelatedModel {
            id: 1,
            description: "Related".to_string(),
        }],
    };

    let cloned = model_with_related.clone();
    assert_eq!(model_with_related, cloned);
}

#[test]
fn test_result_set_clone() {
    let result_set = ResultSet {
        data: vec![TestModel {
            id: 1,
            name: "Test".to_string(),
        }],
        total: 1,
        pagination: Some(Pagination::default()),
    };

    let cloned = result_set.clone();
    assert_eq!(result_set.data, cloned.data);
    assert_eq!(result_set.total, cloned.total);
    assert_eq!(result_set.pagination, cloned.pagination);
}

