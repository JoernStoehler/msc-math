  274         let replay_kind = render_source.replay_kind();
  275         match item {
  276             ThreadItem::UserMessage {
  277                 content, client_id, ..
  278             } => {
  279                 if let Some(replies) = crate::async_question_reply::parse_input(&content) {
  280                     let ids = replies
  281                         .into_iter()
  282                         .map(|reply| reply.question_item_id)
  283                         .collect::<Vec<_>>();
  284                     self.bottom_pane.question_editor().resolve_answers(&ids);
  285                     self.refresh_pending_input_preview();
  286                     self.request_redraw();
  287                 }
  288                 self.on_committed_user_message(
  289                     &content,
  290                     client_id.as_deref(),
  291                     from_replay,
  292                     &turn_id,
  293                 );
  294             }
  295             ThreadItem::AgentMessage {
  296                 id,
  297                 text,
  298                 phase,
  299                 memory_citation,
  300                 delivery,
  301                 questions,
  302                 ..
