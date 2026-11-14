#![forbid(unsafe_code)]

use declmig_desktop_core::{app::DesktopApp, config::DesktopConfig};

fn main() {
    // Shared lifecycle logging stays local and leaves stdout available for IPC.
    let _desktop_session = next_loggers::desktop::DesktopSession::start(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .ok();

    let cfg = DesktopConfig::try_from_env().unwrap_or_else(|message| {
        eprintln!("desktop configuration rejected: {message}");
        std::process::exit(2);
    });
    DesktopApp::new(cfg).run();
}

