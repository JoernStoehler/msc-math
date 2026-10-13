 3088     pub(crate) async fn request_user_input(
 3089         &self,
 3090         turn_context: &TurnContext,
 3091         call_id: String,
 3092         args: RequestUserInputArgs,
 3093     ) -> Option<AcceptedUserInputResponse> {
 3094         let _elicitation = self.services.elicitations.register();
 3095         let sub_id = turn_context.sub_id.clone();
 3096         let (tx_response, rx_response) = oneshot::channel();
 3097         let event_id = sub_id.clone();
 3098         let prev_entry = {
 3099             let mut active = self.active_turn.lock().await;
 3100             match active.as_mut() {
 3101                 Some(at) => {
 3102                     let mut ts = at.turn_state.lock().await;
 3103                     ts.insert_pending_user_input(sub_id, tx_response)
 3104                 }
 3105                 None => None,
 3106             }
 3107         };
 3108         if prev_entry.is_some() {
 3109             warn!("Overwriting existing pending user input for sub_id: {event_id}");
 3110         }
 3111 
 3112         let event = EventMsg::RequestUserInput(RequestUserInputEvent {
 3113             call_id,
 3114             turn_id: turn_context.sub_id.clone(),
 3115             questions: args.questions,
 3116             is_blocking: args.is_blocking,
 3117             auto_resolution_ms: args.auto_resolution_ms,
 3118         });
 3119         turn_context
 3120             .turn_metadata_state
 3121             .mark_user_input_requested_during_turn();
 3122         self.send_event(turn_context, event).await;
 3123         rx_response.await.ok()
 3124     }
