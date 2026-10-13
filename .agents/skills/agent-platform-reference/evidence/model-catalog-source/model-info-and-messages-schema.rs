  404 /// Model metadata returned by the Codex backend `/models` endpoint.
  405 #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, TS, JsonSchema)]
  406 pub struct ModelInfo {
  407     /// Model-owned approval coverage. Absent preserves legacy settings; an empty map disables
  408     /// ordinary Guardian review. Keys are computer_use, shell, file_changes, mcp, network,
  409     /// and permissions. This does not override mandatory safety or administrator requirements.
  410     #[serde(default, skip_serializing_if = "Option::is_none")]
  411     pub guardian: Option<GuardianModelPolicy>,
  412     pub slug: String,
  413     pub display_name: String,
  414     pub description: Option<String>,
  415     #[serde(default, skip_serializing_if = "Option::is_none")]
  416     pub default_reasoning_level: Option<ReasoningEffort>,
  417     pub supported_reasoning_levels: Vec<ReasoningEffortPreset>,
  418     pub shell_type: ConfigShellToolType,
  419     pub visibility: ModelVisibility,
  420     pub supported_in_api: bool,
  421     pub priority: i32,
  422     #[serde(default)]
  423     pub additional_speed_tiers: Vec<String>,
  424     #[serde(default)]
  425     pub service_tiers: Vec<ModelServiceTier>,
  426     #[serde(default, skip_serializing_if = "Option::is_none")]
  427     pub default_service_tier: Option<String>,
  428     #[serde(default, skip_serializing_if = "Option::is_none")]
  429     pub available_access_programs: Option<ModelAccessPrograms>,
  430     pub availability_nux: Option<ModelAvailabilityNux>,
  431     pub upgrade: Option<ModelInfoUpgrade>,
  432     #[serde(default, skip_serializing_if = "Option::is_none")]
  433     pub model_messages: Option<ModelMessages>,
  434     #[serde(default)]
  435     pub include_skills_usage_instructions: bool,
