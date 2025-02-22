
fn main() {
    let task = smol::spawn(async {
        1 + 2
    });
    
    smol::block_on(async {
        assert_eq!(task.await, 3);
    });

}