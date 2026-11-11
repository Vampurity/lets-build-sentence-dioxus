const fn is_allowed_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(
            c,
            ' ' | '\t' | '.' | ',' | '!' | '?' | '\'' | '"' | ':' | ';' | '(' | ')' | '-' | '/' | '&'
        )
}

fn with_spaces(value: &str) -> String {
    value.replace("\r\n", " ").replace(['\n', '\r'], " ")
}

pub fn is_allowed_english_insert(data: &str) -> bool {
    with_spaces(data).chars().all(is_allowed_char)
}

pub fn normalize_english_draft(value: &str) -> String {
    let filtered: String = with_spaces(value).chars().filter(|c| is_allowed_char(*c)).collect();
    let mut chars: Vec<char> = filtered.chars().collect();
    if let Some(index) = chars.iter().position(|c| c.is_ascii_alphabetic()) {
        chars[index] = chars[index].to_ascii_uppercase();
    }
    chars.into_iter().collect()
}

fn utf16_units(value: &str) -> Vec<u16> {
    value.encode_utf16().collect()
}

pub fn utf16_len(value: &str) -> u32 {
    value.encode_utf16().count() as u32
}

pub fn utf16_slice(value: &str, start: usize, end: usize) -> String {
    let units = utf16_units(value);
    let end = end.min(units.len());
    let start = start.min(end);
    String::from_utf16_lossy(&units[start..end])
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnglishEdit {
    pub value: String,
    pub caret_start: u32,
    pub caret_end: u32,
}

pub fn edit_from_raw(raw: &str, start: usize, end: usize) -> EnglishEdit {
    let next = normalize_english_draft(raw);
    if next == raw {
        return EnglishEdit {
            value: next,
            caret_start: start as u32,
            caret_end: end as u32,
        };
    }
    EnglishEdit {
        value: next,
        caret_start: utf16_len(&normalize_english_draft(&utf16_slice(raw, 0, start))),
        caret_end: utf16_len(&normalize_english_draft(&utf16_slice(raw, 0, end))),
    }
}

pub fn edit_from_insert(value: &str, start: usize, end: usize, data: &str) -> EnglishEdit {
    let prefix = utf16_slice(value, 0, start);
    let suffix = utf16_slice(value, end, usize::MAX);
    let caret = utf16_len(&normalize_english_draft(&format!("{prefix}{data}")));
    EnglishEdit {
        value: normalize_english_draft(&format!("{prefix}{data}{suffix}")),
        caret_start: caret,
        caret_end: caret,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalizes_only_the_first_english_letter() {
        assert_eq!(normalize_english_draft("i like apples."), "I like apples.");
        assert_eq!(normalize_english_draft("  hello"), "  Hello");
        assert_eq!(normalize_english_draft("Hello"), "Hello");
        assert_eq!(normalize_english_draft("I like apples."), "I like apples.");
        assert_eq!(normalize_english_draft("don't stop."), "Don't stop.");
        assert_eq!(normalize_english_draft("123 abc"), "123 Abc");
        assert_eq!(normalize_english_draft("rock & roll"), "Rock & roll");
    }

    #[test]
    fn drops_non_english_characters_and_turns_newlines_into_spaces() {
        assert_eq!(normalize_english_draft("你好hello"), "Hello");
        assert_eq!(normalize_english_draft("i like 苹果."), "I like .");
        assert_eq!(normalize_english_draft("hello\nworld"), "Hello world");
        assert_eq!(normalize_english_draft("hello\r\nworld"), "Hello world");
        assert_eq!(normalize_english_draft("ｈello"), "Ello");
        assert_eq!(normalize_english_draft("hi 😀"), "Hi ");
        assert_eq!(normalize_english_draft("..."), "...");
        assert_eq!(normalize_english_draft(""), "");
    }

    #[test]
    fn allows_english_letters_digits_spaces_and_common_punctuation() {
        assert!(is_allowed_english_insert("hello"));
        assert!(is_allowed_english_insert("Don't!"));
        assert!(is_allowed_english_insert("a b, c?"));
        assert!(!is_allowed_english_insert("你"));
        assert!(!is_allowed_english_insert("hello你好"));
        assert!(!is_allowed_english_insert("😀"));
    }

    #[test]
    fn maps_the_caret_after_filtering() {
        let edit = edit_from_raw("i like", 6, 6);
        assert_eq!(edit.value, "I like");
        assert_eq!(edit.caret_start, 6);
        let inserted = edit_from_insert("I ", 2, 2, "苹果like");
        assert_eq!(inserted.value, "I like");
        assert_eq!(inserted.caret_start, 6);
    }
}
