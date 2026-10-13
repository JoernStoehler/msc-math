    1 /// Parse a first-line slash command of the form `/name <rest>`.
    2 /// Returns `(name, rest_after_name, rest_offset)` if the line begins with `/`
    3 /// and contains a non-empty name; otherwise returns `None`.
    4 ///
    5 /// `rest_offset` is the byte index into the original line where `rest_after_name`
    6 /// starts after trimming leading whitespace (so `line[rest_offset..] == rest_after_name`).
    7 pub fn parse_slash_name(line: &str) -> Option<(&str, &str, usize)> {
    8     let stripped = line.strip_prefix('/')?;
    9     let mut name_end_in_stripped = stripped.len();
   10     for (idx, ch) in stripped.char_indices() {
   11         if ch.is_whitespace() {
   12             name_end_in_stripped = idx;
   13             break;
   14         }
   15     }
   16     let name = &stripped[..name_end_in_stripped];
   17     if name.is_empty() {
   18         return None;
   19     }
   20     let rest_untrimmed = &stripped[name_end_in_stripped..];
   21     let rest = rest_untrimmed.trim_start();
   22     let rest_start_in_stripped = name_end_in_stripped + (rest_untrimmed.len() - rest.len());
   23     let rest_offset = rest_start_in_stripped + 1;
   24     Some((name, rest, rest_offset))
