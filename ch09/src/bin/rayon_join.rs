fn main() {
    let result = rayon::join(
        || println!("task 1"), 
        || {println!("task 2"); 2}
    );

    println!("Result: {:?}", result);    
}