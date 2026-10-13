  263     fn snooze_auto_resolution(&mut self) {
  264         if !self.request.is_blocking {
  265             self.auto_resolution_snoozed = true;
  266         }
  267     }
  268 
  269     fn auto_resolution_timing_at(&self, now: Instant) -> AutoResolutionTiming {
  270         // autoResolutionMs is deprecated; isBlocking now controls whether the
  271         // request can auto-resolve using the TUI's fixed grace/countdown policy.
  272         if self.request.is_blocking || self.auto_resolution_snoozed {
  273             return AutoResolutionTiming::Disabled;
  274         }
  275 
  276         let elapsed = now.saturating_duration_since(self.request_started_at);
  277         if elapsed < AUTO_RESOLUTION_HIDDEN_GRACE {
  278             return AutoResolutionTiming::HiddenGrace {
  279                 remaining: AUTO_RESOLUTION_HIDDEN_GRACE.saturating_sub(elapsed),
  280             };
  281         }
  282         let visible_elapsed = elapsed.saturating_sub(AUTO_RESOLUTION_HIDDEN_GRACE);
  283         if visible_elapsed < AUTO_RESOLUTION_VISIBLE_COUNTDOWN {
  284             return AutoResolutionTiming::VisibleCountdown {
  285                 remaining: AUTO_RESOLUTION_VISIBLE_COUNTDOWN.saturating_sub(visible_elapsed),
  286             };
  287         }
  288         AutoResolutionTiming::Due
  289     }
