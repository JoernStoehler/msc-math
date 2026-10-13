   67         if turn.session_source.is_non_root_agent() {
   68             return Err(FunctionCallError::RespondToModel(
   69                 "request_user_input can only be used by the root thread".to_string(),
   70             ));
   71         }
   72 
   73         let mode = turn.collaboration_mode().mode;
   74         if let Some(message) = request_user_input_unavailable_message(mode, &self.available_modes) {
   75             return Err(FunctionCallError::RespondToModel(message));
   76         }
   77 
   78         let args: RequestUserInputToolArgs = parse_arguments(&arguments)?;
   79         let args = normalize_request_user_input_tool_args(args)
   80             .map_err(FunctionCallError::RespondToModel)?;
   81         let args = RequestUserInputArgs {
   82             questions: args.questions,
   83             is_blocking: mode == ModeKind::Plan,
   84             auto_resolution_ms: None,
   85         };
   86         let questions = args.questions.clone();
   87         let accepted = session
   88             .request_user_input(turn.as_ref(), call_id.clone(), args)
   89             .await
   90             .ok_or_else(|| {
   91                 FunctionCallError::RespondToModel(format!(
   92                     "{REQUEST_USER_INPUT_TOOL_NAME} was cancelled before receiving a response"
   93                 ))
   94             })?;
   95 
   96         let response = accepted.response;
   97 
   98         let content = serde_json::to_string(&response).map_err(|err| {
   99             FunctionCallError::Fatal(format!(
  100                 "failed to serialize {REQUEST_USER_INPUT_TOOL_NAME} response: {err}"
