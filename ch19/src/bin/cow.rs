#![feature(cow_is_borrowed)]

use std::borrow::Cow;
use std::mem::size_of;

fn main() {
    create_example();
    cow_example();

    const WORD: usize = size_of::<usize>();

    assert_eq!(size_of::<std::borrow::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);

    let borrowed: beef::lean::Cow<str> = beef::lean::Cow::borrowed("Hello");
    let owned: beef::lean::Cow<str> = beef::lean::Cow::owned(String::from("World"));
    assert_eq!(format!("{} {}!", borrowed, owned), "Hello World!",);
}

fn create_example() {
    // 从借用创建
    let borrowed: Cow<str> = Cow::Borrowed("hello");
    println!(
        "is_borrowed: {}, is_owned: {}",
        borrowed.is_borrowed(),
        borrowed.is_owned()
    ); // true,false

    // 从拥有的数据创建
    let owned: Cow<str> = Cow::Owned(String::from("hello"));
    println!(
        "is_borrowed: {}, is_owned: {}",
        owned.is_borrowed(),
        owned.is_owned()
    ); // false,true

    // 从引用创建（自动选择）
    let text = "hello";
    let cow: Cow<str> = text.into();
    println!(
        "is_borrowed: {}, is_owned: {}",
        cow.is_borrowed(),
        cow.is_owned()
    ); // true,false
}

fn cow_example() {
    let s = "Hello world!";
    let cow = Cow::Borrowed(s);
    assert_eq!(cow.into_owned(), String::from(s));

    let s = "Hello world!";
    let cow: Cow<'_, str> = Cow::Owned(String::from(s));
    assert_eq!(cow.into_owned(), String::from(s));

    let mut cow = Cow::Borrowed("foo");
    cow.to_mut().make_ascii_uppercase();
    assert_eq!(cow, Cow::Owned(String::from("FOO")) as Cow<'_, str>);
    assert_eq!(true, cow.is_owned());
}
