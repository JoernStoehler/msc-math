 1258             // Existing model catalogs still advertise the previous name.
 1259             .any(|tool| {
 1260                 matches!(
 1261                     tool.as_str(),
 1262                     "request_user_input_async" | "send_user_message_async"
 1263                 )
 1264             })
 1265     {
 1266         let model_messages = ResolvedModelMessages::from_model(context.model_info);
 1267         registry.add_with_exposure(
 1268             RequestUserInputAsyncHandler {
 1269                 description: model_messages
 1270                     .request_user_input_async_description()
 1271                     .to_string(),
 1272                 parameters: model_messages
 1273                     .request_user_input_async_parameters_override()
 1274                     .map(str::to_owned),
 1275             },
 1276             ToolExposure::DirectModelOnly,
 1277         );
 1278     }
 1279 
