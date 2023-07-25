use std::collections::HashMap;

use tokio::{
    io,
    net::{TcpListener, ToSocketAddrs},
};

pub struct Router {
    pub listener: TcpListener,
    pub routes: HashMap<String, fn(buf: [u8; 1460])>,
}

impl Router {
    pub async fn new<A: ToSocketAddrs>(add: A) -> io::Result<Self> {
        let listener = TcpListener::bind(add).await?;

        Ok(Router {
            listener,
            routes: HashMap::new(),
        })
    }

    pub fn add_route(self: &mut Self, route: String, handler: fn(buf: [u8; 1460])) {
        self.routes.insert(route, handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router_new() {
        match Router::new("127.0.0.1:8080").await {
            Ok(_router) => {
                assert!(true, "Router was created");
            }
            Err(e) => {
                assert!(false, "Failed to create router: {:?}", e);
            }
        }
    }

    fn a(_buf: [u8; 1460]) {
        unimplemented!("What the f*ck")
    }

    #[tokio::test]
    async fn test_create_route() {
        let mut router = Router::new("127.0.0.1:8080")
            .await
            .expect("Failed to create router");

        router.add_route("/test".to_string(), a);

        match router.routes.get("/test") {
            Some(_) => {
                assert!(true);
            }
            None => {
                assert!(false);
            }
        }
    }
}
