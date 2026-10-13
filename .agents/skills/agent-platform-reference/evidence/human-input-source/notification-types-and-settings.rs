   60             Notification::ElicitationRequested { server_name } => {
   61                 format!("Approval requested by {server_name}")
   62             }
   63             Notification::PlanModePrompt { title } => {
   64                 format!("Plan mode prompt: {title}")
   65             }
   66             Notification::AsyncQuestion { title } => {
   67                 format!("Question: {title}")
   68             }
   69         }
   70     }
   71 
   72     fn type_name(&self) -> &str {
   73         match self {
   74             Notification::AgentTurnComplete { .. } => "agent-turn-complete",
   75             Notification::ExecApprovalRequested { .. }
   76             | Notification::EditApprovalRequested { .. }
   77             | Notification::ElicitationRequested { .. } => "approval-requested",
   78             Notification::PlanModePrompt { .. } => "plan-mode-prompt",
   79             Notification::AsyncQuestion { .. } => "async-question",
   80         }
   81     }
   82 
   83     fn priority(&self) -> u8 {
   84         match self {
   85             Notification::AgentTurnComplete { .. } => 0,
   86             Notification::ExecApprovalRequested { .. }
   87             | Notification::EditApprovalRequested { .. }
   88             | Notification::ElicitationRequested { .. }
   89             | Notification::PlanModePrompt { .. }
   90             | Notification::AsyncQuestion { .. } => 1,
   91         }
   92     }
   93 
   94     pub(super) fn allowed_for(&self, settings: &Notifications) -> bool {
   95         match settings {
   96             Notifications::Enabled(enabled) => *enabled,
   97             Notifications::Custom(allowed) => allowed.iter().any(|a| a == self.type_name()),
   98         }
   99     }
