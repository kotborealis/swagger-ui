extern crate reqwest;

use std::io::Write;
use std::fs::File;
use std::fs::remove_file;
use std::{env, fs};
use flate2::read::GzDecoder;
use tar::Archive;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("cargo:rerun-if-changed=build.rs");

    let swagger_ui_version = "3.45.1";
    let dist = download_dist(swagger_ui_version).await?;
    let dist = unpack_dist(dist);
    patch_index(&dist);
    remove_unused_files(&dist);

    println!("cargo:rustc-env=SWAGGER_UI_DIST_PATH={}", dist);

    Ok(())
}

/// Download swagger-ui-dist from npm
/// using specified version
async fn download_dist(swagger_ui_version: &str) -> Result<String, reqwest::Error>  {
    let target =
        format!(
            "https://registry.npmjs.org/swagger-ui-dist/-/swagger-ui-dist-{}.tgz",
            swagger_ui_version
        );

    let resp = reqwest::get(&target).await?;
    let dist = env::var("OUT_DIR").unwrap() + "/swagger-ui-dist.tgz";
    let mut out = File::create(&dist).expect("failed to create file");
    out.write_all(&*(resp.bytes().await?)).unwrap();
    Ok(dist)
}

/// Unpack swagger-ui-dist tgz
fn unpack_dist(dist: String) -> String {
    let dist = File::open(dist).unwrap();
    let dist = GzDecoder::new(dist);
    let mut dist = Archive::new(dist);
    let unpack_path = env::var("OUT_DIR").unwrap() + "/swagger-ui-dist/";
    dist.unpack(&unpack_path).unwrap();
    let dist = unpack_path + "/package";
    dist
}

/// Remove unused files from swagger-ui-dist
///
/// Removes node.js modules, package.json,
/// readme.md, *.map files and es bundles
fn remove_unused_files(dist: &String) {
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
}

/// Patches index.html from swagger-ui-dist
/// to load our config file
fn patch_index(dist: &String) {
    let path = format!("{}/index.html", dist);
    let content = fs::read_to_string(&path).unwrap();
    let content = content.replace(
        "url: \"https://petstore.swagger.io/v2/swagger.json\"",
        &*format!("configUrl: \"swagger-ui-config.json\"")
    );
    fs::write(&path, content).unwrap();
}