use zed_extension_api as zed;

struct CangjieExtension {
    // state...
}

impl zed::Extension for CangjieExtension {
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Command> {
        Err("`language_server_command` not implemented".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_additional_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _target_language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _target_language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _completion: Completion,
    ) -> Option<CodeLabel> {
        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _symbol: Symbol,
    ) -> Option<CodeLabel> {
        None
    }

    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
    ) -> zed_extension_api::Result<Vec<SlashCommandArgumentCompletion>, String> {
        Ok(Vec::new())
    }

    fn run_slash_command(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> zed_extension_api::Result<SlashCommandOutput, String> {
        Err("`run_slash_command` not implemented".to_string())
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &Project,
    ) -> zed_extension_api::Result<Command> {
        Err("`context_server_command` not implemented".to_string())
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &Project,
    ) -> zed_extension_api::Result<Option<ContextServerConfiguration>> {
        Ok(None)
    }

    fn suggest_docs_packages(
        &self,
        _provider: String,
    ) -> zed_extension_api::Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    fn index_docs(
        &self,
        _provider: String,
        _package: String,
        _database: &KeyValueStore,
    ) -> zed_extension_api::Result<(), String> {
        Err("`index_docs` not implemented".to_string())
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<DebugAdapterBinary, String> {
        Err("`get_dap_binary` not implemented".to_string())
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: zed_extension_api::serde_json::Value,
    ) -> zed_extension_api::Result<StartDebuggingRequestArgumentsRequest, String> {
        Err("`dap_request_kind` not implemented".to_string())
    }

    fn dap_config_to_scenario(
        &mut self,
        _config: DebugConfig,
    ) -> zed_extension_api::Result<DebugScenario, String> {
        Err("`dap_config_to_scenario` not implemented".to_string())
    }

    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<DebugScenario> {
        None
    }

    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
    ) -> zed_extension_api::Result<DebugRequest, String> {
        Err("`run_dap_locator` not implemented".to_string())
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    // methods...
}

zed::register_extension!(MyExtension);
