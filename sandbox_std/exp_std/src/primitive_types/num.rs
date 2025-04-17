#![cfg(test)]
// @todo числов в массив байтов и обратно.

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

/// Безапасные операции
#[test]
fn test_num_safe() {
    //
}
