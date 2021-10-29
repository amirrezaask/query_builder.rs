use query_builder_macros::QueryBuilder;

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
    let mut usb = UserSelectBuilder::new();
    let q = usb.where_name_eq("a".to_string()).build();
    assert_eq!(q, "")
}
