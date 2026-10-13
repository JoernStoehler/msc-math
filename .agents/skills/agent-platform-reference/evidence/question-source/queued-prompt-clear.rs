  202                     || action == QueuedInputAction::Plain && !user_message.text.starts_with('!')
  203                     || queued_slash_prompt);
  204             self.input_queue
  205                 .queued_user_messages
  206                 .push_back(QueuedUserMessage {
  207                     user_message,
  208                     action,
  209                     delivery: MessageDelivery::Unsent,
  210                     pending_pastes,
  211                     source,
  212                 });
  213             self.input_queue
  214                 .queued_user_message_history_records
  215                 .push_back(UserMessageHistoryRecord::UserMessageText);
  216             self.refresh_pending_input_preview();
  217             if model_prompt && !should_run_now {
  218                 self.bottom_pane.clear_pending_questions();
  219             }
  220             if should_run_now {
  221                 self.maybe_send_next_queued_input();
  222             }
