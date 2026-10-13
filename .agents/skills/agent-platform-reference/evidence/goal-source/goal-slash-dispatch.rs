  965             SlashCommand::Goal if !trimmed.is_empty() => {
  966                 if !self.config.features.enabled(Feature::Goals) {
  967                     if source == SlashCommandDispatchSource::Live {
  968                         self.clear_live_goal_submission();
  969                     }
  970                     return;
  971                 }
  972                 enum GoalControlCommand {
  973                     Clear,
  974                     SetStatus(AppThreadGoalStatus),
  975                 }
  976                 let control_command = match trimmed.to_ascii_lowercase().as_str() {
  977                     "clear" => Some(GoalControlCommand::Clear),
  978                     "edit" => {
  979                         self.app_event_tx.send(AppEvent::OpenThreadGoalEditor {
  980                             thread_id: self.thread_id,
  981                         });
  982                         if source == SlashCommandDispatchSource::Live {
  983                             self.clear_live_goal_submission();
  984                         }
  985                         return;
  986                     }
  987                     "pause" => Some(GoalControlCommand::SetStatus(AppThreadGoalStatus::Paused)),
  988                     "resume" => Some(GoalControlCommand::SetStatus(AppThreadGoalStatus::Active)),
  989                     _ => None,
  990                 };
  991                 if let Some(command) = control_command {
  992                     let Some(thread_id) = self.thread_id else {
  993                         self.add_info_message(
  994                             GOAL_USAGE.to_string(),
  995                             Some(
  996                                 "The session must start before you can change a goal.".to_string(),
  997                             ),
  998                         );
  999                         if source == SlashCommandDispatchSource::Live {
 1000                             self.clear_live_goal_submission();
 1001                         }
 1002                         return;
 1003                     };
 1004                     match command {
 1005                         GoalControlCommand::Clear => {
 1006                             self.app_event_tx
 1007                                 .send(AppEvent::ClearThreadGoal { thread_id });
 1008                         }
 1009                         GoalControlCommand::SetStatus(status) => {
 1010                             self.app_event_tx
 1011                                 .send(AppEvent::SetThreadGoalStatus { thread_id, status });
 1012                         }
 1013                     }
 1014                     self.append_message_history_entry(format!("/goal {trimmed}"));
 1015                     if source == SlashCommandDispatchSource::Live {
 1016                         self.clear_live_goal_submission();
 1017                     }
 1018                     return;
 1019                 }
 1020                 let draft = GoalDraft {
 1021                     objective: args,
 1022                     text_elements,
 1023                     pending_pastes,
 1024                     local_images,
 1025                     remote_image_urls,
 1026                 };
 1027                 let Some(thread_id) = self.thread_id else {
 1028                     if source == SlashCommandDispatchSource::Live {
 1029                         const GOAL_PREFIX: &str = "/goal ";
 1030                         let text_elements = draft
 1031                             .text_elements
 1032                             .into_iter()
 1033                             .map(|element| {
 1034                                 element.map_range(|range| ByteRange {
 1035                                     start: range.start + GOAL_PREFIX.len(),
 1036                                     end: range.end + GOAL_PREFIX.len(),
 1037                                 })
 1038                             })
 1039                             .collect();
 1040                         self.queue_user_message_with_options(
 1041                             UserMessage {
 1042                                 text: format!("{GOAL_PREFIX}{}", draft.objective),
 1043                                 local_images: draft.local_images,
 1044                                 remote_image_urls: draft.remote_image_urls,
 1045                                 text_elements,
 1046                                 mention_bindings: Vec::new(),
 1047                             },
 1048                             QueuedInputAction::ParseSlash,
 1049                             draft.pending_pastes,
 1050                         );
 1051                         self.clear_live_goal_submission();
 1052                     } else {
 1053                         self.add_info_message(
 1054                             GOAL_USAGE.to_string(),
 1055                             Some("The session must start before you can set a goal.".to_string()),
 1056                         );
 1057                     }
 1058                     return;
 1059                 };
 1060                 let history_objective = draft.objective.clone();
 1061                 self.app_event_tx.send(AppEvent::SetThreadGoalDraft {
 1062                     thread_id,
 1063                     draft,
 1064                     mode: ThreadGoalSetMode::ConfirmIfExists,
 1065                 });
 1066                 self.append_message_history_entry(format!("/goal {history_objective}"));
 1067                 if source == SlashCommandDispatchSource::Live {
 1068                     self.clear_live_goal_submission();
 1069                 }
 1070             }
