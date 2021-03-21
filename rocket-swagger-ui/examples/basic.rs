#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket_swagger_ui;
use swagger_ui;

fn main() {
    rocket::ignite()
        .mount("/api/v1/swagger/",
               rocket_swagger_ui::routes(
                   swagger_ui::swagger_spec_file!("./openapi.json"),
                   swagger_ui::Config { ..Default::default() }
               )
        )
        .launch();
}