//! Самая длинная подстрока без повторяющихся символов

#![cfg(test)]

use std::collections::HashSet;

// Тут учитывается только ascii символы.
// Если использовать русские буквы, то нужно использовать библиотеку
fn length_of_longest_substring(s: String) -> i32 {
    let count_chars = s.chars().count();
    let mut max = 0;
    for start_pos in 0..count_chars {
        if max > count_chars - start_pos {
            break;
        }

        let mut unc = HashSet::new();
        for ch in s.chars().skip(start_pos) {
            if !unc.insert(ch) {
                break;
            }
        }
        max = unc.len().max(max);
    }

    max as i32
}

fn length_of_longest_substring_2(s: String) -> i32 {
    let count_chars = s.chars().count();
    let mut max = 0;

    let mut start_pos = 0;
    loop {
        if max > count_chars - start_pos {
            break;
        }

        let mut unc = Vec::new();
        for (pos, ch) in s.chars().enumerate().skip(start_pos) {
            if let Some((p, _)) = unc.iter().find(|(_, v)| v == &ch) {
                start_pos = *p;
                break;
            };
            unc.push((pos, ch));
        }
        max = unc.len().max(max);

        start_pos += 1;
        if count_chars == start_pos {
            break;
        }
    }

    max as i32
}

#[test]
fn test_length_of_longest_substring() {
    for (input, output) in [
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3),
        ("au", 2),
        ("dvdf", 3),
        (" ", 1),
        ("aab", 2),
    ] {
        assert_eq!(length_of_longest_substring(input.to_string()), output);
        assert_eq!(length_of_longest_substring_2(input.to_string()), output);
    }
}
