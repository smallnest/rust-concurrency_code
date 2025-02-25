
#[tokio::main]
async fn main() {
    let result = tokio_scoped::scope(|scope| {
        scope.spawn(async {
            println!("Thread 1 is running");
        });

        scope.spawn(async {
            println!("Thread 2 is running");
        });

        42
    });
    
    println!("Scope completed successfully with value: {}", result)
}