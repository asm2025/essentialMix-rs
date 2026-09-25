use emixseaorm::{ClosureFilter, DirectCondition, TFilterCondition, TFilterRelatedCondition};
use sea_orm::{ColumnTrait, Condition, DbBackend, EntityTrait, QueryFilter, QueryTrait};

mod user {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub active: bool,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::post::Entity")]
        Post,
    }

    impl Related<super::post::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Post.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

mod post {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub user_id: i32,
        pub title: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::user::Entity",
            from = "Column::UserId",
            to = "super::user::Column::Id"
        )]
        User,
    }

    impl Related<super::user::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

fn sql<Q: QueryTrait>(query: Q) -> String {
    query.build(DbBackend::Sqlite).to_string()
}

fn active() -> Condition {
    Condition::all().add(user::Column::Active.eq(true))
}

#[test]
fn test_condition_filter() {
    let query = TFilterCondition::<user::Entity>::apply(&active(), user::Entity::find());
    assert_eq!(sql(query), sql(user::Entity::find().filter(user::Column::Active.eq(true))));
}

#[test]
fn test_closure_filter() {
    let filter = ClosureFilter::new(active);
    let query = TFilterCondition::<user::Entity>::apply(&filter, user::Entity::find());
    assert!(sql(query).contains(r#"WHERE "users"."active" = TRUE"#));
}

#[test]
fn test_direct_condition_filter() {
    let filter = DirectCondition(Condition::all().add(user::Column::Name.eq("alice")));
    let query = TFilterCondition::<user::Entity>::apply(&filter, user::Entity::find());
    assert!(sql(query).contains(r#""users"."name" = 'alice'"#));
}

#[test]
fn test_function_filter() {
    let filter = |q: sea_orm::Select<user::Entity>| q.filter(user::Column::Id.gt(10));
    let query = TFilterCondition::<user::Entity>::apply(&filter, user::Entity::find());
    assert!(sql(query).contains(r#""users"."id" > 10"#));
}

#[test]
fn test_related_filters() {
    let base = || user::Entity::find().find_with_related(post::Entity);

    let query = TFilterRelatedCondition::<user::Entity, post::Entity>::apply(&active(), base());
    let rendered = sql(query);
    assert!(rendered.contains(r#"LEFT JOIN "posts""#), "{rendered}");
    assert!(rendered.contains(r#""users"."active" = TRUE"#), "{rendered}");

    let filter = DirectCondition(Condition::all().add(post::Column::Title.contains("rust")));
    let rendered = sql(TFilterRelatedCondition::<user::Entity, post::Entity>::apply(&filter, base()));
    assert!(rendered.contains(r#""posts"."title" LIKE '%rust%'"#), "{rendered}");

    let filter = ClosureFilter::new(active);
    let rendered = sql(TFilterRelatedCondition::<user::Entity, post::Entity>::apply(&filter, base()));
    assert!(rendered.contains(r#""users"."active" = TRUE"#), "{rendered}");
}
