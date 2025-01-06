use suika::json::JsonValue;

#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: String,
}

impl Into<JsonValue> for Address {
    fn into(self) -> JsonValue {
        JsonValue::Object(vec![
            ("street".to_string(), JsonValue::String(self.street)),
            ("city".to_string(), JsonValue::String(self.city)),
            ("zip".to_string(), JsonValue::String(self.zip)),
        ])
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

impl Into<JsonValue> for User {
    fn into(self) -> JsonValue {
        let mut user_map = vec![
            ("name".to_string(), JsonValue::String(self.name)),
            ("age".to_string(), JsonValue::Number(self.age as f64)),
            (
                "is_student".to_string(),
                JsonValue::Boolean(self.is_student),
            ),
        ];

        if let Some(email) = self.email {
            user_map.push(("email".to_string(), JsonValue::String(email)));
        }

        if let Some(address) = self.address {
            user_map.push(("address".to_string(), address.into()));
        }

        user_map.push((
            "courses".to_string(),
            JsonValue::Array(self.courses.into_iter().map(JsonValue::String).collect()),
        ));

        JsonValue::Object(user_map)
    }
}
