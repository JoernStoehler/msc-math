   19     pub(super) fn add_async_questions(
   20         &mut self,
   21         message_id: &str,
   22         questions: &[AsyncUserInputQuestion],
   23     ) {
   24         let previous_count = self.bottom_pane.question_editor().unanswered_count();
   25         self.bottom_pane.push_async_questions(message_id, questions);
   26         let added_count = self.bottom_pane.question_editor().unanswered_count() - previous_count;
   27         if added_count > 0 {
   28             let title = match questions {
   29                 [question] if !question.title.trim().is_empty() => {
   30                     truncate_text(question.title.trim(), /*max_graphemes*/ 30)
   31                 }
   32                 _ if added_count == 1 => "Question requested".to_string(),
   33                 _ => format!("{added_count} questions requested"),
   34             };
   35             self.notify(Notification::AsyncQuestion { title });
   36         }
   37         self.refresh_pending_input_preview();
