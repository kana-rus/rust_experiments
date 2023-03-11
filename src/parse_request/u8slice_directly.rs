use super::{Request, REQUEST_BUFFER_SIZE};

impl Request {
    pub fn parse_directly(buffer: [u8; REQUEST_BUFFER_SIZE]) -> Self {
        
    }
}

#[cfg(test)]
mod test {
    use crate::parse_request::{TEST_REQUEST, REQUEST_BUFFER_SIZE, Request};

    #[test]
    fn parse_request_directly() {
        let test_case: [u8; REQUEST_BUFFER_SIZE] = {
            let mut buffer = TEST_REQUEST.as_bytes().to_vec();
            buffer.resize(REQUEST_BUFFER_SIZE, b' ');
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
