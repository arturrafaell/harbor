pub fn parse_request_line(request: &str) -> (&str, &str, &str) {
    let request_line = request
        .lines()
        .next()
        .expect("The HTTP request does not contain a request line");

    let mut request_parts = request_line.split_whitespace();

    let method = request_parts
        .next()
        .expect("The HTTP request does not contain a method");

    let path = request_parts
        .next()
        .expect("The HTTP request does not contain a path");

    let version = request_parts
        .next()
        .expect("The HTTP request does not contain a version");

    (method, path, version)
}