use sync_engine::FilesystemWatcher;
use std::path::PathBuf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    println!("Watching: {}", path);
    println!("Press Ctrl+C to stop...");

    let watcher = FilesystemWatcher::new(PathBuf::from(&path));
    let rx = watcher.watch()?;

    loop {
        if let Ok(event) = rx.recv() {
            println!("Event: {:?}", event);
        }
    }
}
