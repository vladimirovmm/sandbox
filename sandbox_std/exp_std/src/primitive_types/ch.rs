#![cfg(test)]

use rand::seq::IteratorRandom;

//
#[test]
fn test_match() {
    let c = 'a';

    match c {
        'a'..='z' => println!("Это буква"),
        _ => println!("Не в деапозоне"),
    }

    let c = 'б';
    if matches!(c, 'а'..='я') {
        println!("Русская буква");
    }

    let c = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .choose(&mut rand::rng())
        .unwrap();

    println!("Рандомная буква {c}");

    //
    let dist = u32::from(char::MAX) - u32::from(char::MIN);
    let size = (char::MIN..=char::MAX).count() as u32;
    assert!(size < dist);
    println!("Количество букв: {size} {dist}");

    for c in 'a'..='z' {
        print!("{c}={:?}, ", c as u32);
    }
    println!();
    println!();

    for cn in 97..122 {
        print!("{}={cn}, ", char::from_u32(cn).unwrap());
    }
    println!();
    println!();

    assert_eq!('1'.to_digit(10), Some(1));
    assert_eq!('f'.to_digit(16), Some(15));
    assert_eq!('z'.to_digit(16), None);

    assert_eq!('A'.len_utf8(), 1);
    assert_eq!('ß'.len_utf8(), 2);
    assert_eq!('ß'.len_utf16(), 1);
    assert_eq!('ℝ'.len_utf8(), 3);
    assert_eq!('💣'.len_utf8(), 4);
    assert_eq!('💣'.len_utf16(), 2);

    let text = "Демо текст. Из которого нужно вырезать цифры 2025.03.21";

    // Оставить только буквы
    let s: String = text.chars().filter(|c| c.is_alphabetic()).collect();
    println!("{s}"); // ДемотекстИзкоторогонужновырезатьцифры

    // Оставить только буквы и пробелы
    let s: String = text
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_ascii_whitespace())
        .collect(); // Демо текст Из которого нужно вырезать цифры
    println!("{s}");

    // Только заглавные буквы
    let s: String = text.chars().filter(|c| c.is_uppercase()).collect(); // ДИ
    println!("{s}");

    // Только прописные буквы
    let s: String = text.chars().filter(|c| c.is_lowercase()).collect(); // емотекстзкоторогонужновырезатьцифры
    println!("{s}");

    // Только цифры
    let s: String = text.chars().filter(|c| c.is_numeric()).collect(); // 20250321
    println!("{s}");

    // Цифры, буквы и пробелы
    let s: String = text
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect(); // Демо текст Из которого нужно вырезать цифры 20250321
    println!("{s}");

    // Пунктуация
    let s: String = text.chars().filter(|c| c.is_ascii_punctuation()).collect(); // ...
    println!("{s}");
}
