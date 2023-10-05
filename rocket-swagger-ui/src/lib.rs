mod handlers;

use rocket::http::{ContentType};
use rocket::{Route};
use crate::handlers::{ContentHandler, RedirectHandler};
use swagger_ui::{Assets, Config, Spec};
use std::path::Path;

fn mime_type(filename: &str) -> ContentType {
    let parts = filename.split('.').collect::<Vec<&str>>();
    match parts.last() {
        Some(v) =>
            match *v {
                "html" => ContentType::HTML,
                "js" => ContentType::JavaScript,
                "png" => ContentType::PNG,
                "css" => ContentType::CSS,
                _ => ContentType::Plain
            },
        _ => ContentType::Plain
    }
}

pub fn routes(spec: Spec, mut config: Config) -> Vec<Route> {
    let spec_handler =
        ContentHandler::bytes(
            mime_type(spec.name.as_ref()),
            spec.content.into(),
        );

    let spec_name: &str =
        Path::new(spec.name.as_ref())
            .file_name()
            .unwrap_or("openapi.json".as_ref())
            .to_str()
            .unwrap_or("openapi.json".as_ref());

    config.url = String::from(spec_name);

    let config_handler = ContentHandler::json(&config);

    let mut routes = vec![
        config_handler.into_route(format!("/{}", "swagger-ui-config.json")),
        spec_handler.into_route(format!("/{}", spec_name)),
        RedirectHandler::to("index.html").into_route("/"),
    ];

    for file in Assets::iter() {
        let filename = file.as_ref();
        let mime_type = mime_type(filename);

        let content: Vec<u8> = Assets::get(filename).unwrap().into_owned();

        let path = format!("/{}", filename);
        let handler = ContentHandler::bytes(mime_type, content);
        let route = handler.into_route(path);

        routes.push(route);
    };

    routes
}

#[cfg(test)]
mod tests {
    use rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    fn ignite() -> rocket::Rocket {
        rocket::ignite()
            .mount("/api/v1/swagger/",
                   super::routes(
                       // Specify file with openapi specification,
                       // relative to current file
                       swagger_ui::swagger_spec_file!("../../swagger-ui/examples/openapi.json"),
                       swagger_ui::Config { ..Default::default() },
                   ),
            )
    }

    #[test]
    fn swagger_ui() {
        let client = Client::new(ignite()).expect("valid rocket instance");

        let response = client.get("/api/v1/swagger").dispatch();
        assert_eq!(response.status(), Status::SeeOther);

        let response = client.get("/api/v1/swagger/index.html").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client.get("/api/v1/swagger/swagger-ui-config.json").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let mut response = client.get("/api/v1/swagger/openapi.json").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/../swagger-ui/examples/openapi.json";

        println!("Loading {}", path);

        assert_eq!(
            response.body_string().unwrap(),
            String::from_utf8(std::fs::read(path).unwrap()).unwrap()
        );
    }
}
