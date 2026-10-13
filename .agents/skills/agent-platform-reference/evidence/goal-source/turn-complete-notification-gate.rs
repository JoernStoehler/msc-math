  203         if !from_replay {
  204             // Emit a notification only when the live agent is waiting for the user.
  205             let follow_up_started = self.maybe_send_next_queued_input();
  206             let active_goal_continuing = self
  207                 .current_goal_status
  208                 .as_ref()
  209                 .is_some_and(GoalStatusState::is_active);
  210             if !follow_up_started
  211                 && !active_goal_continuing
  212                 && !self
  213                     .turn_lifecycle
  214                     .last_turn_id
  215                     .as_deref()
  216                     .is_some_and(|turn_id| self.should_hide_realtime_delegation(turn_id))
  217             {
  218                 self.notify(Notification::AgentTurnComplete {
  219                     response: notification_response,
  220                 });
  221             }
  222             self.maybe_show_pending_rate_limit_prompt();
  223         }
