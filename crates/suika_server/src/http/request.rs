use suika_json::{JsonValue, parse_json};
use suika_utils::parse_query_string;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<String>,
    json_body: Option<JsonValue>,
    form_data: Option<HashMap<String, String>>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn new(request_string: &str) -> Result<Request> {
        let mut parts = request_string.split("\r\n");

        let request_line = parts.next().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                "Invalid request: Missing request line",
            )
        })?;

        let request_line_parts: Vec<&str> = request_line.split_whitespace().collect();
        if request_line_parts.len() != 3 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid request: Request line must contain method, path, and HTTP version",
            ));
        }
        let method = request_line_parts[0].to_string();
        let path_with_query = request_line_parts[1].to_string();

        let (path, query_params) = if let Some(query_start) = path_with_query.find('?') {
            let path = path_with_query[..query_start].to_string();
            let query_string = &path_with_query[query_start + 1..];
            (path, parse_query_string(query_string))
        } else {
            (path_with_query, HashMap::new())
        };

        let mut headers = HashMap::new();
        let mut is_body = false;
        let mut body_content = String::new();

        for line in parts {
            if line.is_empty() {
                is_body = true;
                continue;
            }

            if is_body {
                body_content.push_str(line);
                body_content.push_str("\r\n");
            } else {
                if let Some(colon_index) = line.find(':') {
                    let key = line[0..colon_index].trim().to_string();
                    let value = line[colon_index + 1..].trim().to_string();
                    headers.insert(key, value);
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid request: Malformed header",
                    ));
                }
            }
        }

        if body_content.ends_with("\r\n") {
            body_content.truncate(body_content.len() - 2);
        }

        let json_body = if headers
            .get("Content-Type")
            .map(|v| v == "application/json")
            .unwrap_or(false)
        {
            match parse_json(&body_content) {
                Ok(json) => Some(json),
                Err(_) => None,
            }
        } else {
            None
        };

        let form_data = if headers
            .get("Content-Type")
            .map(|v| v == "application/x-www-form-urlencoded")
            .unwrap_or(false)
        {
            let mut form_data = parse_query_string(&body_content);
            form_data.iter_mut().for_each(|(_, v)| {
                *v = v.trim_end_matches('\0').to_string();
            });
            Some(form_data)
        } else {
            None
        };

        Ok(Request {
            method,
            path,
            headers,
            query_params,
            body: if !body_content.is_empty() {
                Some(body_content)
            } else {
                None
            },
            json_body,
            form_data,
            params: HashMap::new(),
        })
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    pub fn query_param(&self, key: &str) -> Option<&str> {
        self.query_params.get(key).map(|s| s.as_str())
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn set_json_body(&mut self, json: JsonValue) {
        self.json_body = Some(json);
    }

    pub fn json_body(&self) -> Option<&JsonValue> {
        self.json_body.as_ref()
    }

    pub fn form_data(&self) -> Option<&HashMap<String, String>> {
        self.form_data.as_ref()
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_new() {
        let request_string = "GET /path?name=John HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::new(request_string).unwrap();
        assert_eq!(request.method(), "GET");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.query_param("name"), Some("John"));
        assert_eq!(request.header("Host"), Some("example.com"));
        assert_eq!(request.body(), None);
    }

    #[test]
    fn test_request_with_body() {
        let request_string = "POST /path HTTP/1.1\r\nHost: example.com\r\n\r\nbody content";
        let request = Request::new(request_string).unwrap();
        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.body(), Some("body content"));
    }

    #[test]
    fn test_request_with_empty_body() {
        let request_string = "POST /path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::new(request_string).unwrap();
        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.body(), None);
    }

    #[test]
    fn test_request_with_large_body() {
        let large_body = "a".repeat(1024);
        let request_string = format!(
            "POST /path HTTP/1.1\r\nHost: example.com\r\n\r\n{}",
            large_body
        );
        let request = Request::new(&request_string).unwrap();
        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.body(), Some(large_body.as_ref()));
    }

    #[test]
    fn test_request_with_special_characters_in_body() {
        let special_body = "body with special characters: !@#$%^&*()_+[]{}|;:',.<>?/";
        let request_string = format!(
            "POST /path HTTP/1.1\r\nHost: example.com\r\n\r\n{}",
            special_body
        );
        let request = Request::new(&request_string).unwrap();
        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.body(), Some(special_body.as_ref()));
    }

    #[test]
    fn test_request_missing_method() {
        let request_string = "/path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let result = Request::new(request_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_missing_path() {
        let request_string = "GET HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let result = Request::new(request_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_malformed_header() {
        let request_string = "GET /path HTTP/1.1\r\nHost example.com\r\n\r\n";
        let result = Request::new(request_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_path_method() {
        let request_string = "GET /test_path HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::new(request_string).unwrap();
        assert_eq!(request.path(), "/test_path");
    }

    #[test]
    fn test_method_method() {
        let request_string = "POST /test_method HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::new(request_string).unwrap();
        assert_eq!(request.method(), "POST");
    }

    #[test]
    fn test_json_body() {
        let request_string = "POST /path HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\n\r\n{\"key\": \"value\"}";
        let request = Request::new(request_string).unwrap();
        let json_value = JsonValue::Object(vec![(
            "key".to_string(),
            JsonValue::String("value".to_string()),
        )]);
        assert_eq!(request.json_body(), Some(&json_value));
    }

    #[test]
    fn test_invalid_json_body() {
        let request_string = "POST /path HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\n\r\n{\"key\": \"value\"";
        let request = Request::new(request_string).unwrap();
        assert!(request.json_body().is_none());
    }

    #[test]
    fn test_form_data() {
        let request_string = "POST /path HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nkey1=value1&key2=value2\0\0\0\0";
        let request = Request::new(request_string).unwrap();
        let mut expected_form_data = HashMap::new();
        expected_form_data.insert("key1".to_string(), "value1".to_string());
        expected_form_data.insert("key2".to_string(), "value2".to_string());
        assert_eq!(request.form_data(), Some(&expected_form_data));
    }
}
