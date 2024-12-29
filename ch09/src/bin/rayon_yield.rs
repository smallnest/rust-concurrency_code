fn main() {
    rayon::scope(|s| {
        s.spawn(|_| {
            // 某些计算
            rayon::yield_local();
            // 继续执行其他计算
        });
    });

    rayon::scope(|s| {
        s.spawn(|_| {
            // 某些计算
            rayon::yield_now();
            // 继续执行其他计算
        });
    });
}