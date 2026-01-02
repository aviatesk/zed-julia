use zed::LanguageServerId;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    settings::LspSettings,
    CodeLabel, CodeLabelSpan, Result,
};

struct JuliaExtension;

impl JuliaExtension {
    fn label_for_completion_impl(&self, completion: Completion) -> Option<CodeLabel> {
        let label = &completion.label;
        let label_len = label.len();

        // For certain kinds, use explicit highlight names instead of Tree-sitter
        let label_span = match completion.kind {
            Some(CompletionKind::Struct | CompletionKind::TypeParameter | CompletionKind::Module) => {
                CodeLabelSpan::literal(label, Some("type".to_string()))
            }
            Some(CompletionKind::Function) => {
                CodeLabelSpan::literal(label, Some("function.call".to_string()))
            }
            Some(CompletionKind::Constant) => {
                CodeLabelSpan::literal(label, Some("constant".to_string()))
            }
            Some(CompletionKind::Variable) => {
                CodeLabelSpan::literal(label, Some("variable".to_string()))
            }
            Some(CompletionKind::Keyword) => {
                CodeLabelSpan::literal(label, Some("keyword".to_string()))
            }
            None => CodeLabelSpan::literal(label, None),
            _ => CodeLabelSpan::code_range(zed::Range {
                start: 0,
                end: label_len as u32,
            }),
        };

        let code = label.clone();

        let mut spans = vec![label_span];
        // Add detail (e.g., "::Type") with no highlight (will get fade_out)
        if let Some(detail) = completion
            .label_details
            .as_ref()
            .and_then(|d| d.detail.as_ref())
        {
            spans.push(CodeLabelSpan::literal(detail, None));
        }
        // Add description (e.g., "local", "method") with no highlight (will get fade_out)
        if let Some(desc) = completion
            .label_details
            .as_ref()
            .and_then(|d| d.description.as_ref())
        {
            spans.push(CodeLabelSpan::literal(format!(" {}", desc), None));
        }
        Some(CodeLabel {
            code,
            spans,
            filter_range: zed::Range {
                start: 0,
                end: label_len as u32,
            },
        })
    }
}

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

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        self.label_for_completion_impl(completion)
    }
}

zed::register_extension!(JuliaExtension);
