use suika::{json::JsonValue, macros::json};

#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: String,
}

impl From<Address> for JsonValue {
    fn from(address: Address) -> Self {
        json!({
            "street" => address.street,
            "city" => address.city,
            "zip" => address.zip,
        })
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub is_student: bool,
    pub email: Option<String>,
    pub address: Option<Address>,
    pub courses: Vec<String>,
}

impl From<User> for JsonValue {
    fn from(user: User) -> Self {
        json!({
            "name" => user.name,
            "age" => user.age,
            "is_student" => user.is_student,
            "email" => user.email,
            "address" => user.address,
            "courses" => user.courses,
        })
    }
}
