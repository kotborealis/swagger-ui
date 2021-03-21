extern crate reqwest;

use std::io::Write;
use std::fs::File;
use std::fs::remove_file;
use std::env;
use flate2::read::GzDecoder;
use tar::Archive;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("cargo:rerun-if-changed=build.rs");

    let swagger_ui_version = "3.45.1";
    let target =
        format!(
            "https://registry.npmjs.org/swagger-ui-dist/-/swagger-ui-dist-{}.tgz",
            swagger_ui_version
        );

    let resp = reqwest::get(&target).await.unwrap();

    let dist = env::var("OUT_DIR").unwrap() + "/swagger-ui-dist.tgz";
    let mut out = File::create(&dist).expect("failed to create file");
    out.write_all(&*(resp.bytes().await.unwrap())).unwrap();

    let dist = File::open(dist).unwrap();
    let dist = GzDecoder::new(dist);
    let mut dist = Archive::new(dist);
    let unpack_path = env::var("OUT_DIR").unwrap() + "/swagger-ui-dist/";
    dist.unpack(&unpack_path).unwrap();
    let dist = unpack_path + "/package";

    let trash = [
        "absolute-path.js",
        "index.js",
        "package.json",
        "README.md",
        "swagger-ui.css.map",
        "swagger-ui.js.map",
        "swagger-ui-bundle.js.map",
        "swagger-ui-es-bundle.js.map",
        "swagger-ui-es-bundle-core.js.map",
        "swagger-ui-standalone-preset.js.map",
        "swagger-ui-es-bundle.js",
        "swagger-ui-es-bundle-core.js"
    ];

    for file in &trash {
        remove_file(format!("{}/{}", dist, file)).unwrap();
    }

    println!("cargo:rustc-env=SWAGGER_UI_DIST_PATH={}", dist);

    Ok(())
}