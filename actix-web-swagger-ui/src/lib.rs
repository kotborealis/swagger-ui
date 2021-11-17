use actix_files::file_extension_to_mime;
use actix_utils::future::ok;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse, Route};

use swagger_ui::{Assets, Config, Spec};

const CONFIG_FILE_PATH: &str = "/swagger-ui-config.json";

/// Returns a function which configures an `App` or a `Scope` to serve the swagger-ui page displaying the given `Spec`
pub fn swagger(spec: Spec, config: Config) -> impl FnOnce(&mut ServiceConfig) {
    let mut routes: Vec<(String, Route)> = vec![];

    let config_route = config_route(config, spec.name.clone());
    routes.push(("/swagger-ui-config.json".into(), config_route));

    let spec_path = spec.name.clone();
    let spec_route = spec_route(spec);
    routes.push((spec_path, spec_route));

    let index_route = index_route();
    routes.push(("".into(), index_route));

    for file in Assets::iter() {
        let filename = file.as_ref();
        let content_type = content_type(filename);
        let content = Assets::get(filename).unwrap().into_owned();

        routes.push((format!("/{}", filename), body(content_type, content)));
    }

    move |c| {
        for (path, route) in routes {
            c.route(path.as_str(), route);
        }
    }
}

fn config_route(config: Config, spec_name: String) -> Route {
    web::to(move |req: HttpRequest| {
        let path = req.path().replace(CONFIG_FILE_PATH, "");
        let mut config = config.clone();
        config.url = format!("{}/{}", path, &spec_name);

        HttpResponse::Ok().json(config)
    })
}

fn spec_route(spec: Spec) -> Route {
    let content_type = content_type(&spec.name);
    let content = Vec::from(spec.content);

    web::to(move || {
        HttpResponse::Ok()
            .content_type(content_type.clone())
            .body(content.clone())
    })
}

fn index_route() -> Route {
    web::to(|req: HttpRequest| {
        let path = req.path();

        let config_url = format!("{}{}", path, CONFIG_FILE_PATH);
        let index_url = format!("{}/index.html?configUrl={}", path, config_url);

        HttpResponse::Found()
            .append_header((LOCATION, index_url))
            .finish()
    })
}

fn body(content_type: ContentType, content: Vec<u8>) -> Route {
    let handler = move || {
        ok::<_, actix_web::Error>(
            HttpResponse::Ok()
                .content_type(content_type.clone())
                .body(content.clone()),
        )
    };

    web::to(handler)
}

fn content_type(filename: impl AsRef<str>) -> ContentType {
    let mime = file_extension_to_mime(extension(filename.as_ref()));

    ContentType(mime)
}

fn extension(filename: &str) -> &str {
    if let Some(dot_index) = filename.rfind('.') {
        &filename[dot_index + 1..]
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use actix_http::Request;
    use actix_web::body::Body;
    use actix_web::{http, test::{TestRequest, call_service, init_service, read_body}, web::scope, App};
    use actix_web::dev::ServiceResponse;
    use actix_web::web::Bytes;

    use super::*;

    macro_rules! init_app {
        ($scope:expr) => {{
            let spec = Spec {
                name: "openapi.json".into(),
                content: include_bytes!("../../swagger-ui/examples/openapi.json"),
            };
            let config = Config::default();
            let app = App::new()
                .service(scope($scope).configure(swagger(spec, config)));
            init_service(app).await
        }};
    }

    #[actix_rt::test]
    async fn extension_works() {
        assert_eq!("html", extension("index.html"));
        assert_eq!("js", extension("jquery.min.js"));
        assert_eq!("", extension("src"));
    }

    fn get(uri: impl AsRef<str>) -> Request {
        TestRequest::with_uri(uri.as_ref()).to_request()
    }

    fn has_location(res: &ServiceResponse, expected_location: String) -> bool {
        let location = res.headers().get(LOCATION).unwrap();

        location.eq(expected_location.as_str().into())
    }

    #[actix_rt::test]
    async fn index_redirects_with_config_url_param() {
        let prefix = "/swagger-ui";

        let mut app = init_app!(prefix);

        let res = call_service(&mut app, get(prefix)).await;
        assert!(res.status().is_redirection());
        assert!(has_location(&res, format!("{0}/index.html?configUrl={0}/swagger-ui-config.json", prefix)));

        let res = call_service(&mut app, get(format!("{}/index.html", prefix))).await;
        assert!(res.status().is_success());

        let res = call_service(&mut app, get(format!("{}/swagger-ui-config.json", prefix))).await;
        assert!(res.status().is_success());

        let res = call_service(&mut app, get(format!("{}/openapi.json", prefix))).await;

        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/../swagger-ui/examples/openapi.json";
        println!("Loading {}", path);
        let expected_body = Bytes::from(fs::read(path).unwrap());

        let body = read_body(res).await;

        assert_eq!(body, expected_body);
    }
}
