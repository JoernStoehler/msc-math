  415     pub(super) fn on_agent_message_item_completed(
  416         &mut self,
  417         item: AgentMessageItem,
  418         turn_id: &str,
  419         from_replay: bool,
  420     ) {
  421         if !from_replay && let Some(questions) = &item.questions {
  422             self.add_async_questions(&item.id, questions);
  423         }
  424         self.transcript.last_completed_agent_message = Some((turn_id.to_string(), item.id.clone()));
