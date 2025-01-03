use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result as IoResult};
use std::sync::{Arc, Mutex};
use suika_json::JsonValue;
use suika_utils::parse_query_string;

/// Represents an HTTP Request.
#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    path: String,
    original_path: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<String>,
    json_body: Option<JsonValue>,
    form_data: Option<HashMap<String, String>>,
    params: HashMap<String, String>,
    modules: Arc<Mutex<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>>,
}

impl Request {
    /// Creates a new `Request` from a request string.
    ///
    /// # Arguments
    ///
    /// * `request_string` - A string slice that holds the HTTP request.
    ///
    /// # Errors
    ///
    /// Returns an `IoResult` containing an error if the request string is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request_string = "GET /path?name=value HTTP/1.1\r\nHost: example.com\r\n\r\n";
    /// let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();
    ///
    /// assert_eq!(request.method(), "GET");
    /// assert_eq!(request.path(), "/path");
    /// assert_eq!(request.header("Host"), Some("example.com"));
    /// assert_eq!(request.query_param("name"), Some("value"));
    /// ```
    pub fn new(
        request_string: &str,
        modules: Arc<Mutex<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>>,
    ) -> IoResult<Request> {
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
            match suika_json::parse_json(&body_content) {
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
            path: path.clone(),
            original_path: path,
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
            modules,
        })
    }

    /// Retrieves a module from the request context by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the module to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing an `Arc` to the module if found, or `None` if not found or if the module
    /// cannot be downcast to the expected type.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc, Mutex};
    /// use std::collections::HashMap;
    /// use std::any::Any;
    ///
    /// struct MyModule;
    ///
    /// impl MyModule {
    ///     fn new() -> Self {
    ///         MyModule
    ///     }
    /// }
    ///
    /// let mut modules: HashMap<String, Arc<dyn Any + Send + Sync>> = HashMap::new();
    /// modules.insert("my_module".to_string(), Arc::new(MyModule::new()) as Arc<dyn Any + Send + Sync>);
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(modules)),
    /// ).unwrap();
    ///
    /// let module: Option<Arc<MyModule>> = request.module("my_module");
    /// assert!(module.is_some());
    /// ```
    pub fn module<T: 'static + Send + Sync>(&self, name: &str) -> Option<Arc<T>> {
        let modules = self.modules.lock().unwrap();
        modules.get(name)?.clone().downcast::<T>().ok()
    }

    /// Returns the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.method(), "GET");
    /// ```
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Returns the path of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n", Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.path(), "/path");
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the original path of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.original_path(), "/path");
    /// ```
    pub fn original_path(&self) -> &str {
        &self.original_path
    }

    /// Returns the value of the specified header.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the header name.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\nHost: example.com\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.header("Host"), Some("example.com"));
    /// ```
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    /// Returns the value of the specified query parameter.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the query parameter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "GET /path?name=value HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.query_param("name"), Some("value"));
    /// ```
    pub fn query_param(&self, key: &str) -> Option<&str> {
        self.query_params.get(key).map(|s| s.as_str())
    }

    /// Returns the body of the request as a string, if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new(
    ///     "POST /path HTTP/1.1\r\n\r\nbody_content",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert_eq!(request.body(), Some("body_content"));
    /// ```
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    /// Sets the JSON body of the request.
    ///
    /// # Arguments
    ///
    /// * `json` - A `JsonValue` representing the JSON body.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::{Request};
    /// use suika_json::JsonValue;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let mut request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// let json = JsonValue::String("new_value".to_string());
    ///
    /// request.set_json_body(json.clone());
    /// assert_eq!(request.json_body(), Some(&json));
    /// ```
    pub fn set_json_body(&mut self, json: JsonValue) {
        self.json_body = Some(json);
    }

    /// Returns the JSON body of the request, if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request_string = "POST /path HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}";
    /// let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();
    /// assert!(request.json_body().is_some());
    /// ```
    pub fn json_body(&self) -> Option<&JsonValue> {
        self.json_body.as_ref()
    }

    /// Returns the form data of the request, if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let request_string = "POST /path HTTP/1.1\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nkey=value";
    /// let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();
    /// assert!(request.form_data().is_some());
    /// ```
    pub fn form_data(&self) -> Option<&HashMap<String, String>> {
        self.form_data.as_ref()
    }

    /// Sets the parameters of the request.
    ///
    /// # Arguments
    ///
    /// * `params` - A `HashMap` containing the parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::collections::HashMap;
    /// use std::sync::{Arc,Mutex};
    ///
    /// let mut request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// let mut params = HashMap::new();
    /// params.insert("key".to_string(), "value".to_string());
    ///
    /// request.set_params(params.clone());
    /// assert_eq!(request.param("key"), Some("value"));
    /// ```
    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    /// Returns the value of the specified parameter.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the parameter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::collections::HashMap;
    /// use std::sync::{Arc,Mutex};
    ///
    /// let mut request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// let mut params = HashMap::new();
    /// params.insert("key".to_string(), "value".to_string());
    /// request.set_params(params.clone());
    ///
    /// assert_eq!(request.param("key"), Some("value"));
    /// ```
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Returns all headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::collections::HashMap;
    /// use std::sync::{Arc,Mutex};
    ///
    /// let request = Request::new(
    ///     "GET /path HTTP/1.1\r\nHost: example.com\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert!(request.headers().contains_key("Host"));
    /// ```
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns all query parameters of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::collections::HashMap;
    /// use std::sync::{Arc,Mutex};
    ///
    /// let request = Request::new(
    ///     "GET /path?name=value HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// assert!(request.query_params().contains_key("name"));
    /// ```
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    /// Sets the path of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the new path.
    ///
    /// # Examples
    ///
    /// ```
    /// use suika_server::request::Request;
    /// use std::sync::{Arc,Mutex};
    /// use std::collections::HashMap;
    ///
    /// let mut request = Request::new(
    ///     "GET /path HTTP/1.1\r\n\r\n",
    ///     Arc::new(Mutex::new(HashMap::new())),
    /// ).unwrap();
    ///
    /// request.set_path("/new_path".to_string());
    /// assert_eq!(request.path(), "/new_path");
    /// ```
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;
    use suika_json::JsonValue;

    #[test]
    fn test_new_request() {
        let request_string = "GET /path?name=value HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();

        assert_eq!(request.method(), "GET");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.original_path(), "/path");
        assert_eq!(request.header("Host"), Some("example.com"));
        assert_eq!(request.query_param("name"), Some("value"));
        assert!(request.body().is_none());
    }

    #[test]
    fn test_new_request_with_json_body() {
        let request_string =
            "POST /path HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}";
        let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();

        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.original_path(), "/path");
        assert_eq!(request.header("Content-Type"), Some("application/json"));

        if let Some(JsonValue::Object(map)) = request.json_body() {
            let key_value = map.iter().find(|(k, _)| k == "key");
            assert_eq!(
                key_value,
                Some(&("key".to_string(), JsonValue::String("value".to_string())))
            );
        } else {
            panic!("Expected JSON body to be an object with key 'key'");
        }
    }

    #[test]
    fn test_new_request_with_form_data() {
        let request_string = "POST /path HTTP/1.1\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nkey=value";
        let request = Request::new(request_string, Arc::new(Mutex::new(HashMap::new()))).unwrap();

        assert_eq!(request.method(), "POST");
        assert_eq!(request.path(), "/path");
        assert_eq!(request.original_path(), "/path");
        assert_eq!(
            request.header("Content-Type"),
            Some("application/x-www-form-urlencoded")
        );
        let form_data = request.form_data().unwrap();
        assert_eq!(form_data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_set_json_body() {
        let mut request = Request::new(
            "GET /path HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
        let json = JsonValue::String("new_value".to_string());

        request.set_json_body(json.clone());
        assert_eq!(request.json_body(), Some(&json));
    }

    #[test]
    fn test_set_params() {
        let mut request = Request::new(
            "GET /path HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
        let mut params = HashMap::new();
        params.insert("key".to_string(), "value".to_string());

        request.set_params(params.clone());
        assert_eq!(request.param("key"), Some("value"));
    }

    #[test]
    fn test_set_path() {
        let mut request = Request::new(
            "GET /path HTTP/1.1\r\n\r\n",
            Arc::new(Mutex::new(HashMap::new())),
        )
        .unwrap();
        request.set_path("/new_path".to_string());
        assert_eq!(request.path(), "/new_path");
    }

    #[allow(dead_code)]
    struct MyModule;

    #[allow(dead_code)]
    impl MyModule {
        fn new() -> Self {
            MyModule
        }
    }

    #[test]
    fn test_module() {
        // Insert module as Arc<dyn Any + Send + Sync>
        let mut modules: HashMap<String, Arc<dyn Any + Send + Sync>> = HashMap::new();
        modules.insert(
            "my_module".to_string(),
            Arc::new(MyModule::new()) as Arc<dyn Any + Send + Sync>,
        );

        let request =
            Request::new("GET /path HTTP/1.1\r\n\r\n", Arc::new(Mutex::new(modules))).unwrap();

        let module: Option<Arc<MyModule>> = request.module("my_module");
        assert!(module.is_some());
    }

    #[test]
    fn test_module_not_found() {
        let modules: HashMap<String, Arc<dyn Any + Send + Sync>> = HashMap::new();
        let request =
            Request::new("GET /path HTTP/1.1\r\n\r\n", Arc::new(Mutex::new(modules))).unwrap();

        let module: Option<Arc<MyModule>> = request.module("non_existent_module");
        assert!(module.is_none());
    }

    #[allow(dead_code)]
    struct MyModule1;

    #[allow(dead_code)]
    struct MyModule2;

    #[allow(dead_code)]
    impl MyModule1 {
        fn new() -> Self {
            MyModule1
        }
    }

    #[allow(dead_code)]
    impl MyModule2 {
        fn new() -> Self {
            MyModule2
        }
    }

    #[test]
    fn test_module_wrong_type() {
        // Insert module as Arc<dyn Any + Send + Sync>
        let mut modules: HashMap<String, Arc<dyn Any + Send + Sync>> = HashMap::new();
        modules.insert(
            "my_module".to_string(),
            Arc::new(MyModule1::new()) as Arc<dyn Any + Send + Sync>,
        );

        let request =
            Request::new("GET /path HTTP/1.1\r\n\r\n", Arc::new(Mutex::new(modules))).unwrap();

        let module: Option<Arc<MyModule2>> = request.module("my_module");
        assert!(module.is_none());
    }
}
