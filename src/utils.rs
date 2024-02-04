
use crossterm::{execute, terminal::LeaveAlternateScreen};

/// Helps to gracefully exit terminal screen on panic.
pub fn init_panic_handler() {
       let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _try_leave = execute!(std::io::stderr(), LeaveAlternateScreen);
        if crossterm::terminal::is_raw_mode_enabled().expect("Gets terminal raw status") {
            crossterm::terminal::disable_raw_mode().expect("Disable raw mode in panic")
        }
        panic_hook(panic_info);
    }));

    // TODO Color-eyre and human-panic hooks
}
