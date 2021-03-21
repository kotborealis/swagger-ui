use swagger_ui::{Assets, Config, Spec, DefaultModelRendering, DocExpansion, Filter, swagger_spec_file};

fn main() {
    println!("swagger-ui bundles files:");
    // Use Assets::iter() to get iterator of all filenames
    for file in Assets::iter() {
        let filename = file.as_ref();
        println!("\t{}", filename);
        // `Assets::get(filename)` returns file content
    };

    // Load openapi spec (compile-time)
    let _spec: Spec = swagger_spec_file!("./openapi.json");

    // swagger-ui configuration struct
    let _config: Config = Config {
        url: "".to_string(),
        urls: vec![],
        deep_linking: false,
        display_operation_id: false,
        default_models_expand_depth: 0,
        default_model_expand_depth: 0,
        default_model_rendering: DefaultModelRendering::Example,
        display_request_duration: false,
        doc_expansion: DocExpansion::List,
        filter: Filter::Bool(false),
        max_displayed_tags: 0,
        show_extensions: false,
        show_common_extensions: false
    };
}