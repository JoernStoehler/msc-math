  747         // Resolve base instructions for the session. Priority order:
  748         // 1. config.base_instructions override
  749         // 2. conversation history => session_meta.base_instructions
  750         // 3. rendered instructions_template for current model
  751         let model_info = models_manager
  752             .get_model_info(model.as_str(), &config.to_models_manager_config())
  753             .await;
  754         let auth = auth_manager.auth_cached();
  755         // Forked subagents keep their parent's activation with the copied history.
