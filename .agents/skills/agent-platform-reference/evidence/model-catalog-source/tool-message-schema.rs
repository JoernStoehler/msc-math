  583 /// Model-owned tool messages and indirect namespace guidance.
  584 #[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, TS, JsonSchema)]
  585 pub struct ToolMessages {
  586     /// Optional guidance for indirectly presented tools; missing or empty adds no prefix.
  587     #[serde(default, skip_serializing_if = "Option::is_none")]
  588     pub indirect_description_prefixes: Option<IndirectDescriptionPrefixes>,
  589     #[serde(default, skip_serializing_if = "Option::is_none")]
  590     pub send_user_message_async: Option<ToolMessage>,
  591     #[serde(default, skip_serializing_if = "Option::is_none")]
  592     pub multi_agent: Option<MultiAgentToolMessages>,
  593     #[serde(default, skip_serializing_if = "Option::is_none")]
  594     pub code_mode: Option<CodeModeToolMessages>,
  595     #[serde(default, skip_serializing_if = "Option::is_none")]
  596     pub mcp_resources: Option<McpResourceToolMessages>,
  597 }
  598 
  599 /// Plain-text prefixes for Code Mode documentation, ALL_TOOLS, and loaded tool-search namespaces.
  600 /// Values are trimmed, and selectors for the same final namespace must agree, including empty values.
  601 /// Empty values add no prefix; unregistered targets are ignored.
  602 #[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, TS, JsonSchema)]
  603 pub struct IndirectDescriptionPrefixes {
  604     /// Exact rendered namespaces, including `functions` for plain tools.
  605     /// Overlap with an MCP server prefix is allowed only when the trimmed values match.
  606     #[serde(default, skip_serializing_if = "Option::is_none")]
  607     pub namespaces: Option<std::collections::BTreeMap<String, String>>,
  608     /// Configured MCP server names, before callable namespace normalization.
  609     /// Each prefix applies to all namespaces exposed by that server.
  610     #[serde(default, skip_serializing_if = "Option::is_none")]
  611     pub mcp_servers: Option<std::collections::BTreeMap<String, String>>,
  612 }
  613 
  614 /// Model-owned messages for a built-in tool.
  615 #[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, TS, JsonSchema)]
  616 pub struct ToolMessage {
  617     /// Missing or null uses the built-in description; an empty string suppresses its static
  618     /// text without disabling the tool. Tool-owned runtime guidance is retained.
  619     #[serde(default, skip_serializing_if = "Option::is_none")]
  620     pub description: Option<String>,
  621     /// Complete JSON Schema encoded as a string. Consumed by Multi-Agent V2 tools, Code Mode wait,
  622     /// request_user_input_async (the send_user_message_async catalog key), and MCP resource helpers.
  623     /// Uses the harness's supported schema subset; unrecognized keywords are ignored.
  624     /// Missing, null, invalid or unsupported structures, or a root without `type: "object"`
  625     /// retains the harness parameters. Schema semantics must remain API-compatible.
  626     /// Overrides must declare harness-encrypted properties so their annotations can be retained.
  627     /// Argument handling is unchanged.
  628     #[serde(default, skip_serializing_if = "Option::is_none")]
  629     pub parameters: Option<String>,
  630 }
