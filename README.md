# swagger-ui

Rust library to render OpenAPI specification file 
using swagger-ui.

## Basic usage

### Rocket

This library provides bindings for `rocket`, 
see [./rocket-swagger-ui/examples/basic.rs](rocket-swagger-ui/examples/basic.rs)
for a full example:

```rust
#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket_swagger_ui;
use swagger_ui;

fn main() {
    rocket::ignite()
        .mount("/api/v1/swagger/",
               rocket_swagger_ui::routes(
                   // Specify file with openapi specification,
                   // relative to current file
                   swagger_ui::swagger_spec_file!("./openapi.json"),
                   swagger_ui::Config { ..Default::default() }
               )
        )
        .launch();
}
```