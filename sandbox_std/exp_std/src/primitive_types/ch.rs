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

    assert_eq!("Hello\n\rWorld\n\n".trim_ascii(), "Hello\n\rWorld");

    let s = b"hello world";
    let mut buf: &[u8] = s;
    loop {
        let Some((first, other)) = buf.split_first() else {
            break;
        };
        let ch = *first as char;
        if ch.is_alphabetic() {
            print!(" {ch} ");
        }
        buf = other;
    }
    println!();

    let mut buf: &[u8] = s;
    while let Some((last, other)) = buf.split_last() {
        let ch = *last as char;
        if ch.is_alphabetic() {
            print!(" {ch} ");
        }
        buf = other;
    }
    println!();

    println!("First: {:?}", s.first().cloned().map(char::from));
    println!("Last: {:?}", s.last().cloned().map(char::from));

    let (sl1, sl2) = s.split_first_chunk::<5>().unwrap();
    assert_eq!(sl1, b"hello");
    assert_eq!(sl2, b" world");
    println!("Split: {:?} {:?}", sl1, sl2);

    let (sl1, sl2) = s.split_last_chunk::<5>().unwrap();
    assert_eq!(sl1, b"hello ");
    assert_eq!(sl2, b"world");

    assert_eq!(s.last_chunk::<5>(), Some(b"world"));
    assert_eq!(s.first_chunk::<5>(), Some(b"hello"));

    let mut m = *s;
    let t = m.first_chunk_mut::<2>().unwrap();
    t[0] = b'H';
    assert_eq!(&m, b"Hello world");

    // Поменять местами буквы в строке
    m.swap(0, 1);
    assert_eq!(&m, b"eHllo world");

    m.reverse();
    assert_eq!(&m, b"dlrow ollHe");

    for ele in s.windows(3) {
        println!("{}", ele.iter().map(|ch| *ch as char).collect::<String>());
    }
    println!();

    for ele in s.chunks(3) {
        println!("{}", ele.iter().map(|ch| *ch as char).collect::<String>());
    }
    println!();

    for ele in s.rchunks(4) {
        println!("{}", ele.iter().map(|ch| *ch as char).collect::<String>());
    }
    println!();

    let mut k = s.chunks_exact(3);
    // [hel][lo ][wor]ld
    for ele in &mut k {
        println!("{}", ele.iter().map(|ch| *ch as char).collect::<String>());
    }
    // hello wor[ld]
    assert_eq!(k.remainder(), b"ld");

    let t = text
        .split_inclusive(|ch: char| ch.is_numeric() || ch.is_ascii_punctuation())
        .next()
        .unwrap();
    println!("{t}"); // Демо текст.
    println!();

    // 3 максимальное число дробления
    for ele in text.splitn(3, |ch: char| ch.is_ascii_punctuation()) {
        println!("{ele}");
    }

    // (split) countains
    for c in ['h', 'l', 'z'] {
        if s.contains(&(c as u8)) {
            println!("'{c}' найден!");
        } else {
            println!("'{c}' не найден!");
        }
    }

    assert!(s.starts_with(b"hello")); // [hello] world
    assert!(s.ends_with(b"ld")); // hello wor[ld]

    let k = [1, 2, 3, 4];
    assert_eq!(k.binary_search(&2), Ok(1));
    assert_eq!(k.binary_search_by(|v| v.cmp(&3)), Ok(2));

    assert!(k.is_sorted());

    let mut s = *b"hello word";
    s.rotate_left(5);
    assert_eq!(&s, b" wordhello");

    s.rotate_right(4);
    assert_eq!(&s, b"ello wordh");

    let mut d = [0u8, 0, 0];
    d.clone_from_slice(&s[5..8]);
    assert_eq!(&d, b"wor");

    s.fill(b'a');
    assert_eq!(&s, b"aaaaaaaaaa");

    if let Ok([a, b]) = s.get_disjoint_mut([0, 2]) {
        *a = b'h';
        *b = b'l';
    }
    assert_eq!(&s, b"halaaaaaaa");
    let b = [s, *b" world    "];
    assert_eq!(b.as_flattened(), b"halaaaaaaa world    ");
}
