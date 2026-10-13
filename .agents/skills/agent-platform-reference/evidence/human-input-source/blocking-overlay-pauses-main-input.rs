 1885     /// Called when the agent requests user input.
 1886     pub fn push_user_input_request(&mut self, request: ToolRequestUserInputParams) {
 1887         let request = if let Some(view) = self.view_stack.last_mut() {
 1888             match view.try_consume_user_input_request(request) {
 1889                 Some(request) => request,
 1890                 None => {
 1891                     self.request_redraw();
 1892                     return;
 1893                 }
 1894             }
 1895         } else {
 1896             request
 1897         };
 1898 
 1899         let modal = RequestUserInputOverlay::new_with_keymap(
 1900             request,
 1901             self.app_event_tx.clone(),
 1902             self.has_input_focus,
 1903             self.enhanced_keys_supported,
 1904             self.disable_paste_burst,
 1905             self.keymap.clone(),
 1906         );
 1907         self.pause_status_timer_for_modal();
 1908         self.set_composer_input_enabled(
 1909             /*enabled*/ false,
 1910             Some("Answer the questions to continue.".to_string()),
 1911         );
