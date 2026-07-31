//! Basic `Request`/`Response` usage example.

use edge_application_base::{Request, Response};

struct Greeting {
    name: String,
}

struct Farewell {
    name: String,
}

impl Request for Greeting {}
impl Response for Farewell {}

fn greet<T: Request>(req: &T) {
    let _ = req;
}

fn respond<T: Response>(resp: &T) {
    let _ = resp;
}

fn main() {
    let req = Greeting {
        name: "world".to_string(),
    };
    greet(&req);
    println!("hello, {}", req.name);

    let resp = Farewell {
        name: "world".to_string(),
    };
    respond(&resp);
    println!("goodbye, {}", resp.name);
}
