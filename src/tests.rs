use query_builder_macros::QueryBuilder;

struct Address(String);

#[derive(QueryBuilder)]
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    score: Option<i32>,
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

#[test]
fn where_null_and_not_null() {
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_email_null()
            .build(),
        "SELECT * FROM users WHERE email is NULL"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_email_not_null()
            .build(),
        "SELECT * FROM users WHERE email is NOT NULL"
    );
}
#[test]
fn where_cmp() {
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_age_le(19)
            .build(),
        "SELECT * FROM users WHERE age<=19"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_age_ge(19)
            .build(),
        "SELECT * FROM users WHERE age>=19"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_age_gt(19)
            .build(),
        "SELECT * FROM users WHERE age>19"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_age_lt(19)
            .build(),
        "SELECT * FROM users WHERE age<19"
    );

    assert_eq!(
        UserSelectBuilder::new()
            .table("users".to_string())
            .where_score_gt(19)
            .build(),
        "SELECT * FROM users WHERE score>19"
    )
}
