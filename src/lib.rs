use std::fs::File;

use anyhow::{Result};

mod daemonize;
use daemonize::{daemonize};

use simplelog::{LevelFilter, Config, TerminalMode, ColorChoice};

const LOG_DIR: &str = "./test.log";

pub struct Daemon {
    f: fn() -> Result<()>,
    debug: bool,
    log_dir: std::path::PathBuf,
}

impl Daemon {
    pub fn new(f: fn() -> anyhow::Result<()>) -> Self {
        Daemon {f, debug: false, log_dir: std::path::PathBuf::from(LOG_DIR)}
    }
    
    pub fn run(&self) -> Result<()> {
        if self.debug {
            let _ = simplelog::TermLogger::init(
                LevelFilter::Info, 
                Config::default(), 
                TerminalMode::Mixed, 
                ColorChoice::Auto
                )?;
        } else {
            let _ = simplelog::WriteLogger::init(
                LevelFilter::Info,
                Config::default(),
                File::create(&self.log_dir)?
            )?;
        }

        daemonize(self.f, self.debug)
            .map_err(|e| log::error!("failed with: {}", e))
            .expect("Fetal error happened.");
        Ok(())
    }

    pub fn set_debug_mode(&mut self, b: bool) {
        self.debug = b;
    }

    pub fn set_log_dir(&mut self, log_dir: &str) {
        self.log_dir = std::path::PathBuf::from(log_dir);
    }
}
