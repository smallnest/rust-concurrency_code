use scopeguard::defer;


fn main() {
    defer! {
        println!("scopeguard: Called at return or panic");
    }
    println!("scopeguard: Called first before panic");
    panic!();
    println!("scopeguard: Called first after panic");
}