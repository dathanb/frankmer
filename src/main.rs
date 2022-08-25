use std::error::Error;

mod timechunk;
mod timer;
mod timer_cli;
mod timetext;
#[cfg(feature="mac-notification")]
mod notifier;

fn main() -> Result<(), Box<dyn Error>> {
    let mut time_ui = timer_cli::TimeCLI::new();
    time_ui.run_interface()?;
    Ok(())
}
