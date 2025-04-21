#![cfg(test)]
// @todo числов в массив байтов и обратно.

use std::iter;

/// Макрос для вывода информации о целочисленом типе числа необходимую для битовой обработки
///
/// Пример использования:
///
/// ```no_run
/// use num_info;
/// num_info!(i8, i16, i32);
/// ```
///
macro_rules! num_info {
    // Если передан только один тип
    ($a:ident) => {
        println!();

        println!("Тип {type_name}", type_name = std::any::type_name::<$a>()); // Имя типа
        println!("MAX: {v} = {v:#b}", v = $a::MIN); // Выводит минимальное значение типа в двоичном формате.
        println!("MIN: {v} = {v:#b}", v = $a::MAX); // Выводит максимальное значение типа в двоичном формате.
        println!("BITS: {}", $a::BITS); // Количество бит в типе.

    };
    // Если передано больше одного типа через запятую
    ($a:ident, $($b:tt)*) => {
        // Рекурсивный вызов
        num_info!($a);      // Первый элемент
        num_info!($($b)*);  // Обработка остальных элементов.
    };
}

/// Макрос для вывода информации о дробном типе числа.
/// Для дробных числе вывод в двоичном формате не поддерживается.
///
/// Пример использования:
///
/// ```no_run
/// use fnum_info;
/// fnum_info!(f32, f64);
/// ```
///
macro_rules! fnum_info {
    // Если передан только один тип
    ($a:ident) => {
        println!();
        println!("Тип {type_name}", type_name = std::any::type_name::<$a>()); // Имя типа
        println!("MAX: {v}", v = $a::MIN); // Выводит минимальное значение типа. Вывод в двоичном формате не поддерживается.
        println!("MIN: {v}", v = $a::MAX); // Выводит максимальное значение типа. Вывод в двоичном формате не поддерживается.

        println!("MIN_POSITIVE: {}", $a::MIN_POSITIVE);
        println!("MIN_EXP: {}", $a::MIN_EXP);
        println!("MAX_EXP: {}", $a::MAX_EXP);
        println!("MIN_10_EXP: {}", $a::MIN_10_EXP);
        println!("MAX_10_EXP:: {}", $a::MAX_10_EXP);
        println!("RADIX: {}", $a::RADIX);
        println!("MANTISSA_DIGITS: {}", $a::MANTISSA_DIGITS);
        println!("DIGITS: {}", $a::DIGITS);
        println!("EPSILON: {}", $a::EPSILON);
    };
    // Если передано больше одного типа через запятую
    ($a:ident,$($b:tt)*) => {
        // Рекурсивный вызов
        fnum_info!($a); // Первый элемент
        fnum_info!($($b)*); // Обработка остальных элементов.ментов.
    };
}

/// Тестирование для целых числе преобразование из бинарного формата в числового.
/// Для каждого переданного типа будет сгенерировано минимальное/максимальное значение в двоичном формате
/// и приобразовано в числовой формат.
///
/// Пример использования:
/// ```no_run
///  test_bits_num!(u8, i16, u32);
/// ```
macro_rules! test_bits_num {
    // Если передан только один тип
    ($a:ident) => {
        println!();

        let type_name = std::any::type_name::<$a>();
        println!("Тип {type_name}"); // Имя типа

        let bits = $a::BITS;        // Количество бит в типе.
        // Минимальное и максимальное число предоставленое в байтах.
        let max_radix;
        let min_radix;

        if type_name.starts_with("i"){
            max_radix = "1".repeat(bits as usize - 1);
            min_radix = format!("-{}{}", "1", "0".repeat(bits as usize - 1));
        } else {
            max_radix = "1".repeat(bits as usize);
            min_radix = "0".repeat(bits as usize);
        };

        // генерация максимального числа по количеству байтов у типа.
        assert_eq!(
            $a::from_str_radix(&max_radix, 2).unwrap(),
            $a::MAX
        );
        // генерация минимально числа по количеству байтов у типа.
        assert_eq!(
            $a::from_str_radix(&min_radix, 2).unwrap(),
            $a::MIN
        );

        println!("MIN: {min_radix}");
        println!("MAX: {max_radix}");
    };
    // Если передано, через запятую, больше одного типа
    ($a:ident, $($b:tt)*) => {
        // Рекурсивный вызов
        test_bits_num!($a);      // Первый элемент
        test_bits_num!($($b)*);  // Обработка остальных элементов.
    };
}

#[allow(clippy::needless_range_loop)]
fn draw_cube<const X: usize, const Y: usize>(matr: [[bool; Y]; X]) {
    for y in 0..Y {
        for x in 0..X {
            if matr[x][y] {
                print!("■ ");
            } else {
                print!("□ ");
            }
        }

        println!()
    }
}

/// Отображение информации о типе числа
#[test]
fn test_num_info() {
    println!();
    println!("Тип i8");
    println!("MAX: {v} = {v:#b}", v = i8::MIN); // Выводит минимальное значение i8 типа в двоичном формате.
    println!("MIN: {v} = {v:#b}", v = i8::MAX); // Выводит максимальное значение i8 типа в двоичном формате.
    println!("BITS: {}", i8::BITS); // Количество бит в i8 типе.

    // Отображение максимальное значения типов со знаком и количество байтов у типа.
    println!();
    println!("Числа со знаком");
    num_info!(i8, i16, i32, i64, i128, isize);

    // Отображение максимальное значения типов без знака и количество байтов у типа.
    println!();
    println!("Числа без знака");
    num_info!(u8, u16, u32, u64, u128, usize);

    println!();
    println!("Дробные числа");
    fnum_info!(f32, f64);
}

/// Конвертация из битового значения в числового
#[test]
fn test_bits_to_numbers() {
    // Число без знака.
    assert_eq!(0b11111111, u8::MAX);
    assert_eq!(0b00000000, u8::MIN);

    // Число со знаком.
    assert_eq!(0b1111111, i8::MAX);
    assert_eq!(-0b10000000_i8, i8::MIN);
    assert_eq!(0b10000000_u8 as i8, i8::MIN);

    // Преобразовать битовое значение в строке в число

    assert_eq!(i8::from_str_radix("-10000000", 2).unwrap(), i8::MIN);
    assert_eq!(u8::from_str_radix("10000000", 2).unwrap() as i8, i8::MIN);
    assert_eq!(i8::from_str_radix("1111111", 2).unwrap(), i8::MAX);

    // для u8.
    // генерация минимально числа по количеству байтов у типа.
    assert_eq!(
        u8::from_str_radix(&"0".repeat(i8::BITS as usize), 2).unwrap(),
        u8::MIN
    );
    // генерация максимального числа по количеству байтов у типа.
    assert_eq!(
        u8::from_str_radix(&"1".repeat(i8::BITS as usize), 2).unwrap(),
        u8::MAX
    );

    // для i8.
    // генерация минимально числа по количеству байтов у типа.
    assert_eq!(
        i8::from_str_radix(
            &format!("-{}{}", "1", &("0".repeat(i8::BITS as usize - 1))),
            2
        )
        .unwrap(),
        i8::MIN
    );
    assert_eq!(
        u8::from_str_radix(
            &format!("{}{}", "1", &("0".repeat(i8::BITS as usize - 1))),
            2
        )
        .unwrap() as i8,
        i8::MIN
    );
    // генерация максимального числа по количеству байтов у типа.
    assert_eq!(
        i8::from_str_radix(&("1").repeat(i8::BITS as usize - 1), 2).unwrap(),
        i8::MAX
    );

    // генерация максимального и минимального числа без знака по количеству байтов у типа.
    test_bits_num!(u8, u16, u32, u64, u128, usize);

    // генерация максимального числа со знаком по количеству байтов у типа.
    test_bits_num!(i8, i16, i32, i64, i128, isize);
}

/// Подсчет количества единиц или нулей в числе в двоичном представлении.
#[test]
fn test_bits_count() {
    // Подсчитать количество единиц в числе в двоичном представлении.
    assert_eq!(u8::MIN.count_ones(), 0); // 0b00000000
    assert_eq!(u8::MAX.count_ones(), u8::BITS); // 0b11111111 => 0b[11111111]
    assert_eq!(0b01010000_u8.count_ones(), 2); // 0b01010000 => 0b0[1]0[1]0000
    assert_eq!((u8::MAX << 2).count_ones(), 6); // 0b11111111 << 2 = 0b00[111111]
    assert_eq!((u8::MIN << 2).count_ones(), 0); // 0b00000000 << 2 = 0b00000000
    assert_eq!((u8::MAX >> 2).count_ones(), 6); // 0b11111111 >> 2 = 0b[111111]00
    assert_eq!((u8::MIN >> 2).count_ones(), 0); // 0b00000000 >> 2 = 0b00000000
    assert_eq!((u8::MAX & 0b00010000).count_ones(), 1); // 0b11111111 & 0b00010000 => 0b000[1]0000
    assert_eq!((u8::MIN & 0b00010000).count_ones(), 0); // 0b000000 & 0b00010000 => 0b00000000
    assert_eq!((u8::MIN | 0b00010000).count_ones(), 1); // 0b00000000 | 0b00010000 => 0b000[1]0000
    assert_eq!((u8::MAX | 0b00010000).count_ones(), u8::BITS); // 0b11111111 | 0b00010000 => 0b[11111111]
    assert_eq!((u8::MIN ^ 0b00010000).count_ones(), 1); // 0b00000000 ^ 0b00010000 => 0b000[1]0000
    assert_eq!((u8::MAX ^ 0b00010000).count_ones(), 7); // 0b11111111 ^ 0b00010000 => 0b[111]0[1111]

    // Подсчитать количество нулей в числе в двоичном представлении.
    assert_eq!(u8::MIN.count_zeros(), u8::BITS); // 0b[00000000]
    assert_eq!(u8::MAX.count_zeros(), 0); // 0b11111111 => 0b11111111
    assert_eq!(0b01010000_u8.count_zeros(), 6); // 0b01010000 => 0b[0]1[0]1[0000]
    assert_eq!((u8::MAX << 2).count_zeros(), 2); // 0b11111111 << 2 = 0b[00]111111
    assert_eq!((u8::MIN << 2).count_zeros(), u8::BITS); // 0b00000000 << 2 = 0b[00000000]
    assert_eq!((u8::MAX >> 2).count_zeros(), 2); // 0b11111111 >> 2 = 0b111111[00]
    assert_eq!((u8::MIN >> 2).count_zeros(), u8::BITS); // 0b00000000 >> 2 = 0b[00000000]
    assert_eq!((u8::MAX & 0b00010000).count_zeros(), 7); // 0b11111111 & 0b00010000 => 0b[000]1[0000]
    assert_eq!((u8::MIN & 0b00010000).count_zeros(), u8::BITS); // 0b000000 & 0b00010000 => 0b00000000
    assert_eq!((u8::MIN | 0b00010000).count_zeros(), 7); // 0b00000000 | 0b00010000 => 0b[000]1[0000]
    assert_eq!((u8::MAX | 0b00010000).count_zeros(), 0); // 0b11111111 | 0b00010000 => 0b11111111
    assert_eq!((u8::MIN ^ 0b00010000).count_zeros(), 7); // 0b00000000 ^ 0b00010000 => 0b[000]1[0000]
    assert_eq!((u8::MAX ^ 0b00010000).count_zeros(), 1); // 0b11111111 ^ 0b00010000 => 0b111[0]1111

    // Подсчитать количество ведущих нулей в числе в двоичном представлении.
    assert_eq!(u8::MIN.leading_zeros(), u8::BITS); // 0b00000000 => 0b[00000000]
    assert_eq!(u8::MAX.leading_zeros(), 0); // 0b11111111 => 0b11111111
    assert_eq!(0b01010000_u8.leading_zeros(), 1); // 0b01010000 => 0b[0]1010000
    assert_eq!((u8::MAX << 2).leading_zeros(), 0); // 0b11111111 << 2 = 0b111111[00]
    assert_eq!((u8::MIN << 2).leading_zeros(), u8::BITS); // 0b00000000 << 2 = 0b[00000000]
    assert_eq!((u8::MAX >> 2).leading_zeros(), 2); // 0b11111111 >> 2 = 0b[00]111111
    assert_eq!((u8::MIN >> 2).leading_zeros(), u8::BITS); // 0b00000000 >> 2 = 0b00000000
    assert_eq!((u8::MAX & 0b00010000).leading_zeros(), 3); // 0b11111111 & 0b00010000 => 0b[000]1000
    assert_eq!((u8::MIN & 0b00010000).leading_zeros(), u8::BITS); // 0b000000 & 0b00010000 => 0b[00000000]
    assert_eq!((u8::MIN | 0b00010000).leading_zeros(), 3); // 0b00000000 | 0b00010000 => 0b[000]1000
    assert_eq!((u8::MAX | 0b00010000).leading_zeros(), 0); // 0b11111111 | 0b00010000 => 0b11111111
    assert_eq!((u8::MIN ^ 0b00010000).leading_zeros(), 3); // 0b00000000 ^ 0b00010000 => 0b[000]1000
    assert_eq!((u8::MAX ^ 0b00010000).leading_zeros(), 0); // 0b11111111 ^ 0b00010000 => 0b1110111

    // Подсчитать количество ведущих едениц в числе в двоичном представлении.
    assert_eq!(u8::MIN.leading_ones(), 0); // 0b00000000 => 0b00000000
    assert_eq!(u8::MAX.leading_ones(), u8::BITS); // 0b11111111 => 0b11111111
    assert_eq!(0b01010000_u8.leading_ones(), 0); // 0b01010000
    assert_eq!((u8::MAX << 2).leading_ones(), 6); // 0b11111111 << 2 = 0b[111111]00
    assert_eq!((u8::MIN << 2).leading_ones(), 0); // 0b00000000 << 2 = 0b00000000
    assert_eq!((u8::MAX >> 2).leading_ones(), 0); // 0b11111111 >> 2 = 0b00111111
    assert_eq!((u8::MIN >> 2).leading_ones(), 0); // 0b00000000 >> 2 = 0b00000000
    assert_eq!((u8::MAX & 0b00010000).leading_ones(), 0); // 0b11111111 & 0b00010000 => 0b00010000
    assert_eq!((u8::MIN & 0b00010000).leading_ones(), 0); // 0b000000 & 0b00010000 => 0b00000000
    assert_eq!((u8::MIN | 0b00010000).leading_ones(), 0); // 0b00000000 | 0b00010000 => 0b00010000
    assert_eq!((u8::MAX | 0b00010000).leading_ones(), u8::BITS); // 0b11111111 | 0b00010000 => 0b[11111111]
    assert_eq!((u8::MIN ^ 0b00010000).leading_ones(), 0); // 0b00000000 ^ 0b00010000 => 0b00010000
    assert_eq!((u8::MAX ^ 0b00010000).leading_ones(), 3); // 0b11111111 ^ 0b00010000 => 0b[111]01111

    // Подсчитать количество завершающих нулей в числе в двоичном представлении.
    assert_eq!(u8::MIN.trailing_zeros(), u8::BITS); // 0b00000000 => 0b[00000000]
    assert_eq!(u8::MAX.trailing_zeros(), 0); // 0b11111111 => 0b11111111
    assert_eq!(0b01010000_u8.trailing_zeros(), 4); // 0b01010000 => 0b0101[0000]
    assert_eq!((u8::MAX << 2).trailing_zeros(), 2); // 0b11111111 << 2 = 0b111111[00]
    assert_eq!((u8::MIN << 2).trailing_zeros(), u8::BITS); // 0b00000000 << 2 = 0b[00000000]
    assert_eq!((u8::MAX >> 2).trailing_zeros(), 0); // 0b11111111 >> 2 = 0b00111111
    assert_eq!((u8::MIN >> 2).trailing_zeros(), u8::BITS); // 0b00000000 >> 2 = 0b[00000000]
    assert_eq!((u8::MAX & 0b00010000).trailing_zeros(), 4); // 0b11111111 & 0b00010000 => 0b0001[0000]
    assert_eq!((u8::MIN & 0b00010000).trailing_zeros(), u8::BITS); // 0b00000000 & 0b00010000 => 0b[00000000]
    assert_eq!((u8::MIN | 0b00010000).trailing_zeros(), 4); // 0b00000000 | 0b00010000 => 0b0001[0000]
    assert_eq!((u8::MAX | 0b00010000).trailing_zeros(), 0); // 0b11111111 | 0b00010000 => 0b11111111
    assert_eq!((u8::MIN ^ 0b00010000).trailing_zeros(), 4); // 0b00000000 ^ 0b00010000 => 0b0001[0000]
    assert_eq!((u8::MAX ^ 0b00010000).trailing_zeros(), 0); // 0b11111111 ^ 0b00010000 => 0b11101111

    // Подсчитать количество завершающих едениц в числе в двоичном представлении.
    assert_eq!(u8::MIN.trailing_ones(), 0); // 0b00000000 => 0b00000000
    assert_eq!(u8::MAX.trailing_ones(), u8::BITS); // 0b11111111 => 0b[11111111]
    assert_eq!(0b01010000_u8.trailing_ones(), 0); // 0b01010000
    assert_eq!((u8::MAX << 2).trailing_ones(), 0); // 0b11111111 << 2 = 0b111111[00]
    assert_eq!((u8::MIN << 2).trailing_ones(), 0); // 0b00000000 << 2 = 0b00000000
    assert_eq!((u8::MAX >> 2).trailing_ones(), 6); // 0b11111111 >> 2 = 0b00[111111]
    assert_eq!((u8::MIN >> 2).trailing_ones(), 0); // 0b00000000 >> 2 = 0b00000000
    assert_eq!((u8::MAX & 0b00010000).trailing_ones(), 0); // 0b11111111 & 0b00010000 => 0b0001000
    assert_eq!((u8::MIN & 0b00010000).trailing_ones(), 0); // 0b00000000 & 0b00010000 => 0b0000000
    assert_eq!((u8::MIN | 0b00010000).trailing_ones(), 0); // 0b00000000 | 0b00010000 => 0b0001000
    assert_eq!((u8::MAX | 0b00010000).trailing_ones(), u8::BITS); // 0b11111111 | 0b00010000 => 0b11111111
    assert_eq!((u8::MIN ^ 0b00010000).trailing_ones(), 0); // 0b00000000 ^ 0b00010000 => 0b0001000
    assert_eq!((u8::MAX ^ 0b00010000).trailing_ones(), 4); // 0b11111111 ^ 0b00010000 => 0b1110[1111]
}

// Свиг битов с одного конца в другой
#[test]
fn test_bits_rotate() {
    // взять бит с конца и добавить в начало
    let value: u8 = 0b00010100;

    println!("rotate_rigth");
    for i in 0..=u8::BITS {
        println!(
            "rotate_right({i}): {k} = {k:0width$b}",
            k = value.rotate_right(i),
            width = u8::BITS as usize
        );
    }
    assert_eq!(0b00000101, value.rotate_right(2)); // 0b000101[00] => 0b[00]000101
    assert_eq!(0b01000001, value.rotate_right(4)); // 0b0001[0100] => 0b[0100]0001

    // взять бит с начала и добавить в конец
    for i in 0..=u8::BITS {
        println!(
            "rotate_left({i}): {k} = {k:0width$b}",
            k = value.rotate_left(i),
            width = u8::BITS as usize
        );
    }
    assert_eq!(0b01010000, value.rotate_left(2)); // 0b[00]010100 => 0b0b010100[00]
    assert_eq!(0b01000001, value.rotate_right(4)); // 0b[0001]0100 => 0b0100[0001]

    // зеркальный сдиг битов с одного конца в другой
    let value: u8 = 0b00000101;
    let new_value = value.reverse_bits(); // 0b00000101 => 0b10100000
    assert_eq!(0b10100000, new_value);
}

/// checked_add - безопасная сложение. c тем же типом
#[test]
fn test_checked_add() {
    let mut result = 0_u8;
    let mut max_iter = 0_u8;
    for v in 0..=u8::MAX {
        let Some(r) = result.checked_add(v) else {
            println!("{result} + {v} = OVERFLOW");
            break;
        };
        println!("{result} + {v} = {r}");
        (max_iter, result) = (v, r);
    }
    println!("Max iter = {}", max_iter);

    assert_eq!(i8::MAX.checked_add(-3), Some(i8::MAX - 3));

    let mut result = 0_u8;
    let max_iter = (1..u8::MAX)
        .find(|i| result.checked_add(*i).map(|r| result = r).is_none())
        .map(|v| v - 1)
        .unwrap_or_default();
    println!("Max iter = {max_iter}");
}

/// checked_add_signed - безопасная сложение c типом тойже размера, но со знаком.
#[test]
fn test_checked_add_signed() {
    assert_eq!(u8::MAX.checked_add_signed(-3), Some(252));
    assert_eq!(u16::MAX.checked_add_signed(-3_i16), Some(u16::MAX - 3));
    // assert_eq!(u16::MAX.checked_add_signed(-3_i32), Some(u16::MAX - 3)); Не соберётся

    // unchecked_add
    unsafe {
        assert_eq!(u8::MIN.unchecked_add(3), 3);
        // assert_eq!(u8::MAX.unchecked_add(3), 3); PANIC
    }
}

/// checked_sub - безопасная вычитание. c тем же типом
#[test]
fn test_checked_sub() {
    assert_eq!(3_u8.checked_sub(0), Some(3));
    assert_eq!(3_u8.checked_sub(2), Some(1));
    assert_eq!(0_u8.checked_sub(3), None);

    let mut result = 0_i8;
    for v in (-10..i8::MAX)
        .map(|v| v.abs())
        .enumerate()
        .map(|(m, mut v)| {
            if m % 3 != 0 {
                v *= -1;
            };
            v
        })
    {
        result = match result.checked_sub(v) {
            Some(r) => {
                println!("{result} - {v} = {r}");
                r
            }
            None => {
                println!("{result} - {v} = OVERFLOW");
                break;
            }
        };
    }
}

/// checked_mul - безопасная умножение
#[test]
fn test_checked_mul() {
    assert_eq!(1_u8.checked_mul(0), Some(0));
    assert_eq!(1_u8.checked_mul(2), Some(2));
    assert_eq!(u8::MAX.checked_mul(2), None);

    let mut result: u8 = 1;
    let last = (1..u8::MAX)
        .find(|v| {
            result
                .checked_mul(*v)
                .map(|r| {
                    println!("{result} * {v} = {r}");
                    result = r
                })
                .is_none()
        })
        .map(|v| v - 1)
        .unwrap_or_default();

    println!("Max iter = {last}");
}

/// checked_div - безопасная деление
#[test]
fn test_checked_div() {
    assert_eq!(3_i8.checked_div(0), None);
    assert_eq!(1_i8.checked_div(3), Some(0));
    assert_eq!(3_i8.checked_div(3), Some(1));

    let mut result: i32 = i32::MAX;
    let mut i = 10;
    loop {
        i -= 1;
        match result.checked_div(i) {
            Some(r) => {
                println!("{result} / {i} = {r}");
                result = r
            }
            None => {
                println!("{result} / {i} = DIV ZERO");
                break;
            }
        };
    }
    println!("Max iter = {i}");

    assert!(i8::MIN.checked_div(-1).is_none());
}

/// checked_rem - безопасный остаток от деления
#[test]
fn test_checked_rem() {
    assert_eq!(3_i8.checked_rem(0), None);
    assert_eq!(1_i8.checked_rem(3), Some(1));
    assert_eq!(3_i8.checked_rem(3), Some(0));
    assert_eq!((-3_i8).checked_rem(2), Some(-1));

    (-10..=10_i32).fold(10_i32, |mut old, n| {
        print!("{old} % {n} = ");

        if let Some(v) = old.checked_rem(n) {
            println!("{v}");
            print!("{old} + {v} = ");
            old += v;
            println!("{old}");
        } else {
            println!("NULL");
        }

        old
    });
}

/// checked_abs - безопасное получение абсолютного(положительного) значения.
/// checked_neg - безопасное получение обратного значения.
#[test]
fn test_checked_neg() {
    // checked_abs
    assert_eq!(None, i8::MIN.checked_abs());
    assert_eq!(i8::MAX, i8::MAX.checked_abs().unwrap());
    assert_eq!(1, (-1_i8).checked_abs().unwrap());
    assert_eq!(1, 1_i8.checked_abs().unwrap());
    // checked_neg
    assert_eq!(None, i8::MIN.checked_neg());
    assert_eq!(-i8::MAX, i8::MAX.checked_neg().unwrap());
    assert_eq!(1, (-1_i8).checked_neg().unwrap());
    assert_eq!(-1, 1_i8.checked_neg().unwrap());

    // checked_abs и checked_neg
    const STEP: f32 = (i8::MIN as f32 / 10.0).abs() * 2.;
    const ITER_STEP: usize = (((i8::MIN as f32).abs() + i8::MAX as f32) / 10.0) as usize;

    let cub: [[bool; 11]; 10] = (i8::MIN..=i8::MAX)
        .step_by(ITER_STEP)
        .filter_map(|v| {
            if v < 0 {
                v.checked_abs().map(|max| (v, max))
            } else {
                v.checked_neg().map(|min| (min, v))
            }
        })
        .map(|(min, max)| (min as f32, max as f32))
        .map(|(min, max)| ((min / STEP).round() as i8, (max / STEP).round() as i8))
        .enumerate()
        .fold(Default::default(), |mut matr, (x, (min, max))| {
            for y in (min..=max).filter_map(|v| (v - 5).checked_abs().map(|v| v as usize)) {
                matr[x][y] = true;
            }

            matr
        });
    draw_cube(cub);
}

/// checked_isqrt - безопасный извлечение корня квадратного, результат будет округлен в меньшую сторону.
#[test]
fn test_checked_sqrt() {
    for n in 0..=i8::MAX {
        println!("{n} = {:?}", n.checked_isqrt());
    }
}

#[test]
fn test_saturating() {
    // saturating_add - безопасное сложение
    assert_eq!(1_u8.saturating_add(0), 1);
    assert_eq!(1_u8.saturating_add(2), 3);
    assert_eq!(u8::MAX.saturating_add(2), u8::MAX);

    // saturating_sub - безопасное вычитание
    assert_eq!(1_u8.saturating_sub(0), 1);
    assert_eq!(1_u8.saturating_sub(2), u8::MIN);
    assert_eq!(u8::MIN.saturating_sub(2), u8::MIN);

    // saturating_mul - безопасное умножение
    assert_eq!(1_u8.saturating_mul(0), 0);
    assert_eq!(1_u8.saturating_mul(2), 2);
    assert_eq!(u8::MAX.saturating_mul(2), u8::MAX);

    // saturating_pow - безопасное возведение в степень
    assert_eq!(1_u8.saturating_pow(0), 1);
    assert_eq!(1_u8.saturating_pow(2), 1);
    assert_eq!(u8::MAX.saturating_pow(2), u8::MAX);

    // saturating_abs - безопасное получение абсолютного(положительного) значения.
    assert_eq!(i8::MAX, i8::MIN.saturating_abs());
    assert_eq!(1, (-1_i8).saturating_abs());
    assert_eq!(1, 1_i8.saturating_abs());

    // saturating_neg - безопасное получение обратного значения.
    assert_eq!(-i8::MAX, i8::MAX.saturating_neg());
    assert_eq!(1, (-1_i8).saturating_neg());
    assert_eq!(-1, 1_i8.saturating_neg());
}

#[test]
fn test_wrapping() {
    // wrapping_add - защищенное сложение
    assert_eq!(1_u8.wrapping_add(0), 1);
    assert_eq!(1_u8.wrapping_add(2), 3);
    assert_eq!(u8::MAX.wrapping_add(2), 1);

    // wrapping_sub - защищенное вычитание
    assert_eq!(1_u8.wrapping_sub(0), 1);
    assert_eq!(1_u8.wrapping_sub(2), 255);
    assert_eq!(u8::MIN.wrapping_sub(2), 254);

    // wrapping_mul - защищенное умножение
    assert_eq!(1_u8.wrapping_mul(0), 0);
    assert_eq!(1_u8.wrapping_mul(2), 2);
    assert_eq!(u8::MAX.wrapping_mul(2), 254);

    // wrapping_pow - защищенное возведение в степень
    assert_eq!(1_u8.wrapping_pow(0), 1);
    assert_eq!(1_u8.wrapping_pow(2), 1);
    assert_eq!(u8::MAX.wrapping_pow(2), 1);

    // wrapping_abs - защищенное получение абсолютного(положительного) значения.
    assert_eq!(-128, i8::MIN.wrapping_abs());
    assert_eq!(1, (-1_i8).wrapping_abs());
    assert_eq!(1, 1_i8.wrapping_abs());

    // wrapping_neg - защищенное получение обратного значения.
    assert_eq!(-i8::MAX, i8::MAX.wrapping_neg());
    assert_eq!(1, (-1_i8).wrapping_neg());
    assert_eq!(-1, 1_i8.wrapping_neg());

    // wrapping_div - защищенное деление
    assert_eq!(1_u8.wrapping_div(1), 1);
    assert_eq!(1_u8.wrapping_div(2), 0);
    assert_eq!(u8::MAX.wrapping_div(2), 127);

    // wrapping_rem - защищенный остаток от деления
    assert_eq!(1_u8.wrapping_rem(1), 0);
    assert_eq!(1_u8.wrapping_rem(2), 1);
    assert_eq!(u8::MAX.wrapping_rem(2), 1);
}

#[test]
fn test_overflowing() {
    // overflowing_add - защищенное сложение
    assert_eq!(1_u8.overflowing_add(0), (1, false));
    assert_eq!(1_u8.overflowing_add(2), (3, false));
    assert_eq!(u8::MAX.overflowing_add(2), (1, true));

    // overflowing_sub - защищенное вычитание
    assert_eq!(1_u8.overflowing_sub(0), (1, false));
    assert_eq!(1_u8.overflowing_sub(2), (255, true));
    assert_eq!(u8::MIN.overflowing_sub(2), (254, true));
    assert_eq!(u8::MAX.overflowing_sub(2), (253, false));

    // overflowing_mul - защищенное умножение
    assert_eq!(1_u8.overflowing_mul(0), (0, false));
    assert_eq!(2_u8.overflowing_mul(2), (4, false));
    assert_eq!(u8::MAX.overflowing_mul(2), (254, true));

    // overflowing_pow - защищенное возведение в степень
    assert_eq!(1_u8.overflowing_pow(0), (1, false));
    assert_eq!(3_u8.overflowing_pow(2), (9, false));
    assert_eq!(u8::MAX.overflowing_pow(2), (1, true));

    // overflowing_abs - защищенное получение абсолютного(положительного) значения.
    assert_eq!((-1_i8).overflowing_abs(), (1, false));
    assert_eq!(i8::MIN.overflowing_abs(), (-128, true));

    // overflowing_neg - защищенное получение обратного значения.
    assert_eq!(i8::MAX.overflowing_neg(), (-i8::MAX, false));
    assert_eq!(i8::MIN.overflowing_neg(), (i8::MIN, true));
    assert_eq!((-1_i8).overflowing_neg(), (1, false));
    assert_eq!(1_i8.overflowing_neg(), (-1, false));
}

/// pow - возведение в степень.
#[test]
fn test_pow() {
    let a = 5_i32; // сторона квадрата
    let s = a.pow(2); // вычисление площади квадрата
    println!("Площадь квадрата с стороной {a} равна {s}");

    let r: f64 = 5.0; // радиус круга
    let s = std::f64::consts::PI * r.powi(2); // вычисление площади круга
    println!("Площадь круга с радиусом {r} равна {s}");

    // Равнобедренный треугольник.
    let h: f32 = 12.0; // высота
    let b: f32 = 10.0; // основание
    println!("Высота  {h}");
    println!("основание {b}");

    let a = h.hypot(b / 2.); // вычисление длины стороны a
    println!("Длина стороны a равна {a}");
}

/// signum - возвращает знак числа.
#[test]
fn test_signum() {
    const L: usize = 10;
    let mut arr: [[bool; 3]; L] = Default::default();
    let rand_iter = iter::repeat_with(rand::random::<i8>).take(L);

    for (tick, rand) in arr.iter_mut().zip(rand_iter) {
        let sig = (rand.signum() + 1) as usize;
        tick[sig] = true;
    }

    draw_cube(arr);
}

/// is_positive - проверяет является ли число положительным.
#[test]
fn test_is_positive() {
    assert!((-1_i8).is_negative());
    assert!(!(-1_i8).is_positive());
    assert!(!1_i8.is_negative());
    assert!(1_i8.is_positive());
    assert!(!0_i8.is_negative());
    assert!(!0_i8.is_positive());

    let print = |num: i8| {
        println!(
            "{num}: \n\
            positive - {:?}\n\
            negative - {:?}",
            num.is_positive(),
            num.is_negative()
        );
    };
    for num in iter::repeat_with(rand::random::<i8>).take(10) {
        print(num);
    }
    print(0);
}

/// floor - округляет число в меньшую сторону.
/// ceil - округляет число в большую сторону.
/// round - округляет число к ближайшему целому.
/// round_ties_even - округляет число к ближайшему целому, если дробная часть равна 0.5, то округляет к ближайшему четному целому.
#[test]
fn test_round() {
    assert_eq!(1.3_f32.floor(), 1.0);
    assert_eq!(1.9_f32.floor(), 1.0);
    assert_eq!((-2.0_f32).floor(), -2.0);
    assert_eq!((-2.3_f32).floor(), -3.0);
    assert_eq!((-2.9_f32).floor(), -3.0);

    assert_eq!(1.3_f32.ceil(), 2.0);
    assert_eq!(1.9_f32.ceil(), 2.0);
    assert_eq!((-2.0_f32).ceil(), -2.0);
    assert_eq!((-2.3_f32).ceil(), -2.0);
    assert_eq!((-2.9_f32).ceil(), -2.0);

    assert_eq!(1.3_f32.round(), 1.0);
    assert_eq!(1.9_f32.round(), 2.0);
    assert_eq!((-2.0_f32).round(), -2.0);
    assert_eq!((-2.3_f32).round(), -2.0);
    assert_eq!((-2.9_f32).round(), -3.0);

    assert_eq!(1.3_f32.round_ties_even(), 1.0);
    assert_eq!(1.5_f32.round_ties_even(), 2.0);
    assert_eq!(1.9_f32.round_ties_even(), 2.0);
    assert_eq!((-2.0_f32).round_ties_even(), -2.0);
    assert_eq!((-2.3_f32).round_ties_even(), -2.0);
    assert_eq!((-2.5_f32).round_ties_even(), -2.0);
    assert_eq!((-2.9_f32).round_ties_even(), -3.0);
}

/// fract - возвращает дробную часть числа.
/// trunc - отбрасывает дробную часть числа.
#[test]
fn test_fract() {
    assert_eq!(1.3_f32.fract(), 0.29999995);
    assert_eq!(1.9_f32.fract(), 0.9);
    assert_eq!((-2.0_f32).fract(), 0.0);
    assert_eq!((-2.3_f32).fract(), -0.29999995);
    assert_eq!((-2.9_f32).fract(), -0.9000001);

    assert_eq!(1.3_f32.trunc(), 1.0);
    assert_eq!(1.9_f32.trunc(), 1.0);
    assert_eq!((-2.0_f32).trunc(), -2.0);
    assert_eq!((-2.3_f32).trunc(), -2.0);
    assert_eq!((-2.9_f32).trunc(), -2.0);
}

/// mul_add - умножает первое число на второе и прибавляет третье.
#[test]
fn test_mul_add() {
    assert_eq!(1_f32.mul_add(2.0, 3.0), 5.0);
    assert_eq!(2_f32.mul_add(-2.0, 3.0), -1.0);
    assert_eq!(3_f32.mul_add(-2.0, -3.0), -9.0);
    assert_eq!(4_f32.mul_add(2.0, -3.0), 5.0);
}

/// exp - возвращает число в степени числа.
/// exp2 - возвращает 2 в степени числа.
#[test]
fn test_exp() {
    assert_eq!(2_f32.exp(), 7.389056);
    assert_eq!(4_f32.exp(), 54.59815);
    assert_eq!(2_f32.exp2(), 4.0);
    assert_eq!(4_f32.exp2(), 16.0);
}

/// cbrt - возвращает кубический корень числа.
#[test]
fn test_cbrt() {
    assert_eq!(1_f32.cbrt(), 1.0);
    assert_eq!(8_f32.cbrt(), 2.0);
    assert_eq!(-8_f32.cbrt(), -2.0);
    assert_eq!(27_f32.cbrt(), 3.0);
    assert_eq!(64_f32.cbrt(), 4.0);
    assert_eq!(125_f32.cbrt(), 5.0);
    assert_eq!(216_f32.cbrt(), 6.0);
}

/// hypot - возвращает гипотенузу прямоугольного треугольника.
#[test]
fn test_hypot() {
    let a = 6.0_f32;
    let b = 8.0_f32;
    let c = a.hypot(b);
    assert_eq!(c, 10.);
    println!("Длина гипотенузы равна {c}");
}

/// max - возвращает максимальное число.
/// min - возвращает минимальное число.
#[test]
fn test_max_min() {
    assert_eq!(1_f32.max(2.0), 2.0);
    assert_eq!(2_f32.max(1.0), 2.0);
    assert_eq!((-2_f32).max(1.0), 1.0);
    assert_eq!((-2_f32).max(-1.0), -1.0);

    assert_eq!(1_f32.min(2.0), 1.0);
    assert_eq!(2_f32.min(1.0), 1.0);
    assert_eq!((-2_f32).min(1.0), -2.0);
    assert_eq!((-2_f32).min(-1.0), -2.0);
}

/// midpoint - возвращает среднее арифметическое двух чисел.
#[test]
fn test_midpoint() {
    assert_eq!(1_f32.midpoint(2.0), 1.5);
    assert_eq!(2_f32.midpoint(1.0), 1.5);
    assert_eq!((-2_f32).midpoint(1.0), -0.5);
    assert_eq!((-2_f32).midpoint(-1.0), -1.5);
}

/// total_cmp - сравнивает два числа по значению.
/// partial_cmp - сравнивает два числа по значению.
/// cmp - сравнивает два числа по значению.
#[test]
fn test_cmp() {
    assert_eq!(1_f32.total_cmp(&2.0), std::cmp::Ordering::Less);
    assert_eq!(2_f32.total_cmp(&1.0), std::cmp::Ordering::Greater);
    assert_eq!((-2_f32).total_cmp(&1.0), std::cmp::Ordering::Less);
    assert_eq!((-2_f32).total_cmp(&-1.0), std::cmp::Ordering::Less);
    assert_eq!(1_f32.total_cmp(&f32::NAN), std::cmp::Ordering::Less);
    assert_eq!(f32::NAN.total_cmp(&2.0), std::cmp::Ordering::Greater);

    assert_eq!(1_f32.partial_cmp(&2.0), Some(std::cmp::Ordering::Less));
    assert_eq!(2_f32.partial_cmp(&1.0), Some(std::cmp::Ordering::Greater));
    assert_eq!((-2_f32).partial_cmp(&1.0), Some(std::cmp::Ordering::Less));
    assert_eq!((-2_f32).partial_cmp(&-1.0), Some(std::cmp::Ordering::Less));
    assert_eq!(1_f32.partial_cmp(&f32::NAN), None);
    assert_eq!(f32::NAN.partial_cmp(&2.0), None);
    assert_eq!(f32::NAN.partial_cmp(&f32::NAN), None);
}

/// copysign - возвращает число с знаком переданного числа.
#[test]
fn test_copysign() {
    assert_eq!(1_f32.copysign(2.0), 1.0);
    assert_eq!(1_f32.copysign(-2.0), -1.0);
    assert_eq!((-1_f32).copysign(2.0), 1.0);
    assert_eq!((-1_f32).copysign(-2.0), -1.0);
}
