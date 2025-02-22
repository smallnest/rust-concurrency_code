use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut interrupt_signal = signal(SignalKind::interrupt())?;
    let mut terminate_signal = signal(SignalKind::terminate())?;
    let mut hangup_signal = signal(SignalKind::hangup())?;

    tokio::select! {
        _ = signal::ctrl_c() => {},
        _ = interrupt_signal.recv() => {},
        _ = terminate_signal.recv() => {},
        _ = hangup_signal.recv() => {},
    }

    println!("exit");
    
    Ok(())
}