#![feature(test)] extern crate test;


const TEST_CASES: &[(&[u8], Option<ClientHeaderName>)] = &[
    (b"accept", Some(ClientHeaderName::Accept)),
    (b"expectation", None),
    (b"content-type", Some(ClientHeaderName::ContentType)),
    (b"accept-encode", None),
    (b"accept-language", Some(ClientHeaderName::AcceptLanguage)),
    (b"datee", None),
    (b"forwarded", Some(ClientHeaderName::Forwarded)),
    (b"host", Some(ClientHeaderName::Host)),
    (b"max-forwarded", None),
    (b"max-forwards", Some(ClientHeaderName::MaxForwards)),
    (b"proxy-auththenticate", None),
    (b"proxy-authorization", Some(ClientHeaderName::ProxyAuthorization)),
    (b"Proxy-Authorization", Some(ClientHeaderName::ProxyAuthorization)),
    (b"Accept-Language", Some(ClientHeaderName::AcceptLanguage)),
    (b"Sec-WebSocket-Extentions", None),
    (b"Sec-WebSocket-extensions", None),
    (b"Sec-WebSocket-Extensions", Some(ClientHeaderName::SecWebSocketExtensions)),
    (b"referrer", None),
    (b"referer", Some(ClientHeaderName::Referer)),
    (b"Referer", Some(ClientHeaderName::Referer)),
    (b"Accept-Language", Some(ClientHeaderName::AcceptLanguage)),
    (b"Max-Forwarded", None),
    (b"Max-Forwards", Some(ClientHeaderName::MaxForwards)),
    (b"Trailer", Some(ClientHeaderName::Trailer)),
    (b"trailer", Some(ClientHeaderName::Trailer)),
    (b"t-e", None),
    (b"T-E", None),
    (b"TE", Some(ClientHeaderName::TE)),
    (b"content-len", None),
    (b"Content-Length", Some(ClientHeaderName::ContentLength)),
    (b"content-types", None),
    (b"content-type", Some(ClientHeaderName::ContentType)),
];

#[bench] #[allow(non_snake_case)] fn v1(b: &mut test::Bencher) {
    b.iter(|| {
        for (input, expected) in TEST_CASES {
            assert_eq!(&from_bytes_v1(input), expected);
        }
    })
}
#[bench] #[allow(non_snake_case)] fn v2(b: &mut test::Bencher) {
    b.iter(|| {
        for (input, expected) in TEST_CASES {
            assert_eq!(&from_bytes_v2(input), expected);
        }
    })
}


macro_rules! ClientHeaderName {
    ($N:literal; $( $konst:ident: $name_bytes:literal | $lower_case:literal, )*) => {
        const _: [&[u8]; $N] = [ $( $name_bytes ),* ];
        #[derive(Debug, PartialEq)]
        pub enum ClientHeaderName {
            $( $konst, )*
        }
        impl ClientHeaderName {
            #[inline] pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$konst => unsafe {std::str::from_utf8_unchecked($name_bytes)},
                    )*
                }
            }
        }

        const fn from_bytes_v1(bytes: &[u8]) -> Option<ClientHeaderName> {
            match bytes {
                $(
                    $name_bytes | $lower_case => Some(ClientHeaderName::$konst),
                )*
                _ => None
            }
        }

        impl<S: AsRef<[u8]>> PartialEq<S> for ClientHeaderName {
            #[inline] fn eq(&self, other: &S) -> bool {
                self.as_str().as_bytes() == other.as_ref()
            }
        }
    };
} ClientHeaderName! {40;
    // 2 bytes
    TE: b"TE" | b"te",

    // 3 bytes
    Via: b"Via" | b"via",

    // 4 bytes
    Date: b"Date" | b"date",
    From: b"From" | b"from",
    Host: b"Host" | b"host",
    Link: b"Link" | b"link",

    // 5 bytes
    Range: b"Range" | b"range",

    // 6 bytes
    Accept: b"Accept" | b"accept",
    Cookie: b"Cookie" | b"cookie",
    Expect: b"Expect" | b"expect",
    Origin: b"Origin" | b"origin",

    // 7 bytes
    Referer: b"Referer" | b"referer",
    Trailer: b"Trailer" | b"trailer",
    Upgrade: b"Upgrade" | b"upgrade",

    // 8 bytes
    IfMatch: b"If-Match" | b"if-match",
    IfRange: b"If-Range" | b"if-range",

    // 9 bytes
    Forwarded: b"Forwarded" | b"forwarded",

    // 10 bytes
    UserAgent:  b"User-Agent" | b"user-agent",
    Connection: b"Connection" | b"connection",

    // 12 bytes
    ContentType: b"Content-Type" | b"content-type",
    MaxForwards: b"Max-Forwards" | b"max-forwards",

    // 13 bytes
    IfNoneMatch:   b"If-None-Match" | b"if-none-match",
    CacheControl:  b"Cache-Control" | b"cache-control",
    Authorization: b"Authorization" | b"authorization",

    // 14 bytes
    ContentLength: b"Content-Length" | b"content-length",

    // 15 bytes
    AcceptEncoding:  b"Accept-Encoding" | b"accept-encoding",
    AcceptLanguage:  b"Accept-Language" | b"accept-language",

    // 16 bytes
    ContentEncoding: b"Content-Encoding" | b"content-encoding",
    ContentLanguage: b"Content-Language" | b"content-language",
    ContentLocation: b"Content-Location" | b"content-location",

    // 17 bytes
    SecWebSocketKey:  b"Sec-WebSocket-Key" | b"sec-websocket-key",
    IfModifiedSince:  b"If-Modified-Since" | b"if-modified-since",
    TransferEncoding: b"Transfer-Encoding" | b"transfer-encoding",

    // 19 bytes
    IfUnmodifiedSince:  b"If-Unmodified-Since" | b"if-unmodified-since",
    ContentDisposition: b"Content-Disposition" | b"content-disposition",
    ProxyAuthorization: b"Proxy-Authorization" | b"proxy-authorization",

    // 21 bytes
    SecWebSocketVersion: b"Sec-WebSocket-Version" | b"sec-websocket-version",

    // 22 bytes
    SecWebSocketProtocol: b"Sec-WebSocket-Protocol" | b"sec-websocket-protocol",

    // 24 bytes
    SecWebSocketExtensions: b"Sec-WebSocket-Extensions" | b"sec-websocket-extensions",

    // 25 bytes
    UpgradeInsecureRequests: b"Upgrade-Insecure-Requests" | b"upgrade-insecure-requests",
}

const fn from_bytes_v2(bytes: &[u8]) -> Option<ClientHeaderName> {
    match bytes.len() {
        2 => match bytes {
            b"TE" | b"te" => Some(ClientHeaderName::TE),
            _ => None
        }
        3 => match bytes {
            b"Via" | b"via" => Some(ClientHeaderName::Via),
            _ => None
        }
        4 => match bytes {
            b"Date" | b"date" => Some(ClientHeaderName::Date),
            b"From" | b"from" => Some(ClientHeaderName::From),
            b"Host" | b"host" => Some(ClientHeaderName::Host),
            b"Link" | b"link" => Some(ClientHeaderName::Link),
            _ => None
        }
        5 => match bytes {
            b"Range" | b"range" => Some(ClientHeaderName::Range),
            _ => None
        }
        6 => match bytes {
            b"Accept" | b"accept" => Some(ClientHeaderName::Accept),
            b"Cookie" | b"cookie" => Some(ClientHeaderName::Cookie),
            b"Expect" | b"expect" => Some(ClientHeaderName::Expect),
            b"Origin" | b"origin" => Some(ClientHeaderName::Origin),
            _ => None
        }
        7 => match bytes {
            b"Referer" | b"referer" => Some(ClientHeaderName::Referer),
            b"Trailer" | b"trailer" => Some(ClientHeaderName::Trailer),
            b"Upgrade" | b"upgrade" => Some(ClientHeaderName::Upgrade),
            _ => None
        }
        8 => match bytes {
            b"If-Match" | b"if-match" => Some(ClientHeaderName::IfMatch),
            b"If-Range" | b"if-range" => Some(ClientHeaderName::IfRange),
            _ => None
        }
        9 => match bytes {
            b"Forwarded" | b"forwarded" => Some(ClientHeaderName::Forwarded),
            _ => None
        }
        10 => match bytes {
            b"User-Agent" | b"user-agent" => Some(ClientHeaderName::UserAgent),
            b"Connection" | b"connection" => Some(ClientHeaderName::Connection),
            _ => None
        }
        12 => match bytes {
            b"Content-Type" | b"content-type" => Some(ClientHeaderName::ContentType),
            b"Max-Forwards" | b"max-forwards" => Some(ClientHeaderName::MaxForwards),
            _ => None
        }
        13 => match bytes {
            b"If-None-Match" | b"if-none-match" => Some(ClientHeaderName::IfNoneMatch),
            b"Cache-Control" | b"cache-control" => Some(ClientHeaderName::CacheControl),
            b"Authorization" | b"authorization" => Some(ClientHeaderName::Authorization),
            _ => None
        }
        14 => match bytes {
            b"Content-Length" | b"content-length" => Some(ClientHeaderName::ContentLength),
            _ => None
        }
        15 => match bytes {
            b"Accept-Encoding" | b"accept-encoding" => Some(ClientHeaderName::AcceptEncoding),
            b"Accept-Language" | b"accept-language" => Some(ClientHeaderName::AcceptLanguage),
            _ => None
        }
        16 => match bytes {
            b"Content-Encoding" | b"content-encoding" => Some(ClientHeaderName::ContentEncoding),
            b"Content-Language" | b"content-language" => Some(ClientHeaderName::ContentLanguage),
            b"Content-Location" | b"content-location" => Some(ClientHeaderName::ContentLocation),
            _ => None
        }
        17 => match bytes {
            b"Sec-WebSocket-Key" | b"sec-websocket-key" => Some(ClientHeaderName::SecWebSocketKey),
            b"If-Modified-Since" | b"if-modified-since" => Some(ClientHeaderName::IfModifiedSince),
            b"Transfer-Encoding" | b"transfer-encoding" => Some(ClientHeaderName::TransferEncoding),
            _ => None
        }
        19 => match bytes {
            b"If-Unmodified-Since" | b"if-unmodified-since" => Some(ClientHeaderName::IfUnmodifiedSince),
            b"Content-Disposition" | b"content-disposition" => Some(ClientHeaderName::ContentDisposition),
            b"Proxy-Authorization" | b"proxy-authorization" => Some(ClientHeaderName::ProxyAuthorization),
            _ => None
        }
        21 => match bytes {
            b"Sec-WebSocket-Version" | b"sec-websocket-version" => Some(ClientHeaderName::SecWebSocketVersion),
            _ => None
        }
        22 => match bytes {
            b"Sec-WebSocket-Protocol" | b"sec-websocket-protocol" => Some(ClientHeaderName::SecWebSocketProtocol),
            _ => None
        }
        24 => match bytes {
            b"Sec-WebSocket-Extensions" | b"sec-websocket-extensions" => Some(ClientHeaderName::SecWebSocketExtensions),
            _ => None
        }
        25 => match bytes {
            b"Upgrade-Insecure-Requests" | b"upgrade-insecure-requests" => Some(ClientHeaderName::UpgradeInsecureRequests),
            _ => None
        }
        _ => None
    }
}
