use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@modelcontextprotocol/server-brave-search";
const PACKAGE_VERSION: &str = "0.6.2";
const SERVER_PATH: &str = "node_modules/@modelcontextprotocol/server-brave-search/dist/index.js";

struct BraveSearchContextServerExtension;

#[derive(Debug, Deserialize)]
struct BraveSearchContextServerSettings {
    brave_api_key: String,
}

impl zed::Extension for BraveSearchContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("brave-search-context-server", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `brave_search_api_key` setting".into());
        };
        let settings: BraveSearchContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: "node".to_string(),
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![("BRAVE_API_KEY".into(), settings.brave_api_key)],
        })
    }
}

zed::register_extension!(BraveSearchContextServerExtension);
