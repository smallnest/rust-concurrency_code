use std::io::Read;
use std::fs::File;

async fn read_file(path: &str) -> std::io::Result<String> {
    let path = path.to_string();
    smol::unblock(move || {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    })
    .await
}

fn main() {
    smol::block_on(async {
        match read_file("Cargo.toml").await {
            Ok(contents) => println!("文件内容：{}", contents),
            Err(err) => eprintln!("读取文件出错：{}", err),
        }
    });
}
