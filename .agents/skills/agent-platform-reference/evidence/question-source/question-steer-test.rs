  110 }
  111 
  112 #[tokio::test]
  113 async fn ordinary_follow_up_clears_unanswered_questions_after_accepted_input() {
  114     for queued in [false, true] {
  115         let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(/*model_override*/ None).await;
  116         chat.thread_id = Some(ThreadId::new());
  117         chat.show_welcome_banner = false;
  118         chat.add_async_questions("old", &questions());
  119         if queued {
  120             chat.on_task_started();
  121         }
  122         chat.bottom_pane
  123             .set_composer_text("New prompt".into(), Vec::new(), Vec::new());
  124         chat.handle_key_event(KeyEvent::from(if queued {
  125             KeyCode::Tab
  126         } else {
  127             KeyCode::Enter
  128         }));
  129 
  130         assert_eq!(question_count(&chat), 0);
  131         if queued {
  132             assert_eq!(
  133                 chat.input_queue.queued_user_messages.front().unwrap().text,
  134                 "New prompt"
  135             );
  136             assert!(op_rx.try_recv().is_err());
  137         } else {
  138             assert_answer(op_rx.try_recv().unwrap(), "New prompt");
  139             insta::assert_snapshot!(
  140                 "questions_cleared_by_follow_up",
  141                 render_bottom_popup(&chat, /*width*/ 80)
  142                     .lines()
  143                     .next()
  144                     .unwrap()
  145             );
  146         }
  147         chat.add_async_questions("old", &questions());
  148         assert_eq!(question_count(&chat), 0);
  149         chat.add_async_questions("new", &[question("New question?", /*options*/ None)]);
  150         assert_eq!(question_count(&chat), 1);
  151     }
  152 }
  153 
  154 #[tokio::test]
  155 async fn local_shell_command_preserves_unanswered_questions() {
