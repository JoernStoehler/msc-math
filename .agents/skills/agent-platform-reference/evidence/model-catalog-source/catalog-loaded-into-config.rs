 1025     /// using the Responses API. When unset, the model catalog default is used.
 1026     pub model_reasoning_summary: Option<ReasoningSummary>,
 1027 
 1028     /// Optional full model catalog loaded from `model_catalog_json`.
 1029     /// When set, this replaces the bundled catalog for the current process.
 1030     pub model_catalog: Option<ModelsResponse>,
 1031 
