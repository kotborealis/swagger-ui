use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::{Content, Responder, Redirect};
use rocket::{Data, Request, Route};

/// A content handler is a wrapper type around `rocket::response::Content`, which can be turned into
/// a `rocket::Route` that serves the content with correct content-type.
#[derive(Clone)]
pub struct ContentHandler<R: Responder<'static> + Clone + Send + Sync + 'static> {
    content: Content<R>,
}

impl ContentHandler<String> {
    /// Create a `ContentHandler<String>` which serves its content as JSON.
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler {
            content: Content(ContentType::JSON, json),
        }
    }
}

impl ContentHandler<Vec<u8>> {
    /// Create a `ContentHandler<Vec<u8>>`, which serves its content with the specified
    /// `content_type`.
    pub fn bytes(content_type: ContentType, content: Vec<u8>) -> Self {
        ContentHandler {
            content: Content(content_type, content),
        }
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> ContentHandler<R> {
    /// Create a `rocket::Route` from the current `ContentHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> Handler for ContentHandler<R> {
    fn handle<'r>(&self, req: &'r Request, data: Data) -> Outcome<'r> {
        // match e.g. "/index.html" but not "/index.html/"
        if req.uri().path().ends_with('/') {
            Outcome::Forward(data)
        } else {
            Outcome::from(req, self.content.clone())
        }
    }
}

/// A handler that instead of serving content always redirects to some specified destination URL.
#[derive(Clone)]
pub struct RedirectHandler {
    dest: &'static str,
}

impl RedirectHandler {
    /// Create a new `RedirectHandler` that redirects to the specified URL.
    pub fn to(dest: &'static str) -> Self {
        Self {
            dest: dest.trim_start_matches('/'),
        }
    }

    /// Create a new `Route` from this `Handler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl Handler for RedirectHandler {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        let path = req.route().unwrap().base().trim_end_matches('/');
        Outcome::from(req, Redirect::to(format!("{}/{}", path, self.dest)))
    }
}