use std::{
    collections::HashMap,
    str::Chars,
    task::{RawWaker, RawWakerVTable, Waker},
};

/// Parses a query string into a HashMap.
///
/// # Arguments
///
/// * `query` - A string slice that holds the query string.
///
/// # Returns
///
/// A HashMap containing the key-value pairs from the query string.
///
/// # Examples
///
/// ```
/// use suika_utils::parse_query_string;
/// let query = "name=John&age=30";
/// let params = parse_query_string(query);
/// assert_eq!(params.get("name"), Some(&"John".to_string()));
/// assert_eq!(params.get("age"), Some(&"30".to_string()));
/// ```
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let mut iter = pair.split('=');
            if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
                Some((key.to_string(), value.to_string()))
            } else {
                None
            }
        })
        .collect()
}

/// Skips whitespace characters in the input.
///
/// # Arguments
///
/// * `chars` - A mutable reference to an iterator over characters.
/// * `current_char` - A mutable reference to an Option containing the current character.
///
/// # Examples
///
/// ```
/// use suika_utils::skip_whitespace;
/// let input = "   abc";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// skip_whitespace(&mut chars, &mut current_char);
/// assert_eq!(current_char, Some('a'));
/// ```
pub fn skip_whitespace(chars: &mut Chars, current_char: &mut Option<char>) {
    while let Some(c) = *current_char {
        if c.is_whitespace() {
            *current_char = chars.next();
        } else {
            break;
        }
    }
}

/// Expects a specific sequence of characters in the input.
///
/// # Arguments
///
/// * `chars` - A mutable reference to an iterator over characters.
/// * `current_char` - A mutable reference to an Option containing the current character.
/// * `expected` - The expected sequence of characters.
///
/// # Returns
///
/// An empty Result if the sequence matches, or an Err with a descriptive message otherwise.
///
/// # Examples
///
/// ```
/// use suika_utils::expect_sequence;
/// let input = "true";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// assert!(expect_sequence(&mut chars, &mut current_char, "true").is_ok());
/// assert_eq!(current_char, None);
///
/// let input = "tru";
/// let mut chars = input.chars();
/// let mut current_char = chars.next();
/// assert!(expect_sequence(&mut chars, &mut current_char, "true").is_err());
/// ```
pub fn expect_sequence(
    chars: &mut Chars,
    current_char: &mut Option<char>,
    expected: &str,
) -> Result<(), String> {
    for expected_char in expected.chars() {
        if Some(expected_char) != *current_char {
            return Err(format!(
                "Expected '{}', found '{:?}'",
                expected_char, current_char
            ));
        }
        *current_char = chars.next();
    }
    Ok(())
}

/// Builds a URL from a base and a set of query parameters.
///
/// # Arguments
///
/// * `base` - A string slice that holds the base URL.
/// * `params` - A reference to a HashMap containing the query parameters.
///
/// # Returns
///
/// A String containing the full URL with query parameters.
///
/// # Examples
///
/// ```
/// use suika_utils::build_url;
/// let base = "https://example.com";
/// let mut params = std::collections::HashMap::new();
/// params.insert("name", "John");
/// params.insert("age", "30");
/// let url = build_url(base, &params);
/// assert_eq!(url, "https://example.com?age=30&name=John");
/// ```
pub fn build_url(base: &str, params: &HashMap<&str, &str>) -> String {
    let mut url = base.to_string();
    if !params.is_empty() {
        url.push('?');
        let mut query_params: Vec<_> = params.iter().collect();
        query_params.sort_by_key(|&(key, _)| key);
        let query_string: String = query_params
            .iter()
            .map(|&(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&query_string);
    }
    url
}

/// Parses a URL into its components: scheme, host, path, and query parameters.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL.
///
/// # Returns
///
/// An Option containing a tuple with the scheme, host, path, and a HashMap of query parameters.
///
/// # Examples
///
/// ```
/// use suika_utils::parse_url;
/// let url = "https://example.com/path?name=John&age=30";
/// let components = parse_url(url).unwrap();
/// assert_eq!(components.0, "https");
/// assert_eq!(components.1, "example.com");
/// assert_eq!(components.2, "/path");
/// assert_eq!(components.3.get("name"), Some(&"John".to_string()));
/// assert_eq!(components.3.get("age"), Some(&"30".to_string()));
/// ```
pub fn parse_url(url: &str) -> Option<(String, String, String, HashMap<String, String>)> {
    let mut url_parts = url.splitn(2, "://");
    let scheme = url_parts.next()?.to_string();
    let rest = url_parts.next()?;

    let mut rest_parts = rest.splitn(2, '/');
    let host_and_query = rest_parts.next()?.to_string();
    let path_and_query = rest_parts.next().unwrap_or("").to_string();

    let (host, path, query) = match host_and_query.find('?') {
        Some(query_start) => {
            let host = &host_and_query[..query_start];
            let query_string = &host_and_query[query_start + 1..];
            (
                host.to_string(),
                "/".to_string(),
                parse_query_string(query_string),
            )
        }
        None => match path_and_query.find('?') {
            Some(query_start) => {
                let path = &path_and_query[..query_start];
                let query_string = &path_and_query[query_start + 1..];
                (
                    host_and_query,
                    format!("/{}", path),
                    parse_query_string(query_string),
                )
            }
            None => (
                host_and_query,
                if path_and_query.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{}", path_and_query)
                },
                HashMap::new(),
            ),
        },
    };

    Some((scheme, host, path, query))
}

/// Minifies an HTML string by removing unnecessary whitespace and line breaks.
///
/// # Arguments
///
/// * `html` - A string slice that holds the HTML content.
///
/// # Returns
///
/// A String containing the minified HTML.
///
/// # Examples
///
/// ```
/// use suika_utils::minify_html;
/// let html = r#"<html>
///     <head>
///         <title>Test</title>
///     </head>
///     <body>
///         <h1>Hello, World!</h1>
///         <script type="module">
///             console.log("Test");
///         </script>
///     </body>
/// </html>"#;
/// let minified = minify_html(html);
/// assert_eq!(minified, "<html><head><title>Test</title></head><body><h1>Hello, World!</h1><script type=\"module\">console.log(\"Test\");</script></body></html>");
/// ```
pub fn minify_html(html: &str) -> String {
    let mut in_script = false;
    let mut result = String::with_capacity(html.len());
    let mut script_content = String::new();

    for line in html.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("<script") {
            in_script = true;
            result.push_str(trimmed_line);
        } else if trimmed_line.ends_with("</script>") {
            in_script = false;
            result.push_str(&script_content.replace("\n", ""));
            script_content.clear();
            result.push_str(trimmed_line);
        } else if in_script {
            script_content.push_str(trimmed_line);
        } else {
            result.push_str(trimmed_line);
        }
    }

    result
}

/// Creates a no-op Waker for use in tests.
///
/// # Returns
///
/// A Waker that does nothing when notified.
///
/// # Examples
///
/// ```
/// use suika_utils::noop_waker;
/// use std::task::{Context, Poll};
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::sync::{
///     atomic::{AtomicBool, Ordering},
///     Arc,
/// };
///
/// let waker = noop_waker();
/// let mut cx = Context::from_waker(&waker);
///
/// let ready = Arc::new(AtomicBool::new(false));
/// let ready_clone = Arc::clone(&ready);
///
/// let mut future = Box::pin(async move {
///     ready_clone.store(true, Ordering::SeqCst);
/// });
///
/// assert!(!ready.load(Ordering::SeqCst));
/// let _ = future.as_mut().poll(&mut cx);
/// assert!(ready.load(Ordering::SeqCst));
/// ```
pub fn noop_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::task::Context;

    #[test]
    fn test_parse_query_string() {
        let query = "name=John&age=30";
        let params = parse_query_string(query);
        assert_eq!(params.get("name"), Some(&"John".to_string()));
        assert_eq!(params.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_skip_whitespace() {
        let input = "   abc";
        let mut chars = input.chars();
        let mut current_char = chars.next();
        skip_whitespace(&mut chars, &mut current_char);
        assert_eq!(current_char, Some('a'));
    }

    #[test]
    fn test_expect_sequence() {
        let input = "true";
        let mut chars = input.chars();
        let mut current_char = chars.next();
        assert!(expect_sequence(&mut chars, &mut current_char, "true").is_ok());
        assert_eq!(current_char, None);

        let input = "false";
        let mut chars = input.chars();
        let mut current_char = chars.next();
        assert!(expect_sequence(&mut chars, &mut current_char, "false").is_ok());
        assert_eq!(current_char, None);

        let input = "tru";
        let mut chars = input.chars();
        let mut current_char = chars.next();
        assert!(expect_sequence(&mut chars, &mut current_char, "true").is_err());
    }

    #[test]
    fn test_build_url() {
        let base = "https://example.com";
        let mut params = HashMap::new();
        params.insert("name", "John");
        params.insert("age", "30");
        let url = build_url(base, &params);
        assert_eq!(url, "https://example.com?age=30&name=John");
    }

    #[test]
    fn test_parse_url() {
        let url = "https://example.com/path?name=John&age=30";
        let components = parse_url(url).unwrap();
        assert_eq!(components.0, "https");
        assert_eq!(components.1, "example.com");
        assert_eq!(components.2, "/path");
        assert_eq!(components.3.get("name"), Some(&"John".to_string()));
        assert_eq!(components.3.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_parse_url_no_path() {
        let url = "https://example.com?name=John&age=30";
        let components = parse_url(url).unwrap();
        assert_eq!(components.0, "https");
        assert_eq!(components.1, "example.com");
        assert_eq!(components.2, "/");
        assert_eq!(components.3.get("name"), Some(&"John".to_string()));
        assert_eq!(components.3.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_minify_html_basic() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <h1>Hello, World!</h1>
                </body>
            </html>
        "#;
        let expected =
            "<html><head><title>Test</title></head><body><h1>Hello, World!</h1></body></html>";
        let minified = minify_html(html);
        assert_eq!(minified, expected);
    }

    #[test]
    fn test_minify_html_with_script() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <h1>Hello, World!</h1>
                    <script type="module">
                        console.log("Test");
                    </script>
                </body>
            </html>
        "#;
        let expected = r#"<html><head><title>Test</title></head><body><h1>Hello, World!</h1><script type="module">console.log("Test");</script></body></html>"#;
        let minified = minify_html(html);
        assert_eq!(minified, expected);
    }

    #[test]
    fn test_minify_html_with_multiple_scripts() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <h1>Hello, World!</h1>
                    <script type="module">
                        console.log("Test 1");
                    </script>
                    <script type="module">
                        console.log("Test 2");
                    </script>
                </body>
            </html>
        "#;
        let expected = r#"<html><head><title>Test</title></head><body><h1>Hello, World!</h1><script type="module">console.log("Test 1");</script><script type="module">console.log("Test 2");</script></body></html>"#;
        let minified = minify_html(html);
        assert_eq!(minified, expected);
    }

    #[test]
    fn test_noop_waker() {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        let ready = Arc::new(AtomicBool::new(false));
        let ready_clone = Arc::clone(&ready);

        let mut future = Box::pin(async move {
            ready_clone.store(true, Ordering::SeqCst);
        });

        assert!(!ready.load(Ordering::SeqCst));
        let _ = future.as_mut().poll(&mut cx);
        assert!(ready.load(Ordering::SeqCst));
    }
}
