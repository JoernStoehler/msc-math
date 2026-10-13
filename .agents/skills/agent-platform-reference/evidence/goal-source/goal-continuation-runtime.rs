  425     pub(crate) async fn continue_if_idle(&self) -> Result<(), String> {
  426         if !self.tools_available() {
  427             self.inner.accounting_state.clear_active_goal();
  428             return Ok(());
  429         }
  430         // Hold this through the read/start window so external set/clear cannot
  431         // change the goal after we read it but before the continuation launches.
  432         let _goal_state_permit = self.goal_state_permit().await?;
  433 
  434         if self
  435             .inner
  436             .state_dbs
  437             .thread_goals()
  438             .has_thread_goal_continuation_deferral(self.thread_id())
  439             .await
  440             .map_err(|err| err.to_string())?
  441         {
  442             return Ok(());
  443         }
  444 
  445         let Some(thread_manager) = self.inner.thread_manager.upgrade() else {
  446             tracing::debug!("skipping goal continuation because thread manager is unavailable");
  447             return Ok(());
  448         };
  449         let Ok(thread) = thread_manager.get_thread(self.inner.thread_id).await else {
  450             tracing::debug!("skipping goal continuation because live thread is unavailable");
  451             return Ok(());
  452         };
  453 
  454         let Some(goal) = self
  455             .inner
  456             .state_dbs
  457             .thread_goals()
  458             .get_thread_goal(self.thread_id())
  459             .await
  460             .map_err(|err| err.to_string())?
  461         else {
  462             self.inner.accounting_state.clear_active_goal();
  463             return Ok(());
  464         };
  465         if goal.status != codex_state::ThreadGoalStatus::Active {
  466             self.inner.accounting_state.clear_active_goal();
  467             return Ok(());
  468         }
  469         let start_options = thread
  470             .thread_extension_data()
  471             .get::<TurnStartOptions>()
  472             .map(|options| options.as_ref().clone())
  473             .unwrap_or_default();
  474         let item = continuation_steering_item(
  475             &protocol_goal_from_state(goal),
  476             thread.config().await.update_plan_enabled,
  477         );
  478 
  479         match thread
  480             .start_turn_if_idle(
  481                 TurnInputRequest::new(TurnInput::ResponseItem(item)).on_start(TurnStartOptions {
  482                     turn_trigger: Some("goal".to_string()),
  483                     ..start_options
  484                 }),
  485             )
  486             .await
