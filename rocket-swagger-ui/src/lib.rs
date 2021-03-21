mod handlers;

use rocket::http::{ContentType};
use rocket::{Route};
use crate::handlers::{ContentHandler, RedirectHandler};
use swagger_ui::{SwaggerUiAssets, Config, Spec};
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

fn swagger_config_url() -> String {
    "swagger-ui-config.json".to_string()
}

fn patch_index(content: Vec<u8>) -> Vec<u8> {
    let content = String::from_utf8(content).unwrap();
    Vec::from(content.replace(
        "url: \"https://petstore.swagger.io/v2/swagger.json\"",
        &*format!("configUrl: \"{}\"", swagger_config_url())
    ))
}

pub fn routes(spec: Spec, mut config: Config) -> Vec<Route> {
    let spec_handler =
        ContentHandler::bytes(
            mime_type(spec.name.as_str()),
            Vec::from(spec.content)
        );

    let spec_name: &str =
        Path::new(&spec.name)
            .file_name()
            .unwrap_or("openapi.json".as_ref())
            .to_str()
            .unwrap_or("openapi.json".as_ref());

    config.url = String::from(spec_name);

    let config_handler = ContentHandler::json(&config);

    let mut routes = vec![
        config_handler.into_route(format!("/{}", swagger_config_url())),
        spec_handler.into_route(format!("/{}", spec_name)),
        RedirectHandler::to("index.html").into_route("/"),
    ];

    for file in SwaggerUiAssets::iter() {
        let filename = file.as_ref();
        let mime_type = mime_type(filename);

        let content: Vec<u8> = SwaggerUiAssets::get(filename).unwrap().into_owned();

        // patch index.html to use our config file
        let content = match filename {
            "index.html" => patch_index(content),
            _ => content
        };

        let path = format!("/{}", filename);
        let handler = ContentHandler::bytes(mime_type, content);
        let route = handler .into_route(path);

        routes.push(route);
    };

    routes
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
