use emixdb::models::Merge;

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: Option<i32>,
}

#[derive(Debug)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
    age: Option<i32>,
}

impl Merge<User> for UpdateUser {
    fn merge(&self, model: &mut User) -> bool {
        let mut changed = false;

        if let Some(ref name) = self.name {
            if &model.name != name {
                model.name = name.clone();
                changed = true;
            }
        }

        if let Some(ref email) = self.email {
            if &model.email != email {
                model.email = email.clone();
                changed = true;
            }
        }

        // For Option fields, we only update if the value is explicitly Some and different
        // None in the update means "don't update this field"
        if let Some(age) = self.age {
            if model.age != Some(age) {
                model.age = Some(age);
                changed = true;
            }
        }

        changed
    }
}

#[test]
fn test_merge_all_fields() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: Some("Jane Doe".to_string()),
        email: Some("jane@example.com".to_string()),
        age: Some(25),
    };

    let changed = update.merge(&mut user);
    assert!(changed);
    assert_eq!(user.name, "Jane Doe");
    assert_eq!(user.email, "jane@example.com");
    assert_eq!(user.age, Some(25));
}

#[test]
fn test_merge_partial_fields() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: Some("Jane Doe".to_string()),
        email: None,
        age: None,
    };

    let changed = update.merge(&mut user);
    assert!(changed);
    assert_eq!(user.name, "Jane Doe");
    assert_eq!(user.email, "john@example.com"); // Unchanged
    assert_eq!(user.age, Some(30)); // Unchanged
}

#[test]
fn test_merge_no_changes() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: None,
        email: None,
        age: Some(30), // Same value
    };

    let changed = update.merge(&mut user);
    assert!(!changed);
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.age, Some(30));
}

#[test]
fn test_merge_with_same_values() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: Some("John Doe".to_string()),
        email: Some("john@example.com".to_string()),
        age: Some(30),
    };

    let changed = update.merge(&mut user);
    assert!(!changed);
}

#[test]
fn test_merge_with_all_none_no_changes() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: None,
        email: None,
        age: None, // None means don't update
    };

    let changed = update.merge(&mut user);
    assert!(!changed);
    assert_eq!(user.age, Some(30)); // Unchanged
    assert_eq!(user.name, "John Doe"); // Unchanged
    assert_eq!(user.email, "john@example.com"); // Unchanged
}

#[test]
fn test_merge_from_none_to_some() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: None,
    };

    let update = UpdateUser {
        name: None,
        email: None,
        age: Some(30),
    };

    let changed = update.merge(&mut user);
    assert!(changed);
    assert_eq!(user.age, Some(30));
}

#[test]
fn test_merge_preserves_id() {
    let mut user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
    };

    let update = UpdateUser {
        name: Some("Jane Doe".to_string()),
        email: Some("jane@example.com".to_string()),
        age: Some(25),
    };

    update.merge(&mut user);
    assert_eq!(user.id, 1); // ID should not change
}
