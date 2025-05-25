use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct JuliaExtension;

impl zed::Extension for JuliaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree("JETLS", worktree)?;

        let jetls_bin = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.path.as_ref())
            .map(|s| s.as_str())
            .unwrap_or_else(|| {
                // Use platform-specific executable name
                if cfg!(windows) {
                    "jetls.exe"
                } else {
                    "jetls"
                }
            });

        // Resolve the binary path from PATH if it's just a command name
        let resolved_bin = if jetls_bin.contains('/') || jetls_bin.contains('\\') {
            // It's a path (absolute or relative), use as-is
            jetls_bin.to_string()
        } else {
            // It's just a command name, resolve from PATH
            worktree
                .which(jetls_bin)
                .ok_or_else(|| format!("'{}' not found in PATH. Please install JETLS as a Julia app or specify the full path in settings.", jetls_bin))?
        };

        // Check if binary.arguments is provided (custom arguments)
        let args = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.arguments.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                // Default arguments for `jetls` command
                vec!["--threads=auto".to_string(), "--".to_string()]
            });

        // Use environment variables from settings if provided (for `JULIA_APPS_JULIA_CMD` in particular)
        let env = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.env.clone())
            .map(|env_map| env_map.into_iter().collect())
            .unwrap_or_default();

        Ok(zed::Command {
            command: resolved_bin,
            args,
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree("JETLS", worktree)
            .ok()
            .and_then(|s| s.initialization_options.clone());
        Ok(initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("JETLS", worktree)
            .ok()
            .and_then(|s| s.settings.clone());
        Ok(settings)
    }
}

zed::register_extension!(JuliaExtension);
