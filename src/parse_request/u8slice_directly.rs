use super::{Request, REQUEST_BUFFER_SIZE, Method, QueryParams, Buffer, Headers};

impl Request {
    pub fn parse_directly(buffer: [u8; REQUEST_BUFFER_SIZE]) -> Self {
        let mut start = 0;

        let method = {
            let mut end = start;
            for b in &buffer[start..] {
                if *b == b' ' {break}
                end += 1
            }
            let method = Method::parse_bytes(&buffer[start..end]);
            start = end + 1;
            method
        };

        let mut have_queries = false;
        let path = {
            let mut end = start;
            for b in &buffer[start..] {
                match b {
                    b'?' => {have_queries = true; break},
                    b' ' => break,
                    _ => end += 1
                }
            }
            let path = start..end;
            start = end + 1;
            path
        };

        let mut queries = QueryParams::new(); if have_queries {
            let mut query_start = start;
            loop {
                let mut is_final = false;

                let mut eq = query_start;
                for b in &buffer[query_start..] {
                    if *b == b'=' {
                        break
                    } else {eq += 1}
                }

                let mut end = eq + 1;
                for b in &buffer[end..] {
                    match b {
                        b' ' => {is_final = true; break},
                        b'&' => break,
                        _ => end += 1
                    }
                }

                queries.push(
                    query_start..eq,
                    (eq+1)..end
                );
                query_start = end + 1/*' ' or '&'*/;
                if is_final {break}
            }
            start = query_start
        }

        let _/*HTTP version*/ = {
            for b in &buffer[start..] {
                start += 1;
                if *b == b'\n' {break}
            }
        };

        let mut headers = Headers::new(); {
            let mut header_start = start;
            loop {
                if buffer[header_start] == b'\r' {break}

                let mut colon = header_start;
                for b in &buffer[header_start..] {
                    if *b == b':' {
                        break
                    } else {colon += 1}
                }

                let mut end = colon + 1/*' '*/ + 1;
                for b in &buffer[end..] {
                    if *b == b'\r' {
                        break
                    } else {end += 1}
                }

                headers.append(
                    header_start..colon,
                    (colon+1/*' '*/+1)..end
                );
                header_start = end + 1/*'\n'*/ + 1
            }
            start = header_start + 1/*'\n'*/ + 1
        };

        let body = (buffer[start] != 0).then_some({
            let mut end = start;
            for b in &buffer[start..] {
                if *b == 0 {
                    break
                } else {end += 1}
            }
            start..end
        });

        Self { buffer:Buffer(buffer), method, path, queries, headers, body }
    }
}

#[cfg(test)]
mod test {
    use crate::parse_request::{TEST_REQUEST, REQUEST_BUFFER_SIZE, Request};

    #[test]
    fn parse_request_directly() {
        let test_case: [u8; REQUEST_BUFFER_SIZE] = {
            let mut buffer = TEST_REQUEST.as_bytes().to_vec();
            buffer.resize(REQUEST_BUFFER_SIZE, 0/*null*/);
            buffer.try_into().unwrap()
        };

        let parsed = Request::parse_directly(test_case.clone());
        assert_eq!(parsed.path(), "/search.html");
        assert_eq!(parsed.query("q1"), Some("query"));
        assert_eq!(parsed.query("q2"), Some("42"));
        assert_eq!(parsed.header("Host"), Some("wa3.i-3-i.info"));
        assert_eq!(parsed.body(), Some("q=test&submitSearch=%E6%A4%9C%E7%B4%A2"));
    }
}
