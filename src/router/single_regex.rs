/* ---

^/(?:
    help$()
    |
    ([^/]+)/(?:
        followe(?:
            es$()
            |
            rs$()
        )
        |
        posts(?:
            $()
            |
            /([^/]+)(?:
                $()
                |
                /likes$()
            )
        )
    )
)

--- */

use regex::{RegexSet, Regex};
use super::{Handler, Router, Method, HandleFunc};


// pub struct SingleRegexRouter {
//     GET: ,
//     POST: ,
//     PATCH: ,
//     DELETE: ,
// }
