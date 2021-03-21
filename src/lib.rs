use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$SWAGGER_UI_DIST_PATH"]
pub struct SwaggerUiAssets;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::SwaggerUiAssets;

    fn asset_list() -> [&'static str; 8] {
        [
            "favicon-16x16.png",
            "favicon-32x32.png",
            "index.html",
            "oauth2-redirect.html",
            "swagger-ui.css",
            "swagger-ui.js",
            "swagger-ui-bundle.js",
            "swagger-ui-standalone-preset.js",
        ]
    }

    #[test]
    fn swagger_ui_dist_exists() {
        let dist = env!("SWAGGER_UI_DIST_PATH");
        println!("Checking if dist env var is set ({})", dist);
        assert!(!dist.is_empty());

        println!("Checking if assets exists");
        for file in &asset_list() {
            let asset = format!("{}/{}", dist, file);
            println!("\t{}", asset);
            assert!(Path::new(&asset).exists());
        }
    }

    #[test]
    fn swagger_ui_assets() {
        println!("Checking if assets exists in binary");
        for asset in &asset_list() {
            println!("\t{}", asset);
            let data = SwaggerUiAssets::get(&asset).unwrap();
            assert!(!data.is_empty());
        }
    }
}
