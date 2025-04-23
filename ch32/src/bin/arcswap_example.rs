use std::sync::Arc;

use arc_swap::ArcSwap;

fn main() {
    let config = ArcSwap::from(Arc::new(String::default()));
    std::thread::scope(|s| {
        s.spawn(|| {
            let new_conf = Arc::new("新配置".to_owned());
            config.store(new_conf);
        });
        for _ in 0..10 {
            s.spawn(|| {
                loop {
                    let cfg = config.load();
                    if !cfg.is_empty() {
                        assert_eq!(**cfg, "新配置");
                        return;
                    }
                }
            });
        }
    });

    println!("config: {}", config.load());
}
