#![feature(cell_update)]

use std::cell::Cell;

struct Counter {
    value: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { value: Cell::new(0) }
    }
    
    fn increment(&self) {
        let current = self.value.get();
        self.value.set(current + 1);
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    println!("counter.value: {}", counter.value.get());

    get_example();
    set_example();

    update_example();
    replace_example();
    swap_example();
    take_example();
    into_inner_example();

    get_mut_example();
    from_mut_example();

    size_of_example();
}

fn get_example() {
    let my_cell = Cell::new(42); // 整数类型 i32 实现了 Copy

    let value = my_cell.get(); // 获取内部值的副本
    println!("Value: {}", value); // 输出：Value: 42

    my_cell.set(99); // 设置新的内部值
    let new_value = my_cell.get();
    println!("New Value: {}", new_value); // 输出 New Value: 99

    // let my_cell = Cell::new(String::from("hello")); // String 没有实现 Copy

    // let value = my_cell.get(); // 编译错误！
    // println!("Value: {}", value);
}

fn set_example() {
    let my_cell = Cell::new(42);

    my_cell.set(99); // 设置新的内部值
    let new_value = my_cell.get();
    println!("New Value: {}", new_value); // 输出 New Value: 99
}

fn update_example() {
    let my_cell = Cell::new(42);

    my_cell.update(|x| x + 1); // 更新内部值
    let new_value = my_cell.get();
    println!("New Value: {}", new_value); // 输出 New Value: 43
}

fn replace_example() {
    let my_cell = Cell::new(42);

    let old_value = my_cell.replace(99); // 替换内部值并返回旧值
    println!("Old Value: {}", old_value); // 输出 Old Value: 42

    let new_value = my_cell.get();
    println!("New Value: {}", new_value); // 输出 New Value: 99
}

fn swap_example() {
    let c1 = Cell::new(5);
    let c2 = Cell::new(10);
    Cell::swap(&c1, &c2); // c1现在包含10，c2包含5

    println!("c1: {}, c2: {}", c1.get(), c2.get());
}

fn take_example() {
    let c = Cell::new(5);
    let five = c.take();

    assert_eq!(five, 5);
    assert_eq!(c.into_inner(), 0);
}

fn into_inner_example() {
    let c = Cell::new(5);
    let five = c.into_inner();

    assert_eq!(five, 5);
    // assert_eq!(c.into_inner(), 0); // 编译错误！
}

fn get_mut_example() {
    let mut c = Cell::new(5);
    *c.get_mut() += 1;
    
    assert_eq!(c.get(), 6);
}

fn from_mut_example() {
    let mut x = 5;
    let c = Cell::from_mut(&mut x);
    c.set(10);
    
    assert_eq!(x, 10);
}


fn size_of_example() {
    use std::mem::size_of;

    println!("size of i32: {}", size_of::<i32>());  // 通常是4字节
    println!("size of Cell<i32>: {}", size_of::<Cell<i32>>()); // 也是4字节
}