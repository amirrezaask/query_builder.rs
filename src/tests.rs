use query_builder_macros::QueryBuilder;

struct Address(String);

#[derive(QueryBuilder)]
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    score: Option<i32>,
    // #[join("addresses", left)]
    // Addresses: Vec<Address>,
}
// #[test]
// fn relation() {
//     assert_eq!(
//         UserSelectBuilder::new()
//             .table("users".to_string())
//             .load_addresses("id", "id")
//             .build(),
//         "SELECT * FROM users LEFT JOIN addresses ON users.id = addresses.user_id"
//     );
// }
#[test]
fn where_eq() {
    assert_eq!(
        User::select().where_age_eq(10).build(),
        "SELECT * FROM users WHERE age=10"
    );
}

#[test]
fn where_null_and_not_null() {
    assert_eq!(
        UserSelectBuilder::new().where_email_null().build(),
        "SELECT * FROM users WHERE email is NULL"
    );
    assert_eq!(
        UserSelectBuilder::new().where_email_not_null().build(),
        "SELECT * FROM users WHERE email is NOT NULL"
    );
}
#[test]
fn where_option_inner_should_be_extracted() {
    assert_eq!(
        UserSelectBuilder::new().where_score_gt(19).build(),
        "SELECT * FROM users WHERE score>19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_score_lt(19).build(),
        "SELECT * FROM users WHERE score<19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_score_ge(19).build(),
        "SELECT * FROM users WHERE score>=19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_score_le(19).build(),
        "SELECT * FROM users WHERE score<=19"
    );
    assert_eq!(
        UserSelectBuilder::new()
            .where_email_eq(String::from("email"))
            .build(),
        "SELECT * FROM users WHERE email=email"
    );
}

#[test]
fn where_cmp() {
    assert_eq!(
        UserSelectBuilder::new().where_age_le(19).build(),
        "SELECT * FROM users WHERE age<=19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_age_ge(19).build(),
        "SELECT * FROM users WHERE age>=19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_age_gt(19).build(),
        "SELECT * FROM users WHERE age>19"
    );
    assert_eq!(
        UserSelectBuilder::new().where_age_lt(19).build(),
        "SELECT * FROM users WHERE age<19"
    );
}
