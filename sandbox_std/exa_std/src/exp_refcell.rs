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

struct MarriedCouple<'a> {
    husband: RMPerson<'a>,
    wife: RMPerson<'a>,
}

struct Person<'a> {
    name: &'a str,
    gender: Gender,
    age: NaiveDate,
    parents: Option<Rc<MarriedCouple<'a>>>,
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

fn crate_parents(child: RMPerson) -> Rc<MarriedCouple> {
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

    let married_couple = Rc::new(MarriedCouple {
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

#[test]
fn test_metdhods() {
    let a = RefCell::new(1);

    // let b = a.into_inner();
    // assert_eq!(1, *a.borrow()); // !panic
    assert_eq!(1, *a.borrow());
    let old = a.replace(2);
    assert_eq!(1, old);
    assert_eq!(2, *a.borrow());
    {
        let _b = a.borrow();
        // a.replace(3); // !panic
    }

    let old = a.replace_with(|v| *v + 1);
    assert_eq!(2, old);
    assert_eq!(3, *a.borrow());

    let b = RefCell::new(4);
    a.swap(&b);
    assert_eq!(3, *b.borrow());
    assert_eq!(4, *a.borrow());
    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;
    assert_eq!(4, *b.borrow());
    assert_eq!(5, *a.borrow());

    {
        let mut r = a.borrow_mut();
        *r += 1;
        let _b = RefCell::new(1);
        // a.swap(&_b); // panic

        assert!(a.try_borrow().is_err());
        assert!(a.try_borrow_mut().is_err());
    }
}
