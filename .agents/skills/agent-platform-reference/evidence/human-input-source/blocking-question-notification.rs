  449     pub(crate) fn handle_request_user_input_now(&mut self, ev: ToolRequestUserInputParams) {
  450         self.flush_answer_stream_with_separator();
  451         let question_count = ev.questions.len();
  452         let summary = Notification::user_input_request_summary(&ev.questions);
  453         let title = match (question_count, summary.as_deref()) {
  454             (1, Some(summary)) => summary.to_string(),
  455             (1, None) => "Question requested".to_string(),
  456             (count, _) => format!("{count} questions requested"),
  457         };
  458         self.notify(Notification::PlanModePrompt { title });
  459         self.bottom_pane.push_user_input_request(ev);
  460         self.set_ambient_pet_notification(
  461             crate::pets::PetNotificationKind::Waiting,
  462             /*body*/ None,
  463         );
  464         self.request_redraw();
  465     }
