   95                 session,
   96                 turn,
   97                 call_id,
   98                 payload,
   99                 ..
  100             } = invocation;
  101             let ToolPayload::Function { arguments } = payload else {
  102                 return Err(FunctionCallError::RespondToModel(format!(
  103                     "{TOOL_NAME} handler received unsupported payload"
  104                 )));
  105             };
  106             let args: RequestUserInputAsyncArgs = parse_arguments(&arguments)?;
  107             if args.questions.is_empty() {
  108                 return Err(FunctionCallError::RespondToModel(
  109                     "questions must not be empty".to_string(),
  110                 ));
  111             }
  112             let mut messages = Vec::with_capacity(args.questions.len());
  113             for question in &args.questions {
  114                 if question.title.trim().is_empty() {
  115                     return Err(FunctionCallError::RespondToModel(
  116                         "question titles must not be empty".to_string(),
  117                     ));
  118                 }
  119                 let mut lines = vec![question.title.clone()];
  120                 if let Some(options) = &question.options {
  121                     if options.is_empty() || options.iter().any(|option| option.trim().is_empty()) {
  122                         return Err(FunctionCallError::RespondToModel(
  123                             "options must contain at least one non-empty answer".to_string(),
  124                         ));
  125                     }
  126                     lines.extend(options.iter().map(|option| format!("- {option}")));
  127                 }
  128                 messages.push(lines.join("\n"));
  129             }
  130 
  131             let item = TurnItem::AgentMessage(AgentMessageItem {
  132                 id: call_id,
  133                 content: vec![AgentMessageContent::Text {
  134                     text: messages.join("\n\n"),
  135                 }],
  136                 phase: Some(MessagePhase::FinalAnswer),
  137                 memory_citation: None,
  138                 delivery: Some(AgentMessageDelivery::Async),
  139                 questions: Some(args.questions),
  140             });
  141             session.emit_turn_item_started(turn.as_ref(), &item).await;
  142             session.emit_turn_item_completed(turn.as_ref(), item).await;
  143 
  144             Ok(boxed_tool_output(FunctionToolOutput::from_text(
