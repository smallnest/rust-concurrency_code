use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 创建根节点
    let root = Rc::new(Node {
        value: 0,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    // 创建子节点，使用弱引用指向父节点
    let child = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Rc::downgrade(&root)),
        children: RefCell::new(vec![]),
    });
    
    // 将子节点添加到父节点
    root.children.borrow_mut().push(Rc::clone(&child));
    
    // 打印结构
    println!("根节点的子节点数量: {}", root.children.borrow().len());
    
    // 通过子节点访问父节点
    if let Some(parent) = child.parent.borrow().upgrade() {
        println!("子节点成功访问父节点，父节点值: {}", parent.value);
    }
    
    // 引用计数
    println!("根节点强引用计数: {}", Rc::strong_count(&root)); // 2 (变量和子节点列表)
    println!("根节点弱引用计数: {}", Rc::weak_count(&root)); // 1 (来自子节点)
    
    println!("子节点强引用计数: {}", Rc::strong_count(&child)); // 2 (变量和父节点的子节点列表)
}