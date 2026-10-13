  180     fn on_thread_idle<'a>(&'a self, input: ThreadIdleInput<'a>) -> ExtensionFuture<'a, ()> {
  181         Box::pin(async move {
  182             let Some(runtime) = goal_runtime_handle(input.thread_store) else {
  183                 return;
  184             };
  185 
  186             if let Err(err) = runtime.continue_if_idle().await {
  187                 tracing::warn!(
  188                     "failed to continue active goal for idle thread {}: {err}",
  189                     runtime.thread_id()
  190                 );
  191             }
  192         })
  193     }
