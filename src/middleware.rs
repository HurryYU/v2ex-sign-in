use std::sync::Arc;
use ureq::middleware::{Middleware, MiddlewareNext};
use ureq::{Body, Error, SendBody, http};

pub struct GlobalCookieMiddleware {
    pub cookie: Arc<String>,
}

impl Middleware for GlobalCookieMiddleware {
    fn handle(
        &self,
        mut request: http::Request<SendBody>,
        next: MiddlewareNext,
    ) -> Result<http::Response<Body>, Error> {
        if let Ok(val) = http::HeaderValue::from_str(&self.cookie) {
            request.headers_mut().insert("Cookie", val);
        }
        next.handle(request)
    }
}
