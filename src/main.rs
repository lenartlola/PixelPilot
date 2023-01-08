mod url;
use crate::url::parser::parser;


fn main() {
    // scheme://host:port/path?query#fragment
    let url = parser::parse_url("http://example.com:3000/home?someq#frag");
    println!("{:?}", url);
}

