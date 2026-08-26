#![forbid(unsafe_code)]

use declmig_desktop_core::{app::DesktopApp, config::DesktopConfig};

fn main() {
    let cfg = DesktopConfig::from_env();
    DesktopApp::new(cfg).run();
}

