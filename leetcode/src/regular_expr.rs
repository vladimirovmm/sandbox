//! Написать функцию, которая проверяет, соответствует ли строка шаблону.
//! В шаблоне могут использоваться символы:
//! `*` - любое количество указанного символа (0 и больше)
//! `.` - любой один символ
//! `.*` - любое количество любого символа (0 и больше)

#![cfg(test)]

fn normalize(p: &str) -> Option<Vec<(&str, bool)>> {
    let mut tmp_pat: Vec<(&str, bool)> = Default::default();

    for part in p.split_inclusive('*') {
        if !part.ends_with('*') {
            tmp_pat.push((part, false));
            continue;
        }

        let bk = part.char_indices().rev().nth(1)?;

        let mult = &part[bk.0..];
        if mult != part {
            tmp_pat.push((&part[..bk.0], false));
        }
        tmp_pat.push((mult.trim_end_matches('*'), true));
    }

    let mut result = Vec::new();
    let mut bk = None;

    for p in tmp_pat.into_iter() {
        if bk == Some(p) {
            continue;
        }
        bk = Some(p);
        result.push(p);
    }

    Some(result)
}

fn match_pattern(s: &str, pattern: &[(&str, bool)]) -> bool {
    if s.is_empty() && pattern.is_empty() {
        return true;
    }

    // Строгий слева
    let Some(l_pattern) = pattern.first() else {
        return s.is_empty();
    };

    if !l_pattern.1 {
        if let Some(s) = s.strip_prefix(l_pattern.0) {
            return match_pattern(s, &pattern[1..]);
        }
        if l_pattern.0.contains('.') {
            let Some(part) = s.get(..l_pattern.0.len()) else {
                return false;
            };
            if l_pattern
                .0
                .chars()
                .zip(part.chars())
                .all(|(a, b)| a == '.' || a == b)
            {
                return match_pattern(&s[l_pattern.0.len()..], &pattern[1..]);
            } else {
                return false;
            }
        }
    }

    // Строгий справа
    let Some(r_pattern) = pattern.last() else {
        return s.is_empty();
    };
    if !r_pattern.1 {
        if let Some(s) = s.strip_suffix(r_pattern.0) {
            return match_pattern(s, &pattern[..pattern.len() - 1]);
        }
        if r_pattern.0.contains('.') {
            let part: String = {
                let s_len = s.chars().count();
                let p_len = r_pattern.0.chars().count();
                let start_pos = s_len - p_len;
                let part: String = s.chars().skip(start_pos).collect();
                if part.chars().count() != p_len {
                    return false;
                }

                part
            };

            if r_pattern
                .0
                .chars()
                .zip(part.chars())
                .all(|(a, b)| a == '.' || a == b)
            {
                return match_pattern(&s[..s.len() - part.len()], &pattern[..pattern.len() - 1]);
            } else {
                return false;
            }
        }
    }

    // Множественный слева
    if l_pattern.1 {
        let Some(need_char) = l_pattern.0.chars().next() else {
            return false;
        };
        if let Some(end) = s
            .char_indices()
            .take_while(|(_, ch)| need_char == '.' || ch == &need_char)
            .last()
        {
            let size_char = need_char.len_utf8();
            for pos in (0..=end.0).rev().step_by(size_char) {
                if match_pattern(&s[pos + size_char..], &pattern[1..]) {
                    return true;
                }
            }
        };

        return match_pattern(s, &pattern[1..]);
    }
    false
}

fn is_match(s: String, p: String) -> bool {
    let Some(pattern) = normalize(&p) else {
        return false;
    };

    match_pattern(&s, &pattern)
}

#[test]
fn test_is_match() {
    for (input_str, input_pat, exp) in [
        ("aa", "aa", true),
        ("aa", "a", false),
        ("aa", "a.", true),
        ("aa", "..", true),
        ("aa", ".", false),
        ("abc", "a.c", true),
        ("abc", "ab.", true),
        ("abc", "a..", true),
        ("aa", "a*", true),
        ("aa", "*a", false),
        ("aa", "*a*", false),
        ("abc", "a*bc", true),
        ("abc", "a*b*c*", true),
        ("aaaabc", "a*bc", true),
        ("abc", ".*", true),
        ("aaaabc", "a*..", true),
        ("aaa", "ab*a", false),
        ("aaa", "ab*a*c*a", true),
        ("a", "ab*a", false),
        ("a", ".*..a*", false),
        ("aa", ".*.*", true),
        ("ab", ".*..", true),
        ("abc", ".*..", true),
        ("abc", ".*..*", true),
        ("abc", "...*", true),
        ("abc", "..*.", true),
        ("abc", ".*..", true),
        ("abc", "...", true),
        ("abc", ".*.*.", true),
        ("abc", ".*.*.*", true),
        ("aaaaaaaaaaaaaaaaaaab", "a*a*a*a*a*a*a*a*a*a*", false),
        ("mississippi", "mis*is*ip*.", true),
        ("bbbb", ".c*.", false),
        ("bccbbabcaccacbcacaa", ".*b.*c*.*.*.c*a*.c", false),
    ] {
        assert_eq!(
            is_match(input_str.to_string(), input_pat.to_string()),
            exp,
            "{input_str}, {input_pat}"
        );
    }
}
