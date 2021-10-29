use query_builder_macros::QueryBuilder;

struct Address(String);

#[derive(QueryBuilder)]
struct User {
    name: String,
    age: i32,
    // email: Option<String>,
    // Addresses: Vec<Address>,
}

#[test]
fn where_eq() {
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_name_eq("a".to_string())
            .build(),
        "SELECT * FROM users WHERE name=a"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_age_eq(10)
            .build(),
        "SELECT * FROM users WHERE age=10"
    );
}
