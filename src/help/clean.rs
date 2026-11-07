//! An error text as Help is handed it: Help only ever draws it as words, so it is cleaned and cut again here, whatever sent it.

/// The most of an error's real words Help keeps.
pub const ERROR_LIMIT: usize = 4000;

/// Characters that show nothing, change the direction of text, or act on a terminal, less the joiners and variation selectors that emoji and some scripts need.
fn hides(c: char) -> bool {
    let n = c as u32;
    let hidden = c.is_control()
        || matches!(n, 0xad | 0x34f | 0x61c | 0x115f | 0x1160 | 0x17b4 | 0x17b5 | 0x180b..=0x180f | 0x200b..=0x200f | 0x202a..=0x202e
            | 0x2028 | 0x2029 | 0x2060..=0x206f | 0x2800 | 0x3164 | 0xfe00..=0xfe0f | 0xfeff | 0xffa0 | 0xfff9..=0xfffb | 0xe000..=0xf8ff
            | 0xe0000..=0xe007f | 0xf0000..=0x10ffff | 0x1d173..=0x1d17a)
        || (n & 0xfffe) == 0xfffe
        || (0xfdd0..=0xfdef).contains(&n);
    c.is_control() || (hidden && !matches!(c, '\u{200C}' | '\u{200D}' | '\u{FE00}'..='\u{FE0F}'))
}

/// Control characters other than line ends, and the direction marks and invisible characters that could disguise what is copied, become spaces, and it is cut at `ERROR_LIMIT` bytes on a character.
pub fn error(text: &str) -> String {
    let mut out: String = text.chars().map(|c| if c == '\n' || !hides(c) { c } else { ' ' }).collect();
    if out.len() > ERROR_LIMIT {
        let mut at = ERROR_LIMIT;
        while !out.is_char_boundary(at) {
            at -= 1;
        }
        out.truncate(at);
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_text_is_cleaned_and_cut() {
        assert_eq!(error("a\u{7}b\nc\u{1b}[0m "), "a b\nc [0m");
        assert_eq!(error("  \n "), "");
        assert_eq!(error("rm\u{202e}fi\u{200b}le"), "rm fi le");
        let cut = error(&"\u{e9}".repeat(ERROR_LIMIT));
        assert!(cut.len() <= ERROR_LIMIT && cut.chars().all(|c| c == '\u{e9}'));
    }
}
