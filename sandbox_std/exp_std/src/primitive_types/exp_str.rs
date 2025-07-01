#![cfg(test)]

//
#[test]
fn test_ptr() {
    use std::slice;
    use std::str;

    let story = "Once upon a time...";
    assert_eq!(19, story.len());

    let ptr = story.as_ptr();

    // Через указатель создаём строку
    let s = unsafe {
        // Считать первые 4 байта нашего указателя, и вернуть их как срез.
        let slice = slice::from_raw_parts(ptr, 4);

        // приобразовать в строку
        str::from_utf8(slice)
    };

    assert_eq!(s, Ok("Once"));
    assert_eq!(story.as_ptr(), s.unwrap().as_ptr());

    // Безопасный способ создания строки через указатель
    let k = &story[0..4];
    assert_eq!(k, "Once");
    assert_eq!(story.as_ptr(), k.as_ptr());
}

#[allow(clippy::const_is_empty)]
#[test]
fn test_len() {
    let text = "Hello, world!";
    assert_eq!(text.len(), 13);
    assert_eq!(text.chars().count(), 13);

    let text = "Здравствуйте!"; // 13 букв
    assert_eq!(text.len(), 25); // В строке 25 байт
    assert_eq!(text.chars().count(), 13); // Символов 13

    let text = "";
    assert!(text.is_empty()); // пустая строка
    assert_eq!(text.chars().count(), 0); // 0 символов
    assert_eq!(text.len(), 0); // 0 байт

    let text = "\u{0}"; // нулевой символ
    assert_eq!(text.chars().count(), 1); // 1 символ
    assert_eq!(text.len(), 1); // 1 байт
    assert!(!text.is_empty()); // не пустая

    assert!(!" ".is_empty()); // не пустая строка
}

/// Срез строки
#[test]
fn test_slice() {
    let text = "Hello, world!";
    let slice = &text[7..12]; // world
    assert_eq!(slice, "world");
    let slice = &text[7..250];
    assert_eq!(slice, "world");

    // Получение подстроки (срез строки)
    let slice = text.get(7..12).unwrap(); // world
    assert_eq!(slice, "world");

    // Выход за границы строки
    let slice = text.get(7..14);
    assert_eq!(slice, None);
}

#[test]
#[allow(clippy::manual_pattern_char_comparison)]
fn test_split() {
    let date = "2025-04-22 13:44:00";
    let text = "Hello, world.. Hi!";
    // Разделить строку по пробелам
    let words: Vec<_> = text.split(' ').collect();
    assert_eq!(words, vec!["Hello,", "world..", "Hi!"]);

    // Разделить пустую строку по букве 'd'
    let r: Vec<_> = "".split('d').collect();
    assert_eq!(r, vec![""]); // Пустая строка

    // Разделить по пробельным символам
    let r: Vec<_> = text.split(char::is_whitespace).collect();
    assert_eq!(r, vec!["Hello,", "world..", "Hi!"]);

    // Разделить по списку символов
    let r: Vec<_> = date.split(&['-', ' ', ':']).collect();
    assert_eq!(r, vec!["2025", "04", "22", "13", "44", "00"]);

    // Разделить по условию
    let r: Vec<_> = date
        .split(|c: char| c == '-' || c == ':' || c == ' ')
        .collect();
    assert_eq!(r, vec!["2025", "04", "22", "13", "44", "00"]);

    // Срез по индексу
    let t = text.split_at(6); // ("Hello,", " world.. Hi!")
    assert_eq!(t, ("Hello,", " world.. Hi!"));

    let t = "Здравствуйте!".split_at(6); // ("Здр", "авствуйте!")
    assert_eq!(t, ("Здр", "авствуйте!"));

    // Выходить за пределы нельзя
    // let t = text.split_at(2500); // Panic

    // Безопасный вариант получения среза
    let t = text.split_at_checked(6);
    assert_eq!(t, Some(("Hello,", " world.. Hi!")));
    // Безопасный вариант получения среза. Выход за границы
    let t = text.split_at_checked(2500);
    assert_eq!(t, None);

    let t = "Здравствуйте!".split_at_checked(6); // ("Здравс", "твуйте!")
    assert_eq!(t, Some(("Здр", "авствуйте!")));

    // Интератор по символам и индексам
    for (index, ch) in text.char_indices() {
        println!("{index}#: {ch}");
    }
    println!();

    for (index, ch) in "Здравствуйте!".char_indices() {
        // index позиция в байтов в строке
        println!("{index}#: {ch}");
    }
    println!();

    // Разбить строку по пробельным символам
    assert_eq!(text.split_whitespace().count(), 3); // ["Hello,", "world..", "Hi!"]
    assert_eq!("1\n2\t3 4\r5".split_whitespace().count(), 5);
    assert_eq!("hi\t\t\t5".split_whitespace().count(), 2);

    let r: Vec<_> = text.split_inclusive(char::is_whitespace).collect();
    assert_eq!(r, vec!["Hello, ", "world.. ", "Hi!"]);

    // Разбить строку по пробелу с конца.
    let r: Vec<_> = text.rsplit(' ').collect();
    assert_eq!(r, vec!["Hi!", "world..", "Hello,"]);

    // Разбить строку по пробельным символам с ограничением кол-ва разбиений
    let r: Vec<_> = text.splitn(2, char::is_whitespace).collect();
    assert_eq!(r, vec!["Hello,", "world.. Hi!"]);

    let r: Vec<_> = text.splitn(4, char::is_whitespace).collect();
    assert_eq!(r, vec!["Hello,", "world..", "Hi!"]);

    // Разбить строку c конца по пробельным символам с ограничением кол-ва разбиений.
    let r: Vec<_> = text.rsplitn(2, char::is_whitespace).collect();
    assert_eq!(r, vec!["Hi!", "Hello, world.."]);

    // Разбить строку по переданному условию один раз
    let r = text.split_once(' ');
    assert_eq!(r, Some(("Hello,", "world.. Hi!")));

    let r = text.split_once('z');
    assert_eq!(r, None);

    // Разбить строку с конца по переданному условию один раз
    let r = text.rsplit_once(" ");
    assert_eq!(r, Some(("Hello, world..", "Hi!")));

    // Получить символы только удовлетворяющие условию
    let r: Vec<_> = date.matches(char::is_numeric).collect();
    assert_eq!(
        r,
        vec![
            "2", "0", "2", "5", "0", "4", "2", "2", "1", "3", "4", "4", "0", "0"
        ]
    );

    // Получить символы только удовлетворяющие условию начиная с конца
    let r: Vec<_> = date.rmatches(char::is_numeric).collect();
    assert_eq!(
        r,
        vec![
            "0", "0", "4", "4", "3", "1", "2", "2", "4", "0", "5", "2", "0", "2"
        ]
    );

    // Получить символы только удовлетворяющие условию и позиции символов
    let r: Vec<_> = date.match_indices(char::is_numeric).collect();
    assert_eq!(
        r,
        vec![
            (0, "2"),
            (1, "0"),
            (2, "2"),
            (3, "5"),
            (5, "0"),
            (6, "4"),
            (8, "2"),
            (9, "2"),
            (11, "1"),
            (12, "3"),
            (14, "4"),
            (15, "4"),
            (17, "0"),
            (18, "0")
        ]
    );

    // Получить символы только удовлетворяющие условию начиная с конца и позиции символов
    let r: Vec<_> = date.rmatch_indices(char::is_numeric).collect();
    assert_eq!(
        r,
        vec![
            (18, "0"),
            (17, "0"),
            (15, "4"),
            (14, "4"),
            (12, "3"),
            (11, "1"),
            (9, "2"),
            (8, "2"),
            (6, "4"),
            (5, "0"),
            (3, "5"),
            (2, "2"),
            (1, "0"),
            (0, "2")
        ]
    );
}

/// Проверка содержит ли строка подстроку
#[test]
fn test_countains() {
    let text = "Hello, world!";
    assert!(text.contains("Hello"));
    assert!(text.contains("world"));
    assert!(!text.contains("HI"));

    assert!(text.starts_with("Hello")); // Проверка начинается ли с подстроки
    assert!(!text.starts_with("world!"));

    // Проверка заканчивается ли с подстрокой
    assert!(text.ends_with("world!"));
    assert!(!text.ends_with("Hello"));

    assert_eq!(text.find('o'), Some(4)); // 4 - позиция первого вхождения подстроки
    assert_eq!(text.find("wor"), Some(7)); // 7 - позиция первого вхождения подстроки
    assert_eq!(text.find(char::is_whitespace), Some(6)); // Поиск первого пробельного символа
    assert_eq!(text.find(|ch: char| matches!(ch, 'o'..='z')), Some(4)); // Поиск по деапозону.
    assert_eq!(text.find(|ch: char| (ch > 'o') && (ch < 'z')), Some(7)); // Поиск по деапозону.

    assert_eq!(text.rfind('o'), Some(8)); // Поиск с конца
}

/// Удалить пробельные символы
#[test]
fn test_trim() {
    let text = " \t\n\r\n Hello, world!\n\t\r\n ";
    let r = text.trim();
    assert_eq!(r, "Hello, world!");

    let r = text.trim_start();
    assert_eq!(r, "Hello, world!\n\t\r\n ");

    let r = text.trim_end();
    assert_eq!(r, " \t\n\r\n Hello, world!");

    let r = "111 Hello, world! 111".trim_matches(char::is_numeric);
    assert_eq!(r, " Hello, world! ");

    let r = "Hello-2".strip_prefix(char::is_numeric);
    assert_eq!(r, None);
    let r = "Hello-2".strip_suffix(char::is_numeric);
    assert_eq!(r, Some("Hello-"));
    let r = "Hello-2".strip_suffix("-2");
    assert_eq!(r, Some("Hello"));
}

#[test]
fn test_parse() {
    let date = "2025-04-22 13:44:00";

    let r: Result<Vec<_>, _> = date
        .matches(char::is_numeric)
        .map(|ch| ch.parse::<i32>())
        .collect();
    assert_eq!(r, Ok(vec![2, 0, 2, 5, 0, 4, 2, 2, 1, 3, 4, 4, 0, 0]));

    let r: Result<Vec<_>, _> = date
        .split(&['-', ' ', ':'])
        .map(|s| s.parse::<i32>())
        .collect();
    assert_eq!(r, Ok(vec![2025, 4, 22, 13, 44, 0]));
}
#[test]
fn test_escape() {
    // Экранирование специальных символов в строке.
    let r = "\t\n\r\n Hello, world!\n\t\r\n ".escape_default();
    assert_eq!(&r.to_string(), "\\t\\n\\r\\n Hello, world!\\n\\t\\r\\n ");
}

#[test]
fn test_replace() {
    let date = "2025-04-22 13:44:00";

    let text = "Hello, world!";
    let r = text.replace("world", "rust");
    assert_eq!(r, "Hello, rust!");

    let r = date.replace(char::is_numeric, "*");
    assert_eq!(r, "****-**-** **:**:**");
}
