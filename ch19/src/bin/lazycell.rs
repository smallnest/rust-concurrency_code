#![feature(lazy_get)]
use std::cell::LazyCell;

fn main() {
    let lazy: LazyCell<i32> = LazyCell::new(|| {
        println!("initializing");
        47
    });
    println!("ready");
    println!("{}", *lazy);
    println!("{}", *lazy);

    // Prints:
    //   ready
    //   initializing
    //   92
    //   92

    {
        let lazy = LazyCell::new(|| 47);

        assert_eq!(LazyCell::force(&lazy), &47);
        assert_eq!(&*lazy, &47);
    }

    {
        let mut lazy = LazyCell::new(|| 92);

        let p = LazyCell::force_mut(&mut lazy);
        assert_eq!(*p, 92);
        *p = 47;
        assert_eq!(*lazy, 47);
    }

    {
        use std::cell::LazyCell;

        let lazy = LazyCell::new(|| 47);

        assert_eq!(LazyCell::get(&lazy), None);
        let _ = LazyCell::force(&lazy);
        assert_eq!(LazyCell::get(&lazy), Some(&47));
    }
}
