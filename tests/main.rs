use query_builder::QueryBuilder;

struct Address(String);

#[derive(QueryBuilder)]
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    Addresses: Vec<Address>,
}

#[test]
fn make_methods() {
    User::query()
        .where_name()
        .where_age_le()
        .has_email()
        .with_addresses()
        .build()
}
