   25 pub fn create_create_goal_tool() -> ToolSpec {
   26     let properties = BTreeMap::from([
   27         (
   28             "objective".to_string(),
   29             JsonSchema::string(Some(
   30                 "Required. The concrete objective to start pursuing. This starts a new active goal when no goal exists or replaces the current goal when it is complete."
   31                     .to_string(),
   32             )),
   33         ),
   34         (
   35             "token_budget".to_string(),
   36             JsonSchema::integer(Some(
   37                 "Positive token budget for the new goal. Omit unless explicitly requested."
   38                     .to_string(),
   39             )),
   40         ),
   41     ]);
   42 
   43     ToolSpec::Function(ResponsesApiTool {
   44         name: CREATE_GOAL_TOOL_NAME.to_string(),
   45         description: format!(
   46             r#"Create a goal only when explicitly requested by the user or system/developer instructions; do not infer goals from ordinary tasks.
   47 Set token_budget only when an explicit token budget is requested. Fails if an unfinished goal exists; use {UPDATE_GOAL_TOOL_NAME} only for status."#
   48         ),
   49         strict: false,
   50         defer_loading: None,
   51         parameters: JsonSchema::object(
   52             properties,
   53             /*required*/ Some(vec!["objective".to_string()]),
   54             Some(false.into()),
   55         ),
   56         output_schema: None,
   57     })
   58 }
   59 
   60 pub fn create_update_goal_tool() -> ToolSpec {
   61     let properties = BTreeMap::from([(
   62         "status".to_string(),
   63         JsonSchema::string_enum(
   64             vec![json!("complete"), json!("blocked"), json!("paused")],
   65             Some(
   66                 "Required. `paused` requires an explicit user request. Set to `complete` only when the objective is achieved and no required work remains. Set to `blocked` only after the same blocking condition has recurred for at least three consecutive goal turns and the agent is at an impasse. After a previously blocked goal is resumed, the resumed run starts a fresh blocked audit."
   67                     .to_string(),
   68             ),
   69         ),
   70     )]);
   71 
   72     ToolSpec::Function(ResponsesApiTool {
   73         name: UPDATE_GOAL_TOOL_NAME.to_string(),
   74         description: r#"Update the existing goal.
   75 Set status to `paused` only at the user's explicit request to pause this goal, never on your own initiative. Ask if unclear; a later resume revokes that request. Report the returned status and stop goal work. Budget limits take precedence over pausing.
   76 Set status to `complete` only when the objective has actually been achieved and no required work remains.
   77 Set status to `blocked` only when the same blocking condition has repeated for at least three consecutive goal turns, counting the original/user-triggered turn and any automatic continuations, and the agent cannot make meaningful progress without user input or an external-state change.
   78 If the user resumes a goal that was previously marked `blocked`, treat the resumed run as a fresh blocked audit. If the same blocking condition then repeats for at least three consecutive resumed goal turns, set status to `blocked` again.
   79 Once the blocked threshold is satisfied, do not keep reporting that you are still blocked while leaving the goal active; set status to `blocked`.
   80 Do not use `blocked` merely because the work is hard, slow, uncertain, incomplete, or would benefit from clarification.
   81 Do not mark a goal complete merely because its budget is nearly exhausted or because you are stopping work.
   82 You cannot use this tool to resume, budget-limit, or usage-limit a goal; those status changes are controlled by the user or system.
   83 When marking a budgeted goal achieved with status `complete`, report the final token usage from the tool result to the user."#
   84             .to_string(),
   85         strict: false,
   86         defer_loading: None,
   87         parameters: JsonSchema::object(
   88             properties,
   89             /*required*/ Some(vec!["status".to_string()]),
   90             Some(false.into()),
   91         ),
   92         output_schema: None,
   93     })
   94 }
