  648     pub(crate) fn is_active_goal_turn_running(&self) -> bool {
  649         self.turn_lifecycle.agent_turn_running
  650             && self
  651                 .current_goal_status
  652                 .as_ref()
  653                 .is_some_and(GoalStatusState::is_active)
  654     }
  655 
  656     pub(crate) fn pause_active_goal_for_interrupt(&self) {
  657         if !self.is_active_goal_turn_running() {
  658             return;
  659         }
  660         let Some(thread_id) = self.thread_id else {
  661             return;
  662         };
  663         self.app_event_tx.send(AppEvent::SetThreadGoalStatus {
  664             thread_id,
  665             status: AppThreadGoalStatus::Paused,
  666         });
  667     }
