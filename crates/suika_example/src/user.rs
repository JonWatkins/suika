use suika::json::JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: String,
}

impl From<Address> for JsonValue {
    fn from(address: Address) -> Self {
        let mut map = HashMap::new();
        map.insert("street".to_string(), JsonValue::String(address.street));
        map.insert("city".to_string(), JsonValue::String(address.city));
        map.insert("zip".to_string(), JsonValue::String(address.zip));
        JsonValue::Object(map.into_iter().collect())
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
        let mut map = HashMap::new();
        map.insert("name".to_string(), JsonValue::String(user.name));
        map.insert("age".to_string(), JsonValue::Number(user.age as f64));
        map.insert("is_student".to_string(), JsonValue::Boolean(user.is_student));

        if let Some(email) = user.email {
            map.insert("email".to_string(), JsonValue::String(email));
        }

        if let Some(address) = user.address {
            map.insert("address".to_string(), JsonValue::from(address));
        }

        map.insert(
            "courses".to_string(),
            JsonValue::Array(user.courses.into_iter().map(JsonValue::String).collect()),
        );

        JsonValue::Object(map.into_iter().collect())
    }
}
