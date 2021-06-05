use box_shorthand::box_shorthand;

/// here we're simply testing that a bunch of generics and stuff work properly
#[test]
fn test_complicated() {
    #[derive(box_shorthand, PartialEq, Debug)]
    enum Hello<'a, I, T>
    where
        T: PartialEq + std::fmt::Debug,
        I: IntoIterator<Item = T> + PartialEq + std::fmt::Debug,
    {
        StringRef(&'a str),
        Vector(Vec<T>),
        It(I, I),
    }

    let a = HelloB::Vector::<'_, Vec<usize>, _>(vec![1, 2, 3]);
    let b = HelloB::It::<'_, Vec<usize>, _>(vec![1, 2, 3], vec![1, 2, 3]);
    let c = HelloB::StringRef::<Vec<usize>, _>("Hello");

    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(c, a);
}

/// test that use syntax works properly on the generated module
#[test]
fn test_use_generated_module() {
    #[derive(box_shorthand)]
    enum E { A, B }

    // Using the generated module shortens shorthand even more
    use EB::*;

    match (*A(), *B()) {
        (E::A, E::B) => {}
        _ => assert!(false),
    }
}

/// test that struct variants generate the appropriate function
#[test]
fn test_struct_variant() {
    #[derive(box_shorthand)]
    enum E {
        A {
            x: i64,
            y: Vec<usize>,
        }
    }

    // Note that struct variant shorthand uses function call syntax instead of struct initialization syntax because it is a function call, rather than a true struct variant constructor
    let value: Box<E> = EB::A(1, vec![2, 3]);

    // We have to deref the box to match on it
    match *value {
        E::A { x, y } => {
            assert_eq!(x, 1);
            assert_eq!(y, vec![2, 3]);
        }
    }
}

/// test that unit variants generate the appropriate function
#[test]
fn test_unit_variant() {
    #[derive(box_shorthand)]
    enum E { A }

    // Note that unit variant shorthand still requires parens because it is a function call, rather than a true unit variant constructor
    let value: Box<E> = EB::A();

    // We have to deref the box to match on it
    match *value {
        E::A => {}
    }
}
