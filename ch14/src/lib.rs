// 实现一个函数，计算一个随机数的哈希值，看看它是否以n个0开启，就像比特币挖矿一样。

use sha2::{Digest, Sha256};
use rand::Rng; 

pub fn mine_zeros(n: usize) -> (u64, [u8; 32]) {
    let mut rng = rand::thread_rng();
    
    let mut nonce = rng.r#gen::<u64>();
    loop {
        nonce = nonce.wrapping_add(1);
        let hash = Sha256::digest(&nonce.to_be_bytes());

        if hash.iter().take(n).all(|&x| x == 0) {
            return (nonce, hash.into()); // 正确返回 (nonce, hash)
        }
        
    }
}

// 测试代码
#[test]
fn test_mine_zeros() {
    let n = 2;
    let (nonce,hash) = mine_zeros(n);
    println!("随机数：{}, {:?}", nonce, hash[0..n].to_vec());
}