
#[tokio::main]
async fn main() {
    let mut count = 0u8;

    loop {
        tokio::select! {
            // 如果你在没有 `biased;` 的情况下运行这个例子，轮询顺序是伪随机的，
            // 并且对 count 值的断言（可能）会失败。
            biased;

            _ = async {}, if count < 1 => {
                count += 1;
                assert_eq!(count, 1);
            }
            _ = async {}, if count < 2 => {
                count += 1;
                assert_eq!(count, 2);
            }
            _ = async {}, if count < 3 => {
                count += 1;
                assert_eq!(count, 3);
            }
            _ = async {}, if count < 4 => {
                count += 1;
                assert_eq!(count, 4);
            }

            else => {
                break;
            }
        };
    }
}