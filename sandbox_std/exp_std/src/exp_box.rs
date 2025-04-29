//! Тип `Box` позволяет управлять данными в куче.
//! В отличие от значений с фиксированными размерами и временем жизни, которые хранятся в стеке,
//! Box позволяет выделять память для известного типа в куче.
//! После размещения в куче значение принадлежит объекту Box.
//! И когда значение выходит за пределы области видимости, Box автоматически освобождает память.
//!
//!
//! Задача:
//! Написать простенький парсер json который будет работать с потоком данных
//! и хранить в себе только range и обязательно использовать где-нибудь Box.

#![cfg(test)]

use std::ops::Range;
use std::{iter, vec};

use thiserror::Error;
use tracing::debug;
use tracing_test::traced_test;

/// Обработанный элемент разметки
#[derive(Debug, PartialEq, Clone)]
struct Markup {
    position: Range<usize>,  // диапазон позиции в строке
    optional: MarkupOptions, // параметры разметки
}

impl Markup {
    /// Преобразовать разметку в строку
    fn display(&self, source: &str, level: usize) -> String {
        match &self.optional {
            MarkupOptions::UnTyped
            | MarkupOptions::DoubleQuote
            | MarkupOptions::SingleQuote
            | MarkupOptions::RawChar(..) => source[self.position.clone()].to_string(),
            // Массив или объект
            MarkupOptions::Array { nested, .. } | MarkupOptions::Object { nested, .. } => {
                // Дочерние элементы
                let chields = nested
                    .iter()
                    .map(|v| v.display(source, level + 1))
                    .collect::<Vec<String>>();

                // Табуляция для вложенных элементов
                let tab = "   ".repeat(level + 1);
                let chields = if chields.iter().map(|v| v.len()).sum::<usize>() > 30 {
                    format!("\n{tab}{}\n", chields.join(&format!(",\n{tab}")))
                } else {
                    chields.join(", ")
                };

                if matches!(self.optional, MarkupOptions::Object { .. }) {
                    format!("{{{chields}}}") // Объект
                } else {
                    format!("[{chields}]") // Массив
                }
            }
            // Ключ значение для объекта
            MarkupOptions::KeyValue { key, value } => {
                format!(
                    "{}: {}",
                    key.display(source, level),
                    value.display(source, level)
                )
            }
        }
    }

    fn char_type(&self) -> Option<CharType> {
        let Self {
            optional: MarkupOptions::RawChar(char_type),
            ..
        } = self
        else {
            return None;
        };
        Some(*char_type)
    }
}

impl From<(usize, char)> for Markup {
    fn from((position, ch): (usize, char)) -> Self {
        Self {
            position: position..position + ch.len_utf8(),
            optional: MarkupOptions::RawChar(ch.into()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MarkupOptions {
    RawChar(CharType), // Необработанный символ
    UnTyped,           // Неизвестный тип. Может использоваться для числа и ключа в объекте
    DoubleQuote,       // Двойные кавычки
    SingleQuote,       // Одинарные кавычки
    // Массив
    Array {
        finished: bool,
        nested: Vec<Markup>,
    },
    // Объект
    Object {
        finished: bool,
        nested: Vec<Markup>,
    },
    // Ключ значение для объекта
    KeyValue {
        key: Box<Markup>,
        value: Box<Markup>,
    },
}

/// Тип входящего символа
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum CharType {
    Space,
    Char,
    Escape,
    DoubleQuote,
    SingleQuote,
    SquareBracketsOpen,
    SquareBracketsClose,
    CurlyBracketsOpen,
    CurlyBracketsClose,
    Colon,
    Comma,
}

impl From<char> for CharType {
    fn from(value: char) -> Self {
        match value {
            '\'' => CharType::SingleQuote,
            '"' => CharType::DoubleQuote,
            '\\' => CharType::Escape,
            '[' => CharType::SquareBracketsOpen,
            ']' => CharType::SquareBracketsClose,
            ',' => CharType::Comma,
            '{' => CharType::CurlyBracketsOpen,
            '}' => CharType::CurlyBracketsClose,
            ':' => CharType::Colon,
            _ if value.is_whitespace() => CharType::Space,
            _ => CharType::Char,
        }
    }
}

#[derive(Debug, Error, PartialEq)]
enum MarkupError {
    #[error("Не обработанный символ {0}")]
    RawChar(usize),

    #[error("Двойная кавычка не закрыта {0}")]
    DoubleQuote(usize),

    #[error("Одинарная кавычка не закрыта {0}")]
    SingleQuote(usize),

    #[error("Не найдена открывающая квадратная скобка {0}")]
    SquareBracketsNotFoundOpen(usize),

    #[error("Не найдена закрывающая квадратная скобка {0}")]
    SquareBracketsNotFoundClose(usize),

    #[error("Не найдена открывающая фигурная скобка {0}")]
    CurlyBracketsNotFoundOpen(usize),

    #[error("Не найдена закрывающая фигурная скобка {0}")]
    CurlyBracketsNotFoundClose(usize),

    #[error("Ожидался ключ {0}")]
    ExpectedKey(usize),

    #[error("Ожидалось значение {0}")]
    ExpectedValue(usize),

    #[error("Элементы массива должны быть разделены запятой. {0}")]
    MustSeparatedComma(usize),

    #[error("Элементы ожидался разделитель двоеточие. {0}")]
    MustSeparatedColon(usize),

    #[error("Неправильное экранирование {0}")]
    Escape(usize),
}

/// Распарсить строку в поток из разметки.
fn parse(input: &str) -> impl Iterator<Item = Result<Markup, MarkupError>> + use<'_> {
    parse_by_iterator(input.char_indices())
}

/// Преобразовать поток из символов в поток из разметки
fn parse_by_iterator(
    stream_chars: impl Iterator<Item = (usize, char)>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    // Типизируем входящие символы
    let stream_markup = stream_chars.map(Into::into);
    // Обработать экранирование
    let stream_markup = escape(stream_markup);

    // Объединяем пробелы и символы в отрезки
    let stream_markup = group_chars_and_spaces(stream_markup)
        // убираем пробелы больше они не понадобятся
        .filter(|item| {
            !matches!(
                item,
                Ok(Markup {
                    optional: MarkupOptions::RawChar(CharType::Space),
                    ..
                })
            )
        });
    // Сгруппировать кавычки
    let stream_markup = group_quotes(stream_markup)
        // Преобразовываем не используемый диапазоны символов в разметку.
        .map(converted_range_chars_to_markup);

    // Сгруппировать объекты
    let stream_markup = group_object(stream_markup);

    // Сгруппировать массивы
    let stream_markup = group_arrays(stream_markup);

    stream_markup.map(processing)
}

/// Отобразить содержимое в разметке
fn display(markup: &Markup, source: &str) -> String {
    markup.display(source, 0)
}

/// Обработка экранирования. За экранированный символ изменяется на тип Char
fn escape(
    mut stream_markup: impl Iterator<Item = Markup>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    let mut back_item = None;

    iter::from_fn(move || {
        for item in stream_markup.by_ref() {
            match (back_item.as_mut(), item.char_type()) {
                // обработка следующего символа после экранирования
                (Some(Markup { position, optional }), _) => {
                    position.end = item.position.end;
                    *optional = MarkupOptions::RawChar(CharType::Char);
                    let mut item = None;
                    std::mem::swap(&mut back_item, &mut item);
                    return item.map(Ok);
                }
                // Символ экранирования
                (None, Some(CharType::Escape)) => back_item = Some(item),
                // Все остальные символы просто возвращаем
                _ => return Some(Ok(item)),
            }
        }
        let Markup { position, .. } = back_item.take()?;
        // Строка завершилась экранированием
        Some(Err(MarkupError::Escape(position.start)))
    })
}

/// Сгруппировать все символы и пробелы идущие подряд
fn group_chars_and_spaces(
    mut stream_markup: impl Iterator<Item = Result<Markup, MarkupError>>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    let mut back_item: Option<Markup> = None;

    iter::from_fn(move || {
        for item in stream_markup.by_ref() {
            let Ok(item) = item else {
                return Some(item);
            };

            match (back_item.as_mut(), item.char_type()) {
                // Первый элемент
                (None, _) => back_item = Some(item),
                // Если это не пробел и не символ то не нужно группировать
                (_, item_type)
                    if item_type != Some(CharType::Space) && item_type != Some(CharType::Char) =>
                {
                    let mut item = Some(item);
                    std::mem::swap(&mut back_item, &mut item);
                    return item.map(Ok);
                }
                // Новый тип такой же как и предыдущий
                (
                    Some(Markup {
                        optional: MarkupOptions::RawChar(back_type),
                        position: back_position,
                    }),
                    Some(item_type),
                ) if *back_type == item_type => {
                    back_position.end = item.position.end;
                }
                // Тип изменился
                (Some(..), _) => {
                    let mut item = Some(item);
                    std::mem::swap(&mut back_item, &mut item);
                    return item.map(Ok);
                }
            }
        }

        back_item.take().map(Ok)
    })
}

/// Сгруппировать содержимое в скобках
fn group_quotes(
    mut stream_markup: impl Iterator<Item = Result<Markup, MarkupError>>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    let mut back_item = None;

    iter::from_fn(move || {
        for item in stream_markup.by_ref() {
            let Ok(item) = item else {
                return Some(item);
            };

            match (back_item.as_mut(), &item) {
                // Закрытие кавычки
                (
                    Some(Markup {
                        position: back_position,
                        optional: back_optional,
                    }),
                    Markup {
                        position: new_position,
                        optional: new_optional,
                    },
                ) if back_optional == new_optional => {
                    back_position.end = new_position.end;

                    match back_optional {
                        MarkupOptions::RawChar(CharType::SingleQuote) => {
                            *back_optional = MarkupOptions::SingleQuote
                        }
                        MarkupOptions::RawChar(CharType::DoubleQuote) => {
                            *back_optional = MarkupOptions::DoubleQuote
                        }
                        _ => (),
                    }

                    let mut item = None;
                    std::mem::swap(&mut back_item, &mut item);
                    return item.map(Ok);
                }
                // Внутри кавычек
                (Some(Markup { position, .. }), _) => {
                    position.end = item.position.end;
                }
                // Открытие кавычки
                (
                    None,
                    Markup {
                        optional:
                            MarkupOptions::RawChar(CharType::DoubleQuote | CharType::SingleQuote),
                        ..
                    },
                ) => back_item = Some(item),
                // Вне кавычек
                _ => return Some(Ok(item)),
            }
        }

        let Markup { position, optional } = back_item.take()?;

        // Какая-то кавычка не закрыта
        let err = match optional {
            MarkupOptions::RawChar(CharType::SingleQuote) => {
                Err(MarkupError::SingleQuote(position.start))
            }
            MarkupOptions::RawChar(CharType::DoubleQuote) => {
                Err(MarkupError::DoubleQuote(position.start))
            }
            _ => unreachable!(),
        };
        Some(err)
    })
}

/// Сгруппировать содержимой в фигурных скобках
/// Валидация содержимого происходит позже
fn group_object(
    mut stream_markup: impl Iterator<Item = Result<Markup, MarkupError>>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    let mut stack = Vec::new();

    iter::from_fn(move || {
        for item in stream_markup.by_ref() {
            let Ok(item) = item else {
                return Some(item);
            };

            match (stack.is_empty(), item.char_type()) {
                // Квадратные скобки открываются
                (_, Some(CharType::CurlyBracketsOpen)) => stack.push(item),
                // Ошибка - Квадратные скобки закрываются, но нет родителя
                (true, Some(CharType::CurlyBracketsClose)) => {
                    return Some(Err(MarkupError::CurlyBracketsNotFoundOpen(
                        item.position.start,
                    )));
                }
                (_, Some(CharType::CurlyBracketsClose)) => {
                    let Some(open_index) = stack.iter().rposition(|item| {
                        item.optional == MarkupOptions::RawChar(CharType::CurlyBracketsOpen)
                    }) else {
                        return Some(Err(MarkupError::CurlyBracketsNotFoundOpen(
                            item.position.start,
                        )));
                    };

                    // Обновляем диапазон
                    stack[open_index].position.end = item.position.end;

                    // Получаем вложенные элементы
                    let nested = stack.split_off(open_index + 1);

                    // Обновляем тип разметки
                    stack[open_index].optional = MarkupOptions::Object {
                        finished: false,
                        nested,
                    };

                    if stack.len() == 1 {
                        // Если элемент на верхнем уровне, то возвращаем его
                        return Some(Ok(stack.pop()?));
                    }
                }
                // другие элементы. Нет родителя
                (true, _) => return Some(Ok(item)),
                // другие элементы. Есть родитель
                (false, _) => stack.push(item),
            }
        }

        let position = stack.pop()?.position;
        Some(Err(MarkupError::CurlyBracketsNotFoundClose(position.start)))
    })
}

fn group_arrays(
    mut stream_markup: impl Iterator<Item = Result<Markup, MarkupError>>,
) -> impl Iterator<Item = Result<Markup, MarkupError>> {
    let mut stack = Vec::new();

    iter::from_fn(move || {
        for item in stream_markup.by_ref() {
            let Ok(item) = item else {
                return Some(item);
            };

            match (stack.is_empty(), item.char_type()) {
                // Квадратные скобки открываются
                (_, Some(CharType::SquareBracketsOpen)) => stack.push(item),
                // Ошибка - Квадратные скобки закрываются, но нет родителя
                (true, Some(CharType::SquareBracketsClose)) => {
                    return Some(Err(MarkupError::SquareBracketsNotFoundOpen(
                        item.position.start,
                    )));
                }
                (_, Some(CharType::SquareBracketsClose)) => {
                    let Some(open_index) = stack.iter().rposition(|item| {
                        item.optional == MarkupOptions::RawChar(CharType::SquareBracketsOpen)
                    }) else {
                        return Some(Err(MarkupError::SquareBracketsNotFoundOpen(
                            item.position.start,
                        )));
                    };

                    // Обновляем диапазон
                    stack[open_index].position.end = item.position.end;

                    // Получаем вложенные элементы
                    let nested = stack.split_off(open_index + 1);

                    // Проверка на разделение запятой вынесено после обработки объектов

                    // Обновляем тип разметки
                    stack[open_index].optional = MarkupOptions::Array {
                        finished: false,
                        nested,
                    };

                    if stack.len() == 1 {
                        // Если элемент на верхнем уровне, то возвращаем его
                        return Some(Ok(stack.pop()?));
                    }
                }
                // другие элементы. Нет родителя
                (true, _) => return Some(Ok(item)),
                // другие элементы. Есть родитель
                (false, _) => stack.push(item),
            }
        }

        let position = stack.pop()?.position;
        Some(Err(MarkupError::SquareBracketsNotFoundClose(
            position.start,
        )))
    })
}

fn processing(mut item_result: Result<Markup, MarkupError>) -> Result<Markup, MarkupError> {
    let Ok(item) = &mut item_result else {
        return item_result;
    };

    processing_mut(item)?;

    item_result
}

fn processing_mut(item: &mut Markup) -> Result<(), MarkupError> {
    match &mut item.optional {
        MarkupOptions::Array { .. } => processing_arrays_mut(item)?,
        MarkupOptions::Object { .. } => processing_object_mut(item)?,
        MarkupOptions::KeyValue { key, value } => {
            processing_mut(key.as_mut())?;
            processing_mut(value.as_mut())?;
        }
        _ => (),
    };
    validate_unused_chars(item)
}

/// Преобразовываем не используемый диапазоны символов в разметку
fn converted_range_chars_to_markup(
    item: Result<Markup, MarkupError>,
) -> Result<Markup, MarkupError> {
    let Ok(Markup {
        optional: MarkupOptions::RawChar(CharType::Char),
        position,
    }) = item
    else {
        return item;
    };

    Ok(Markup {
        position,
        optional: MarkupOptions::UnTyped,
    })
}

/// Пост обработка массива. Валидация и удаление запятых
fn processing_arrays_mut(item: &mut Markup) -> Result<(), MarkupError> {
    let Markup {
        optional: MarkupOptions::Array { finished, nested },
        ..
    } = item
    else {
        return Ok(());
    };

    nested.iter().skip(1).step_by(2).try_for_each(|item| {
        if item.char_type() != Some(CharType::Comma) {
            Err(MarkupError::MustSeparatedComma(item.position.start))
        } else {
            Ok(())
        }
    })?;

    nested.retain(|v| v.char_type() != Some(CharType::Comma));
    nested.iter_mut().try_for_each(processing_mut)?;
    *finished = true;

    Ok(())
}

/// Пост обработка объекта. Валидация и удаление запятых
fn processing_object_mut(item: &mut Markup) -> Result<(), MarkupError> {
    let Markup {
        optional: MarkupOptions::Object { finished, nested },
        ..
    } = item
    else {
        return Ok(());
    };

    let mut it = nested.clone().into_iter();
    let mut group_result = Vec::new();

    loop {
        let Some(key) = it.next() else {
            break;
        };
        if key.char_type().is_some() {
            return Err(MarkupError::ExpectedKey(key.position.start));
        }

        let colon = it
            .next()
            .ok_or(MarkupError::MustSeparatedColon(key.position.start))?;
        if colon.char_type() != Some(CharType::Colon) {
            return Err(MarkupError::MustSeparatedColon(key.position.start));
        }

        let value = it
            .next()
            .ok_or(MarkupError::ExpectedValue(colon.position.end))?;
        if value.char_type().is_some() {
            return Err(MarkupError::ExpectedValue(value.position.end));
        }

        group_result.push(Markup {
            position: key.position.start..value.position.end,
            optional: MarkupOptions::KeyValue {
                key: Box::new(key),
                value: Box::new(value),
            },
        });

        let Some(comma) = it.next() else {
            break;
        };

        if comma.char_type() != Some(CharType::Comma) {
            return Err(MarkupError::MustSeparatedComma(comma.position.start));
        }
    }

    *nested = group_result;

    nested.retain(|v| v.char_type() != Some(CharType::Comma));
    nested.iter_mut().try_for_each(processing_mut)?;
    *finished = true;

    Ok(())
}

fn validate_unused_chars(item: &Markup) -> Result<(), MarkupError> {
    match item {
        Markup {
            optional: MarkupOptions::RawChar(_),
            position,
        } => Err(MarkupError::RawChar(position.start)),
        Markup {
            optional: MarkupOptions::Array { nested, .. },
            ..
        } => nested.iter().try_for_each(validate_unused_chars),
        _ => Ok(()),
    }
}

#[test]
#[traced_test]
fn test_exp_box() {
    for (input, expected_output) in [
        ("5", vec!["5"]),
        ("-5", vec!["-5"]),
        ("5.1", vec!["5.1"]),
        ("-5.1", vec!["-5.1"]),
        ("   \n\r\t-5.1", vec!["-5.1"]),
        ("-5.1   \n\r\t", vec!["-5.1"]),
        ("5.", vec!["5."]),
        ("5    5", vec!["5", "5"]),
        ("Слово", vec!["Слово"]),
        ("Слово   ", vec!["Слово"]),
        ("   Слово", vec!["Слово"]),
        (" ---   ---  ", vec!["---", "---"]),
        (r#""5""#, vec![r#""5""#]),
        ("  \n\t\r\"5\"", vec![r#""5""#]),
        ("\"5\"  \n\t\r", vec![r#""5""#]),
        (r#""   Word""#, vec![r#""   Word""#]),
        (r#""Слово   ""#, vec![r#""Слово   ""#]),
        (
            "\"Слово\nс\n\tПереносом\"",
            vec!["\"Слово\nс\n\tПереносом\""],
        ),
        (r#"   "Слово"   "#, vec!["\"Слово\""]),
        ("   \"Сло'во\"   ", vec!["\"Сло'во\""]),
        (r#"   'Слово'   "#, vec!["'Слово'"]),
        (r#"   'Сло"во 9'   "#, vec!["'Сло\"во 9'"]),
        ("   'Сло\\'во 10'   ", vec!["'Сло\\'во 10'"]),
        ("1 2 3", vec!["1", "2", "3"]),
        ("'|' Слово '|'", vec!["'|'", "Слово", "'|'"]),
        ("['1']", vec!["['1']"]),
        ("[\"1\"  ]", vec!["[\"1\"]"]),
        ("[1,2,3]", vec!["[1, 2, 3]"]),
        ("[1,2]", vec!["[1, 2]"]),
        ("[1,22,5555]", vec!["[1, 22, 5555]"]),
        ("[1,\"22\",5555]", vec!["[1, \"22\", 5555]"]),
        ("[1,\"22\",'5555']", vec!["[1, \"22\", '5555']"]),
        ("[1,[2]]", vec!["[1, [2]]"]),
        ("[1, 2 , 3,]", vec!["[1, 2, 3]"]),
        ("[1,]", vec!["[1]"]),
        (
            "[1,'2',\"3\",[4,5,[6,[7,'8','[9]']]]]",
            vec!["[\n   1,\n   '2',\n   \"3\",\n   [4, 5, [6, [7, '8', '[9]']]]\n]"],
        ),
        ("{a:b}", vec!["{a: b}"]),
        ("{a:b, c:d}", vec!["{a: b, c: d}"]),
        (
            "{'a':b, c:\"d\", e:{'f':'j'}}",
            vec!["{'a': b, c: \"d\", e: {'f': 'j'}}"],
        ),
        ("[1,{a:b}]}", vec!["[1, {a: b}]"]),
    ] {
        for (markup_result, expected_result) in parse(input).zip(&expected_output) {
            debug!(
                "результат: {markup_result:#?}\n\
                    Вводные данные: {input:#?}\n\
                    Ожидаемый результат: {expected_output:#?}"
            );
            let markup = markup_result.unwrap();
            let result_str = display(&markup, input);
            debug!("result: {result_str}");
            assert_eq!(&result_str, expected_result, "Структура: {markup:#?}");
        }
    }

    for input in [
        "\"demo",
        "demo\"",
        "demo'",
        "'demo",
        "   'demo",
        "   demo' demo",
        "   \"demo demo",
        "[1 2]",
        "[1, 2 3,]",
        "[1,\"22,5555]",
        "[1,22,5555''']",
        "[1,22,5555'']",
        "{a}",
        "{a,b}",
        "{,a:b}",
        "{a:b,c}",
        "{a:b,c:}",
    ] {
        assert!(
            parse(input).any(|result| result.is_err()),
            "Вводные данные: {input:?}"
        );
    }
}
