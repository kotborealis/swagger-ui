use actix_web::{App, HttpResponse};
use actix_web::web::{get, scope};

use actix_web_swagger_ui;
use swagger_ui;

fn main() {
    let spec = swagger_ui::swagger_spec_file!("../../swagger-ui/examples/openapi.json");
    let config = swagger_ui::Config::default();

    let _app = App::new()
        .service(
            scope("/api/v1/swagger")
                    .configure(actix_web_swagger_ui::swagger(spec, config))
        )
        .route("/index.html", get().to(|| HttpResponse::Ok()));
}