#![cfg(test)]

// Однопаточные. Не реализуют Sync
// Cell<T>
// RefCell<T>
// OnceCell<T>

// Многопоточные. Реализуют Sync
// Mutex<T>
// RwLock<T>
// OnceLock<T>
// atomic

// Rc<T>
// Arc<T>

mod TestCell {
    // Cell<T> Обычно используется для более простых типов,
    //  где копирование или перемещение значений не требует больших ресурсов (например, для чисел),
    //  и по возможности его следует предпочитать другим типам ячеек.
    // Для более крупных типов, не поддерживающих копирование, RefCell предоставляет некоторые преимущества.
}

mod test_rc {
    // Rc<T> (Reference Counted): Умный указатель для подсчёта ссылок,
    // позволяющий иметь несколько владельцев одного значения в куче.
    // Используется в однопоточных приложениях.

    use std::{collections::BTreeMap, rc::Rc};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_rc() {
        let mut a = Rc::new(1);
        assert!(Rc::get_mut(&mut a).is_some());
        assert_eq!(1, Rc::strong_count(&a)); // Ссылок на данные: 1
        assert_eq!(*a, 1);

        *Rc::make_mut(&mut a) += 1;
        assert_eq!(2, *a);

        {
            let b = a.clone();
            assert_eq!(2, Rc::strong_count(&a)); // Ссылок на данные: 2
            let _c = b;
            assert_eq!(3, Rc::strong_count(&a)); // Ссылок на данные: 3

            assert!(Rc::get_mut(&mut a).is_none()); // Нельзя получить. Ссылок больше чем одна.
        }
        assert!(Rc::get_mut(&mut a).is_some()); // Можно получить. Только А ссылается на данные.

        {
            let b = a.clone();
            assert_eq!(2, Rc::strong_count(&a));
            assert_eq!(*b, *a); // значение одинаковое = 2
            assert!(Rc::ptr_eq(&a, &b)); // Ссылки на данные указывают на одно место памяти.

            *Rc::make_mut(&mut a) += 1; // Ссылок больше одной. Копируются данные

            // Теперь a и b указывают на разные области памяти.
            assert_ne!(*a, *b);
            assert_eq!(3, *a);
            assert_eq!(2, *b);
            assert_eq!(1, Rc::strong_count(&a));
            assert_eq!(1, Rc::strong_count(&b));
            assert!(!Rc::ptr_eq(&a, &b)); // Ссылки на данные указывают на разные места памяти.

            let _c = a.clone();
            assert_eq!(2, Rc::strong_count(&a));
        }

        let a = "demo".to_string();
        let ptr_a = a.as_ptr();
        let a = Rc::new(a);
        let v = Rc::unwrap_or_clone(a); // a - после не будет существовать. Передаётся владения на данные v.
        assert_eq!(v, "demo");
        let ptr_v = v.as_ptr();
        assert_eq!(ptr_a, ptr_v);

        let a = "demo".to_string();
        let ptr_a = a.as_ptr();
        let a = Rc::new(a);
        let _b = a.clone(); // Создается вторая ссылка на данные.
        let v = Rc::unwrap_or_clone(a); // Есть вторая ссылка на данные. Данные будут скопированы.
        assert_eq!(v, "demo");
        let ptr_v = v.as_ptr();
        assert_ne!(ptr_a, ptr_v);
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Block {
        number: u64,
        divider: BTreeMap<u64, Rc<Block>>,
    }

    #[test]
    fn test_vert() {
        let mut nums: BTreeMap<u64, Rc<Block>> = Default::default();

        for number in 2..=100 {
            let mut block = Block {
                number,
                divider: Default::default(),
            };

            for (num_f, block_f) in nums.iter().rev() {
                if number % num_f != 0 || block.divider.contains_key(num_f) {
                    continue;
                }

                block.divider.insert(*num_f, block_f.clone());
                block.divider.append(&mut block_f.divider.clone());
            }
            nums.insert(number, block.into());
        }

        println!("Блоки: {nums:#?}");
    }
}

mod TestRefCell {
    // RefCell<T> использует время жизни объектов в Rust для реализации «динамического заимствования» — процесса,
    // при котором можно получить временный, исключительный, изменяемый доступ к внутреннему значению.
    // Заимствования для RefCell<T> отслеживаются во время выполнения, в отличие от собственных ссылочных
    // типов Rust, которые полностью отслеживаются статически, во время компиляции.
    // Соответствующая Sync версия RefCell<T> — RwLock<T>.

    use std::{
        cell::RefCell,
        fmt::{Debug, Display},
        rc::Rc,
    };

    use chrono::{Duration, NaiveDate, Utc};
    use rand::{random_range, rng, seq::IndexedRandom};

    #[test]
    fn test_refcell() {
        let value = RefCell::new(1);

        fn add_one(x: &RefCell<i32>) {
            let mut b = x.borrow_mut();

            *b += 1;
        }

        assert_eq!(value, 1.into());
        add_one(&value);
        assert_eq!(value, 2.into());

        let value_2 = RefCell::clone(&value); // Создаётся копия на новый ресурс
        assert_eq!(value, value_2);
        assert_ne!(value.as_ptr(), value_2.as_ptr());

        add_one(&value_2);
        assert_ne!(value, value_2);

        {
            let a = value.borrow(); // Создаётся ссылка на ресурс
            let b = value.borrow(); // Создаётся ссылка на ресурс

            assert_eq!(*a, *b);

            assert!(value.try_borrow_mut().is_err()); //  Уже есть заимствования
        }

        {
            assert_eq!(*value.borrow(), 2);
            let mut a_mut = value.borrow_mut(); // Создается изменяемая ссылка на ресурс
            *a_mut += 1;
            assert!(value.try_borrow().is_err()); // Уже есть мутабельное заимствования
        }
    }

    const MENS_NAMES: [&str; 10] = [
        "Александр",
        "Алексей",
        "Андрей",
        "Валентин",
        "Виктор",
        "Евгений",
        "Иван",
        "Максим",
        "Михаил",
        "Никита",
    ];
    const WOMENS_NAMES: [&str; 10] = [
        "Алина",
        "Анна",
        "Валерия",
        "Дарья",
        "Елена",
        "Елизавета",
        "Ксения",
        "Любовь",
        "Мария",
        "Наталья",
    ];

    type RMPerson<'a> = Rc<RefCell<Person<'a>>>;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Gender {
        Male,
        Female,
    }

    struct MarriedСouple<'a> {
        husband: RMPerson<'a>,
        wife: RMPerson<'a>,
    }

    struct Person<'a> {
        name: &'a str,
        gender: Gender,
        age: NaiveDate,
        parents: Option<Rc<MarriedСouple<'a>>>,
        spouse: Option<RMPerson<'a>>,
        children: Rc<Vec<RMPerson<'a>>>,
    }

    impl Display for Person<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Имя: {}", self.name)?;
            writeln!(f, "Пол: {:?}", self.gender)?;
            writeln!(f, "Возраст: {}", self.age)?;
            writeln!(
                f,
                "Супруг(а): {:?}",
                self.spouse
                    .as_ref()
                    .map(|s| {
                        let s = s.borrow();
                        format!("{}({})", s.name, s.age)
                    })
                    .unwrap_or_else(|| "Нет".to_string())
            )?;
            write!(f, "Дети: ")?;
            if self.children.is_empty() {
                writeln!(f, "Нет")?;
            } else {
                writeln!(f)?;
                for child in self.children.iter() {
                    writeln!(f, "\t{}", child.borrow().name)?;
                }
            }

            let mut last = self.parents.clone();
            if last.is_none() {
                return Ok(());
            }

            writeln!(f, "Родители:")?;
            loop {
                let Some(parents) = last.as_ref() else {
                    break;
                };

                writeln!(f, "\t|")?;
                let father = parents.husband.clone();
                {
                    let s = father.borrow();
                    write!(f, "{}({}) + ", s.name, s.age)?;
                    let s = parents.wife.borrow();
                    writeln!(f, "{}({})", s.name, s.age)?;
                }

                last = father.borrow().parents.clone();
            }

            Ok(())
        }
    }

    fn crate_parents(child: RMPerson) -> Rc<MarriedСouple> {
        // Рождение ребёнка
        let child_date = child.borrow().age;

        // Возраст жены на момент рождения ребёнка. от 18 до 45 лет
        let wife_age = random_range(18 * 365..45 * 365);
        // Муж может быть младше на 2 года но не старше 45 лет
        let husband_age = random_range(wife_age + (18 * 365 - wife_age).max(-2 * 365)..45 * 365);

        // Дата рождения жены
        let wife_birthday = child_date - Duration::days(wife_age);
        let wife = Rc::new(RefCell::new(Person {
            name: WOMENS_NAMES.choose(&mut rng()).unwrap(),
            gender: Gender::Female,
            age: wife_birthday,
            spouse: Default::default(),
            children: Default::default(),
            parents: Default::default(),
        }));
        // Дата рождения мужа
        let husband_birthday = child_date - Duration::days(husband_age);
        let husband = Rc::new(RefCell::new(Person {
            name: MENS_NAMES.choose(&mut rng()).unwrap(),
            gender: Gender::Male,
            age: husband_birthday,
            spouse: Some(wife.clone()),
            children: Default::default(),
            parents: Default::default(),
        }));
        wife.borrow_mut().spouse = Some(husband.clone());

        let married_couple = Rc::new(MarriedСouple {
            husband: husband.clone(),
            wife: wife.clone(),
        });
        child.borrow_mut().parents = Some(married_couple.clone());

        // День свадьбы
        let start = wife_birthday.min(husband_birthday) + Duration::days(18 * 365);
        let max_end = child_date - start;
        let weeding_day = start + Duration::days(random_range(0..max_end.num_days()));

        // Каждый год совершается попытка зачатия ребёнка
        let end_date = Utc::now()
            .date_naive()
            .min(wife_birthday.max(husband_birthday) + Duration::days(50 * 365));

        let mut children = vec![child];
        let mut current_position = weeding_day;

        loop {
            if current_position > end_date {
                break;
            }
            if random_range(0..10) == 0
                && let Some(min) = children
                    .iter()
                    .map(|ch| ch.borrow().age)
                    .map(|bd| (current_position - bd).num_days().abs())
                    .min()
                && min > 300
            {
                current_position += Duration::days(random_range(0..min.min(365)));

                let gender = *[Gender::Male, Gender::Female].choose(&mut rng()).unwrap();
                let new_child = Rc::new(RefCell::new(Person {
                    name: if gender == Gender::Male {
                        MENS_NAMES.choose(&mut rng()).unwrap()
                    } else {
                        WOMENS_NAMES.choose(&mut rng()).unwrap()
                    },
                    gender,
                    age: current_position,
                    parents: Some(married_couple.clone()),
                    spouse: Default::default(),
                    children: Default::default(),
                }));
                children.push(new_child.clone());
            }

            current_position += Duration::days(365);
        }

        children.sort_by(|a, b| a.borrow().age.cmp(&b.borrow().age));
        let children = Rc::new(children);
        married_couple.husband.borrow_mut().children = children.clone();
        married_couple.wife.borrow_mut().children = children.clone();

        married_couple
    }

    #[test]
    fn test_family_tree() {
        let first = Rc::new(RefCell::new(Person {
            name: MENS_NAMES.choose(&mut rng()).unwrap(),
            gender: Gender::Male,
            age: Utc::now().date_naive(),
            spouse: Default::default(),
            children: Default::default(),
            parents: Default::default(),
        }));

        let mut last = first.clone();
        for _ in 0..10 {
            last = crate_parents(last).husband.clone();
        }

        println!("{}", first.borrow());
    }
}
