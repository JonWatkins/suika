use std::collections::HashMap;

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

pub fn parse_url(url: &str) -> Option<(String, String, String, HashMap<String, String>)> {
    let url_parts: Vec<&str> = url.split("://").collect();
    if url_parts.len() != 2 {
        return None;
    }

    let scheme = url_parts[0].to_string();
    let mut rest = url_parts[1].splitn(2, '/');
    let host_and_query = rest.next()?.to_string();
    let path_and_query = rest.next().unwrap_or("").to_string();

    let (host, path, query) = if let Some(query_start) = host_and_query.find('?') {
        let host = host_and_query[..query_start].to_string();
        let query_string = &host_and_query[query_start + 1..];
        (host, "/".to_string(), parse_query_string(query_string))
    } else {
        let (path, query) = if let Some(query_start) = path_and_query.find('?') {
            let path = path_and_query[..query_start].to_string();
            let query_string = &path_and_query[query_start + 1..];
            (format!("/{}", path), parse_query_string(query_string))
        } else if path_and_query.is_empty() {
            ("/".to_string(), HashMap::new())
        } else {
            (format!("/{}", path_and_query), HashMap::new())
        };
        (host_and_query, path, query)
    };

    Some((scheme, host, path, query))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_query_string() {
        let query = "name=John&age=30";
        let params = parse_query_string(query);
        assert_eq!(params.get("name"), Some(&"John".to_string()));
        assert_eq!(params.get("age"), Some(&"30".to_string()));
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
}
