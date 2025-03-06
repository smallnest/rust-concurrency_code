#![feature(once_cell_get_mut)]
#![feature(once_cell_try)]

use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());

    {
        let cell = OnceCell::new();
        assert!(cell.get().is_none());

        assert_eq!(cell.set(92), Ok(()));
        assert_eq!(cell.set(62), Err(62));

        assert!(cell.get().is_some());
    }

    {
        let mut cell: OnceCell<String> = OnceCell::new();
        assert_eq!(cell.take(), None);

        let mut cell = OnceCell::new();
        let _ = cell.set("hello".to_owned());
        assert_eq!(cell.take(), Some("hello".to_owned()));
        assert_eq!(cell.get(), None);
    }

    {
        let cell: OnceCell<String> = OnceCell::new();
        assert_eq!(cell.into_inner(), None);

        let cell = OnceCell::new();
        let _ = cell.set("hello".to_owned());
        assert_eq!(cell.into_inner(), Some("hello".to_owned()));
    }

    {
        let cell = OnceCell::new();
        let value = cell.get_or_init(|| 92);
        assert_eq!(value, &92);
        let value = cell.get_or_init(|| unreachable!());
        assert_eq!(value, &92);
    }

    {
        let mut cell = OnceCell::new();
        let value = cell.get_mut_or_init(|| 92);
        assert_eq!(*value, 92);

        *value += 2;
        assert_eq!(*value, 94);

        let value = cell.get_mut_or_init(|| unreachable!());
        assert_eq!(*value, 94);
    }

    {
        let cell = OnceCell::new();
        assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
        assert!(cell.get().is_none());
        let value = cell.get_or_try_init(|| -> Result<i32, ()> { Ok(92) });
        assert_eq!(value, Ok(&92));
        assert_eq!(cell.get(), Some(&92))
    }

    {
        let mut cell: OnceCell<u32> = OnceCell::new();

        // Failed initializers do not change the value
        assert!(
            cell.get_mut_or_try_init(|| "not a number!".parse())
                .is_err()
        );
        assert!(cell.get().is_none());

        let value = cell.get_mut_or_try_init(|| "1234".parse());
        assert_eq!(value, Ok(&mut 1234));

        let Ok(value) = value else {
            return;
        };
        *value += 2;
        assert_eq!(cell.get(), Some(&1236))
    }
}
