use async_std::process::{Command, Stdio};

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let output = Command::new("echo")
        .arg("hello")
        .output()
        .await?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[async_std::test]
async fn test_multi_args() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .stdout(Stdio::piped())
        .output()
        .await?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

#[async_std::test]
async fn test_bash() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("bash") // 或 Command::new("sh")
        .arg("-c")
        .arg("ls -l | grep txt") // 这里是你的 shell 命令
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // 捕获错误输出
        .output()
        .await?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}