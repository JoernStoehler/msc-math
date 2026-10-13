   17 pub fn request_user_input_available_modes(features: &Features) -> Vec<ModeKind> {
   18     TUI_VISIBLE_COLLABORATION_MODES
   19         .into_iter()
   20         .filter(|mode| {
   21             mode.allows_request_user_input()
   22                 || (features.enabled(Feature::DefaultModeRequestUserInput)
   23                     && *mode == ModeKind::Default)
   24         })
   25         .collect()
   26 }
