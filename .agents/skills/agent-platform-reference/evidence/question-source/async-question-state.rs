    1 //! Pending async questions retain drafts until handled locally, answered by another client, or a live turn ends.
    2 //! Message IDs survive removal so replay cannot reopen an answered or skipped question.
    3 
    4 use super::*;
    5 use crate::history_cell::sanitize_user_text;
    6 use codex_context_fragments::AnsweredQuestion;
    7 use codex_context_fragments::ContextualUserFragment;
    8 use codex_utils_string::take_bytes_at_char_boundary;
    9 
   10 // Match the model-authored title budget used by AnsweredQuestion.
   11 const MAX_RECOVERED_QUESTION_TITLE_BYTES: usize = 512;
   12 
   13 impl AsyncQuestions {
   14     pub(crate) fn append(&mut self, message_id: &str, questions: &[AsyncUserInputQuestion]) {
   15         if questions.is_empty() || !self.state.seen_ids.insert(message_id.to_string()) {
   16             return;
   17         }
   18         let was_empty = self.state.pending.is_empty();
   19         let expires_at = (!self.expanded).then(|| Instant::now() + Duration::from_secs(30));
   20         for (index, question) in questions.iter().enumerate() {
   21             // Match the desktop's JSON.stringify([tool name, item id, question index]).
   22             let question_id =
   23                 serde_json::json!(["request_user_input_async", message_id, index]).to_string();
   24             if self.state.answered_ids.contains(&question_id)
   25                 || self.state.answered_ids.contains(message_id)
   26             {
   27                 continue;
   28             }
   29             // Bound work before cloning or wrapping model-authored suggestions.
   30             let question = AsyncUserInputQuestion {
   31                 title: question.title.clone(),
   32                 options: question.options.as_ref().map(|options| {
   33                     options
   34                         .iter()
   35                         .take(32)
   36                         .filter(|label| label.len() <= 512)
   37                         .cloned()
   38                         .collect()
   39                 }),
   40             };
   41             let has_options = question
   42                 .options
   43                 .as_ref()
   44                 .is_some_and(|options| !options.is_empty());
   45             let mut options_state = ScrollState::new();
   46             options_state.selected_idx = has_options.then_some(0);
   47             self.state.pending.push(PendingQuestion {
   48                 message_id: message_id.into(),
   49                 question_id,
   50                 question,
   51                 options_state,
   52                 draft: ComposerDraft::default(),
   53                 expires_at,
   54             });
   55         }
   56         if was_empty {
   57             self.state.current_idx = 0;
   58             self.restore_current_draft();
   59         }
   60     }
   61 
   62     pub(crate) fn set_expanded(&mut self, expanded: bool) {
   63         self.save_current_draft();
   64         self.expanded = expanded && !self.state.pending.is_empty();
   65         if self.expanded {
   66             self.snooze_auto_resolution();
   67         }
   68     }
   69 
   70     pub(crate) fn navigate(&mut self, forward: bool) -> bool {
   71         let next = if forward {
   72             self.state.current_idx.checked_add(1)
   73         } else {
   74             self.state.current_idx.checked_sub(1)
   75         };
   76         let Some(next) = next.filter(|&index| index < self.state.pending.len()) else {
   77             return false;
   78         };
   79         self.save_current_draft();
   80         self.visible_options.set((0, 0));
   81         self.state.current_idx = next;
   82         self.restore_current_draft();
   83         true
   84     }
   85 
   86     pub(super) fn snooze_auto_resolution(&mut self) {
   87         for question in &mut self.state.pending {
   88             question.expires_at = None;
   89         }
   90     }
   91 
   92     pub(super) fn timer_remaining(&self, now: Instant) -> Option<Duration> {
   93         self.state
   94             .pending
   95             .iter()
   96             .filter_map(|question| question.expires_at?.checked_duration_since(now))
   97             .filter(|remaining| !remaining.is_zero())
   98             .min()
   99     }
  100 
  101     pub(crate) fn countdown(&self, now: Instant) -> Option<String> {
  102         self.timer_remaining(now)
  103             .filter(|remaining| *remaining <= Duration::from_secs(20))
  104             .map(|remaining| format!("{}s", remaining.as_secs_f64().ceil() as u64))
  105     }
  106 
  107     pub(super) fn go_next_or_submit(&mut self) {
  108         self.save_current_draft();
  109         if !self.delivery_enabled {
  110             return;
  111         }
