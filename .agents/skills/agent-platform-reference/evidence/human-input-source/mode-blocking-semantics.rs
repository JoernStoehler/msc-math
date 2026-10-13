  674 pub enum ModeKind {
  675     Plan,
  676     #[default]
  677     #[serde(
  678         alias = "code",
  679         alias = "pair_programming",
  680         alias = "execute",
  681         alias = "custom"
  682     )]
  683     Default,
  684 }
  685 
  686 pub const TUI_VISIBLE_COLLABORATION_MODES: [ModeKind; 2] = [ModeKind::Default, ModeKind::Plan];
  687 
  688 impl ModeKind {
  689     pub const fn display_name(self) -> &'static str {
  690         match self {
  691             Self::Plan => "Plan",
  692             Self::Default => "Default",
  693         }
  694     }
  695 
  696     pub const fn is_tui_visible(self) -> bool {
  697         matches!(self, Self::Plan | Self::Default)
  698     }
  699 
  700     pub const fn allows_request_user_input(self) -> bool {
  701         matches!(self, Self::Plan)
  702     }
  703 }
