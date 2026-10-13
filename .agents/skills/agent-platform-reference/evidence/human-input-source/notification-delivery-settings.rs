    5 impl ChatWidget {
    6     pub(super) fn notify(&mut self, notification: Notification) {
    7         if !notification.allowed_for(&self.local_settings.tui.notification_settings.notifications) {
    8             return;
    9         }
   10         if let Some(existing) = self.pending_notification.as_ref()
   11             && existing.priority() > notification.priority()
   12         {
   13             return;
   14         }
   15         self.pending_notification = Some(notification);
   16         self.request_redraw();
   17     }
   18 
   19     pub(crate) fn maybe_post_pending_notification(&mut self, tui: &mut crate::tui::Tui) {
   20         if let Some(notif) = self.pending_notification.take() {
   21             tui.notify(notif.display());
   22         }
   23     }
