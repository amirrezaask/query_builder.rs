# query_builder
For each struct field following methods will be generated.

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
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    score: Option<i32>,
    #[join("addresses", left)]
    Addresses: Vec<Address>,
}
*join attribute*: (table_name, left|right|inner|full_outer)