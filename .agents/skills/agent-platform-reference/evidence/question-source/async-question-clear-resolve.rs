  145                 Duration::from_secs(5),
  146             );
  147         } else {
  148             self.submission = Some(QuestionSubmission::Submit(reply));
  149         }
  150     }
  151 
  152     pub(crate) fn resolve_answers(&mut self, question_ids: &[String]) {
  153         // History can arrive before live questions or before restoring local drafts.
  154         self.state.answered_ids.extend(question_ids.iter().cloned());
  155         // Older desktop replies identify the whole source message instead of one question.
  156         let answered = |question: &PendingQuestion| {
  157             question_ids.contains(&question.question_id)
  158                 || question_ids.contains(&question.message_id)
  159         };
  160         if !self.state.pending.iter().any(answered) {
  161             return;
  162         }
  163         let current_answered = self.current_answer().is_some_and(answered);
  164         if current_answered {
  165             self.composer.flush_pending_input();
  166         }
  167         let current_idx = self
  168             .state
  169             .pending
  170             .iter()
  171             .take(self.state.current_idx)
  172             .filter(|question| !answered(question))
  173             .count();
  174         self.state.pending.retain(|question| !answered(question));
  175         self.state.current_idx = if current_idx < self.state.pending.len() {
  176             current_idx
  177         } else {
  178             0
  179         };
  180         self.expanded &= !self.state.pending.is_empty();
  181         self.visible_options.set((0, 0));
  182         if current_answered {
  183             self.restore_current_draft();
  184             self.composer.reset_vim_mode();
  185         }
  186     }
  187 
  188     pub(crate) fn accept_answer(&mut self) {
  189         if self.state.pending.is_empty() {
  190             return;
  191         }
  192         self.composer.flush_pending_input();
  193         self.visible_options.set((0, 0));
  194         self.state.pending.remove(self.state.current_idx);
  195         if self.state.current_idx >= self.state.pending.len() {
  196             self.state.current_idx = 0;
  197         }
  198         self.expanded &= !self.state.pending.is_empty();
  199         self.restore_current_draft();
  200         self.composer.reset_vim_mode();
  201     }
  202 
  203     pub(crate) fn clear_pending(&mut self) {
  204         // Keep seen_ids so replay cannot restore questions from the previous prompt.
  205         self.state.pending.clear();
