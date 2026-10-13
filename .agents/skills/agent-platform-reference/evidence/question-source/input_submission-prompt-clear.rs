  480         }
  481         let render_before_submit =
  482             render_in_history && matches!(&self.codex_op_target, CodexOpTarget::AppEvent);
  483         if render_before_submit {
  484             self.on_user_message_display(user_message_display_for_history(
  485                 submitted_message.clone(),
  486                 &history_record,
  487             ));
  488         }
  489 
  490         if !self.submit_op(op.clone()) {
  491             return (false, None);
  492         }
  493         if source == UserMessageSource::Prompt {
  494             self.bottom_pane.clear_pending_questions();
  495         }
  496         self.dismiss_backend_banner_for_new_turn();
  497         self.note_realtime_typed_input(&submitted_message.text);
  498         if render_in_history {
  499             self.input_queue.user_turn_pending_start = true;
  500             self.input_queue.pending_user_message_client_id = Some(client_user_message_id.clone());
  501         }
