use std::cell::RefCell;

struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            messages: RefCell::new(Vec::new()),
        }
    }

    fn log(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }
}

fn main() {
    let logger = Logger::new();
    logger.log("Hello");
    logger.log("World");

    let messages = logger.get_messages();
    for message in messages {
        println!("{}", message);
    }

    {
        let ref_cell = RefCell::new(5);
        let borrowed = ref_cell.borrow(); // 返回Ref<T>
        println!("Value: {}", *borrowed);
    }

    {
        let ref_cell = RefCell::new(5);
        let mut borrowed_mut = ref_cell.borrow_mut(); // 返回RefMut<T>
        *borrowed_mut += 1;
        println!("New value: {}", *borrowed_mut);
    }

    {
        let data = RefCell::new(vec![1, 2, 3]);

        // 获取RefMut<Vec<i32>>
        let mut data_ref_mut = data.borrow_mut();

        // 使用Deref特性访问内部值
        println!("Vector length: {}", data_ref_mut.len()); // 等同于 (*data_ref_mut).len()

        // 使用DerefMut特性修改内部值
        data_ref_mut.push(4); // 等同于 (*data_ref_mut).push(4)

        // 现在vector是[1, 2, 3, 4]
        println!("Vector: {:?}", *data_ref_mut);
    }

    {
        let ref_cell = RefCell::new(5);
        let borrow_result = ref_cell.try_borrow(); // 返回Result<Ref<T>, BorrowError>
        if let Ok(borrowed) = borrow_result {
            println!("Successfully borrowed: {}", *borrowed);
        } else {
            println!("Failed to borrow");
        }
    }

    {
        let ref_cell = RefCell::new(5);
        let borrow_mut_result = ref_cell.try_borrow_mut(); // 返回Result<RefMut<T>, BorrowMutError>
        if let Ok(mut borrowed) = borrow_mut_result {
            *borrowed += 1;
            println!("Successfully mutably borrowed: {}", *borrowed);
        } else {
            println!("Failed to mutably borrow");
        }
    }

    {
        let ref_cell = RefCell::new(5);
        let value = unsafe { &mut *ref_cell.as_ptr() };
        *value += 1;
        println!("Value: {}", ref_cell.borrow());
    }

    {
        let ref_cell = RefCell::new(5);
        let value = ref_cell.into_inner(); // 返回5  
        println!("Value: {}", value);
    }

    {
        let ref_cell = RefCell::new(5);
        let _borrow = ref_cell.borrow_mut();
        println!("Borrow state: {:?}", ref_cell.try_borrow()); // Prints: Err(BorrowError)
    }

    {
        let cell = RefCell::new(5);
        let old_value = cell.replace_with(|&mut old| old + 1);
        assert_eq!(old_value, 5);
        assert_eq!(cell, RefCell::new(6));
    }
}
