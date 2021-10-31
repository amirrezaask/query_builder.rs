# query_builder
For each struct field following methods will be generated.

### Usage
Imagine you have a struct that is representation of your database schema like this

```rust
#[derive(QueryBuilder)] // this is the call to the derive macro
struct User {
    name: String,
    age: i32,
    email: Option<String>, // It's a nullable SQL field
    score: Option<i32>, // It's a nullable SQL field
}
```

Now QueryBuilder macro generates a QueryBuilder specific to your struct which has methods for querying each field based on their type and you can chain
these calls to get a fluent API feel.
for example

```rust
UserSelectBuilder::new().where_email_null().where_age_le(18).build()


```
### All fields
- where_FIELDNAME_eq

### Numeric fields
- where_FIELDNAME_le
- where_FIELDNAME_ge
- where_FIELDNAME_lt
- where_FIELDNAME_gt

### Option fields
- where_FIELDNAME_null
- where_FIELDNAME_not_null

### Relation attributes [TBA]
```rust
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    score: Option<i32>,
    #[join("addresses", left)]
    Addresses: Vec<Address>,
}
```
*join attribute*: (table_name, left|right|inner|full_outer)