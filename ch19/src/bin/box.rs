use std::mem::size_of;
use std::thread;

fn main() {
    {
        // 在栈上分配
        let _stack_data = 42;
        // 在堆上分配

        let _heap_data = Box::new(42);
        // 堆上分配大型数据结构
        let _large_array = Box::new([0; 1000000]); // 在堆上分配大数组
    }

    assert_eq!(size_of::<Box<i32>>(), size_of::<usize>()); // 通常是8字节

    {
        let boxed = Box::new(42);
        let raw_ptr = Box::into_raw(boxed);
        let _back_to_box = unsafe { Box::from_raw(raw_ptr) };
    }

    {
        struct MyStruct {
            value: i32,
        }

        impl MyStruct {
            fn get_value(&self) -> i32 {
                self.value
            }
        }
        let my_box = Box::new(MyStruct { value: 42 });

        // 无需显式解引用就能调用方法
        let _val = my_box.get_value();

        // 显式解引用也可以
        let _val = (*my_box).get_value();

        // 可以直接访问字段
        println!("Value: {}", my_box.value);
    }

    {
        fn take_ownership(boxed: Box<i32>) {
            println!("Value: {}", *boxed);
        } // boxed在这里被释放

        let my_box = Box::new(42);
        take_ownership(my_box);
        // println!("{}", *my_box); // 编译错误：my_box已被移动
    }

    {
        fn borrow_box(boxed: &Box<i32>) {
            println!("Borrowed value: {}", **boxed);
        }

        fn mut_borrow_box(boxed: &mut Box<i32>) {
            **boxed += 1;
        }

        fn main() {
            let mut my_box = Box::new(42);
            borrow_box(&my_box);
            mut_borrow_box(&mut my_box);
        }
    }

    {
        // 递归类型
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        // 动态大小类型
        trait Animal {
            fn make_sound(&self);
        }

        struct Dog;
        impl Animal for Dog {
            fn make_sound(&self) {
                println!("Woof!");
            }
        }

        // 可以创建特质对象
        let animal: Box<dyn Animal> = Box::new(Dog);
    }
    {
        let boxed = Box::new(42);

        let handle = thread::spawn(move || {
            println!("Value in new thread: {}", *boxed);
        });

        handle.join().unwrap();
    }
}
