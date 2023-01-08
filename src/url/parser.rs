pub mod parser {
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct ParseError {
        pub message: String,
        pub input: String,
    }

    // scheme://host:port/path?query#fragment
    #[derive(Debug)]
    pub struct Url {
        pub scheme: String,
        pub host: String,
        pub port: Option<u32>,
        pub path: String,
        pub query: Option<String>,
        pub fragment: Option<String>,
    }

    // Return Ok of Url struct or an Err
    pub fn parse_url(input: &str) -> Result<Url, ParseError> {
        let mut url = Url {
            scheme: "".to_string(),
            host: "".to_string(),
            port: None,
            path: "".to_string(),
            query: None,
            fragment: None,
        };

        // Oh god I need a mutable input
        let url_input = input;

        // Parse scheme
        let (scheme, rest) = match input.find("://") {
            Some(pos) => (&input[..pos], &input[pos + 3..]),
            None => {
                return Err(ParseError {
                    message: "Invalid URL: missing scheme".to_string(),
                    input: input.to_string(),
                })
            }
        };
        url.scheme = scheme.to_string();

        // Parse host
        let (host_port, rest) = match rest.find("/") {
            Some(pos) => (&rest[..pos], &rest[pos..]),
            None => {
                return Err(ParseError {
                    message: "Invalid URL: missing hostname".to_string(),
                    input: input.to_string(),
                })
            }
        };
        let (host, port) = match host_port.find(":") {
            Some(pos) => (&host_port[..pos], Some(&host_port[pos + 1..])),
            None => (host_port, None),
        };
        url.host = host.to_string();
        if let Some(port) = port {
            match u32::from_str(port) {
                Ok(p) => url.port = Some(p),
                Err(_) => {
                    return Err(ParseError {
                        message: "Invalid URL: invalid port".to_string(),
                        input: input.to_string(),
                    })
                }
            }
        }

        // Parse path
        if let Some(pos) = url_input.find("?") {
            url.path = url_input[..pos].to_string();
        } else if let Some(pos) = url_input.find("#") {
            url.path = url_input[..pos].to_string();
        } else {
            url.path = url_input.to_string();
        }

        // Parse query + fragment
        let rest = rest;
        let (path, rest) = match rest.find("?") {
            Some(pos) => (&rest[..pos], &rest[pos..]),
            None => match rest.find("#") {
                Some(pos) => (&rest[..pos], &rest[pos..]),
                None => (rest, ""),
            },
        };
        url.path = path.to_string();
        let (query, fragment) = match rest.find("#") {
            Some(pos) => (&rest[1..pos], Some(&rest[pos + 1..])),
            None => (rest, None),
        };
        if !query.is_empty() {
            url.query = Some(query.to_string());
        }
        if let Some(fragment) = fragment {
            url.fragment = Some(fragment.to_string());
        }

        Ok(url)
    }
}
