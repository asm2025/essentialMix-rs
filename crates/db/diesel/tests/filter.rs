use emixdiesel::{ClosureFilter, TFilterQuery};

#[test]
fn test_closure_filter_applies_function() {
    let filter = ClosureFilter::new(|v: i32| v * 2);
    assert_eq!(filter.apply(21), 42);
}

#[test]
fn test_plain_closure_is_filter() {
    let filter = |v: Vec<i32>| v.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>();
    assert_eq!(TFilterQuery::apply(&filter, vec![1, 2, 3, 4]), vec![2, 4]);
}

#[test]
fn test_filters_compose_as_trait_objects() {
    let filters: Vec<Box<dyn TFilterQuery<String>>> = vec![
        Box::new(|s: String| s.trim().to_string()),
        Box::new(ClosureFilter::new(|s: String| s.to_uppercase())),
    ];
    let result = filters.iter().fold("  hello ".to_string(), |acc, f| f.apply(acc));
    assert_eq!(result, "HELLO");
}

#[cfg(feature = "sqlite")]
mod sqlite {
    use diesel::debug_query;
    use diesel::sqlite::Sqlite;
    use emixdiesel::prelude::*;
    use emixdiesel::{ClosureFilter, TFilterQuery};

    diesel::table! {
        users (id) {
            id -> Integer,
            name -> Text,
            active -> Bool,
        }
    }

    #[test]
    fn test_filter_on_diesel_query() {
        let filter = ClosureFilter::new(|q: users::BoxedQuery<'static, Sqlite>| q.filter(users::active.eq(true)));
        let query = filter.apply(users::table.into_boxed());
        let sql = debug_query::<Sqlite, _>(&query).to_string();
        assert!(sql.contains("WHERE"), "{sql}");
        assert!(sql.contains("`users`.`active` = ?"), "{sql}");
    }

    #[test]
    fn test_chained_filters_on_diesel_query() {
        let filters: Vec<Box<dyn TFilterQuery<users::BoxedQuery<'static, Sqlite>>>> = vec![
            Box::new(|q: users::BoxedQuery<'static, Sqlite>| q.filter(users::active.eq(true))),
            Box::new(|q: users::BoxedQuery<'static, Sqlite>| q.filter(users::name.like("a%"))),
        ];
        let query = filters.iter().fold(users::table.into_boxed(), |q, f| f.apply(q));
        let sql = debug_query::<Sqlite, _>(&query).to_string();
        assert!(sql.contains("`users`.`active` = ?"), "{sql}");
        assert!(sql.contains("`users`.`name` LIKE ?"), "{sql}");
    }
}
