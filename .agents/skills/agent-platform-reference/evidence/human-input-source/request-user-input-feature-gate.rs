 1244     if turn_context.config.experimental_request_user_input_enabled {
 1245         registry.add_with_exposure(
 1246             RequestUserInputHandler {
 1247                 available_modes: request_user_input_available_modes(features),
 1248             },
 1249             ToolExposure::DirectModelOnly,
 1250         );
 1251     }
