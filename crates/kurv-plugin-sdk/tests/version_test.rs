use kurv_plugin_sdk::{PluginMetadata, plugin_metadata};

#[test]
fn test_plugin_metadata_macro_uses_call_site_package_metadata() {
    let metadata = plugin_metadata!();

    assert_eq!(metadata.name, env!("CARGO_PKG_NAME"));
    assert_eq!(metadata.version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_plugin_metadata_display_matches_expected_format() {
    let metadata = PluginMetadata {
        name: "kurv-ui",
        version: "0.1.2",
    };

    assert_eq!(metadata.to_string(), "kurv-ui@0.1.2");
}
