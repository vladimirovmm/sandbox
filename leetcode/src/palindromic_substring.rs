//! Поиск самого длинного палиндрома в строке

#![cfg(test)]

pub fn longest_palindrome(s: String) -> String {
    fn fpalindrome(s: &[char], center: usize) -> Option<String> {
        let result_1: Option<String> = (0..=center)
            .take_while(|v| s.get(center - v) == s.get(center + v))
            .last()
            .map(|p| s[center - p..=center + p].iter().collect());

        let result_2: Option<String> = (0..=center)
            .take_while(|v| s.get(center - v) == s.get(center + v + 1))
            .last()
            .map(|p| s[center - p..=center + p + 1].iter().collect());

        match (result_1, result_2) {
            (Some(result_1), Some(result_2)) => {
                if result_1.len() > result_2.len() {
                    Some(result_1)
                } else {
                    Some(result_2)
                }
            }
            (Some(result), None) => Some(result),
            (None, Some(result)) => Some(result),
            (None, None) => None,
        }
    }

    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    let mut result = "".to_string();
    for cn in 0..len {
        if let Some(r) = fpalindrome(&s, cn) {
            if r.len() > result.len() {
                result = r;
            }
        }
        if len - cn < result.len() / 2 {
            break;
        }
    }
    result
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(
        longest_palindrome("kaabbaa".to_string()),
        "aabbaa".to_string()
    );
    assert_eq!(
        longest_palindrome("kaabbaa".to_string()),
        "aabbaa".to_string()
    );
    assert_eq!(
        longest_palindrome("ababababa".to_string()),
        "ababababa".to_string()
    );
}
