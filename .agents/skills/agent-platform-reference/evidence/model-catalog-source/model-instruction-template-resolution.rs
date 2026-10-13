    1 //! Returns literal model instructions from the captured model metadata.
    2 //! Missing templates produce a warning; supplied empty templates remain valid input.
    3 
    4 use crate::ResolvedModelMessages;
    5 use codex_protocol::openai_models::ModelInfo;
    6 
    7 /// Renders base instructions from the caller's captured model, retaining missing-template warnings.
    8 pub fn render_model_instructions(model_info: &ModelInfo) -> String {
    9     let template = ResolvedModelMessages::from_model(model_info).instructions_template();
   10     if template.is_none() {
   11         tracing::warn!(
   12             model = %model_info.slug,
   13             "Model has no instruction template; returning empty instructions."
   14         );
   15     }
   16     template.unwrap_or_default().to_owned()
   17 }
   18 
