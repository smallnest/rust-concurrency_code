use std::rc::Rc;

fn main() {
    // 创建一个包含5的Rc智能指针
    let a = Rc::new(5);    
    // 克隆Rc，增加引用计数
    let b = Rc::clone(&a);
    let _weak = Rc::downgrade(&b);
    println!("引用计数: {}, 弱引用计数: {}", Rc::strong_count(&a), Rc::weak_count(&a)); // 输出：引用计数: 2, 1

    let mut unique = Rc::new(String::from("hello"));
    if let Some(s) = Rc::get_mut(&mut unique) {
        s.push_str(" world");
    }

    uninit_example();
    downgrade_example();
    decrement_example();
    get_mut_example();
}
fn uninit_example() {
    let mut five = Rc::<u32>::new_uninit();
    // 延迟初始化
    Rc::get_mut(&mut five).unwrap().write(5);
    let five = unsafe { five.assume_init() };
    assert_eq!(*five, 5)
}

fn downgrade_example() {
    use std::any::Any;
    use std::rc::Rc;

    fn print_if_string(value: Rc<dyn Any>) {
        if let Ok(string) = value.downcast::<String>() {
            println!("String ({}): {}", string.len(), string);
        }
    }

    let my_string = "Hello World".to_string();
    print_if_string(Rc::new(my_string));
    print_if_string(Rc::new(0i8));
}

fn decrement_example() {
    use std::rc::Rc;

    let five = Rc::new(5);

    unsafe {
        let ptr = Rc::into_raw(five);
        Rc::increment_strong_count(ptr);

        let five = Rc::from_raw(ptr);
        assert_eq!(2, Rc::strong_count(&five));
        Rc::decrement_strong_count(ptr);
        assert_eq!(1, Rc::strong_count(&five));
    }
}

fn get_mut_example() {
    use std::rc::Rc;

    let mut five = Rc::new(5);
    let _five2 = Rc::clone(&five);
    if let Some(value) = Rc::get_mut(&mut five) {
        *value = 10;
    }
    assert_eq!(*five, 5);

    let mut data = Rc::new(5);

    *Rc::make_mut(&mut data) += 1;         // 不会克隆, 6

    let mut other_data = Rc::clone(&data); // 不会克隆底层数据, 6

    *Rc::make_mut(&mut data) += 1;         // 克隆了底层数据,此时data为7，other_data为6
    
    *Rc::make_mut(&mut data) += 1;         // 不会克隆，8
    *Rc::make_mut(&mut other_data) *= 2;   // 不会克隆，12

    // data 和 other_data 是不同的数据
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);
}