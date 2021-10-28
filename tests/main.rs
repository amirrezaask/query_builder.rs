use query_builder::QueryBuilder;

struct Address(String);

#[derive(QueryBuilder)]
struct User {
    name: String,
    last_name: String,
    age: i32,
    Addresses: Vec<Address>,
}

#[test]
fn run_tests() {
    assert!(true)
}
