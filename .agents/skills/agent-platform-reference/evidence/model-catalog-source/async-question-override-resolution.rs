  150 
  151     /// Resolves the asynchronous user-input tool description.
  152     pub fn request_user_input_async_description(&self) -> &'a str {
  153         self.catalog_messages
  154             .and_then(|messages| messages.tools.as_ref())
  155             .and_then(|tools| tools.send_user_message_async.as_ref())
  156             .and_then(|tool| tool.description.as_deref())
  157             .unwrap_or(REQUEST_USER_INPUT_ASYNC_DESCRIPTION)
  158     }
  159 
  160     /// Selects the asynchronous user-input schema; parsing belongs to the tool consumer.
  161     pub fn request_user_input_async_parameters_override(&self) -> Option<&'a str> {
  162         self.catalog_messages?
  163             .tools
  164             .as_ref()?
  165             .send_user_message_async
  166             .as_ref()?
  167             .parameters
  168             .as_deref()
  169     }
  170 
  171     /// Selects a V2 tool's static description by its name, independently of its runtime namespace.
  172     /// Missing text retains the tool's bundled description; an empty string replaces it.
  173     pub fn multi_agent_tool_description_override(&self, tool_name: &str) -> Option<&'a str> {
  174         self.multi_agent_tool(tool_name)?.description.as_deref()
  175     }
  176 
  177     /// Selects a V2 tool's complete parameter schema; parsing belongs to the tool consumer.
  178     pub fn multi_agent_tool_parameters_override(&self, tool_name: &str) -> Option<&'a str> {
  179         self.multi_agent_tool(tool_name)?.parameters.as_deref()
  180     }
  181 
  182     fn multi_agent_tool(self, tool_name: &str) -> Option<&'a ToolMessage> {
  183         self.catalog_messages
  184             .and_then(|messages| messages.tools.as_ref())
  185             .and_then(|tools| tools.multi_agent.as_ref())?
  186             .by_name(tool_name)
  187     }
