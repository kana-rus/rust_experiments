use super::{Request, REQUEST_BUFFER_SIZE, Method, QueryParams, Headers, Buffer};

impl Request {
    pub fn parse_via_str(buffer: [u8; REQUEST_BUFFER_SIZE]) -> Self {
        let mut start = 0;
        let mut lines =
            unsafe {std::str::from_utf8_unchecked(&buffer)}
            .lines();


        let mut line = lines.next().unwrap();
        let len = line.len();

        let method = {
            let end = line.find(' ').unwrap();
            let method = Method::parse_str(&line[..end]);
                start += end + 1;
                line = &line[start..];
            method
        };

        let (path, queries) = {
            let (path_str, _) = line.split_once(' ').unwrap();
            match path_str.split_once('?') {
                None => {
                    let path = start..(start + path_str.len());
                    (path, QueryParams::new())
                },
                Some((path_str, queries_str)) => {
                    let mut q = start + path_str.len();
                    let path = start..q;

                    let mut queries = QueryParams::new();
                    for (e, len) in queries_str
                        .split('&')
                        .map(|s|
                            (s.find('=').unwrap(), s.len())
                        )
                    {
                        q += 1/*'?' or '&'*/;
                        queries.push(
                            (q)..(q+e),
                            (q+e+1)..(q+len)
                        );
                        q += len;
                    }

                    (path, queries)
                },
            }
        };

        let mut headers = Headers::new();
        start += len + 2/*'\r\n'*/;
        while let Some(line) = lines.next() {
            if line.is_empty() {break}
            let len = line.len();

            let e = line.find('=').unwrap();
            headers.append(
                start..(start+e),
                (start+e+1)..(start+len)
            );

            start += len + 2/*'\r\n'*/;
        }

        let body = lines.next()
            .map(|s| start..(start + s.len()));

        Self { buffer:Buffer(buffer), method, path, queries, headers, body }
    }
}

#[cfg(test)]
mod test {
    use crate::parse_request::{TEST_REQUEST, REQUEST_BUFFER_SIZE, Request, Buffer};

    #[test]
    fn parse_request_via_str() {
        let test_case: [u8; REQUEST_BUFFER_SIZE] = {
            let mut buffer = TEST_REQUEST.as_bytes().to_vec();
            buffer.resize(REQUEST_BUFFER_SIZE, b' ');
            buffer.try_into().unwrap()
        };

        let parsed = Request::parse_via_str(test_case.clone());
        let buffer = Buffer(test_case);

        assert_eq!();
    }
}
