use anyhow::{Context, Result};
use nix::sys::stat::{umask, Mode};
use nix::unistd::{chdir, close, dup, dup2, getpid, fork, setsid, ForkResult};
use std::process::exit;

use log::{info};

pub fn daemonize (
    process: fn() -> Result<()>,
    debug: bool,
) -> Result<()> {

    // if debug {
    //     let _ = simplelog::TermLogger::init(
    //         LevelFilter::Info, 
    //         Config::default(), 
    //         TerminalMode::Mixed, 
    //         ColorChoice::Auto
    //     ).unwrap();
    // }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            info!("forked as parent PID: {} and exiting", getpid());
            info!("child process is PID: {}", child);
            exit(0);
        }
        Ok(ForkResult::Child) => {
            setsid()?;
            info!("setsid sucsessed");
            
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child }) => {
                    info!("forked as parent PID: {} and exiting", getpid());
                    info!("grandchild process is PID: {}", child);
                    exit(0);
                }
                Ok(ForkResult::Child) => {
                    info!("Initilizing deamon process: PID {}", getpid());

                    chdir("/")?;
                    info!("change directory into root");

                    let mode = Mode::from_bits(0o002).context("Couldn't generate 'mode'")?;
                    umask(mode);
                    info!("unmasked with 002 (664)");
                    
                    let old_stdin = dup(0)?;
                    info!("dup fd(0): old_stdin = {}", old_stdin);
                    let old_stdout = dup(1)?;
                    info!("dup fd(1): old_stdout = {}", old_stdout);
                    let old_stderr = dup(2)?;
                    info!("dup fd(2): old_stderr = {}", old_stderr);
                    
                    close(0)?;
                    info!("close standard input");

                    close(1)?;
                    info!("close standard output");
                    
                    close(2)?;
                    info!("close standard error");

                    if debug {
                        dup2(old_stdin, 0).unwrap();
                        dup2(old_stdout, 1).unwrap();
                        dup2(old_stderr, 2).unwrap();
                        info!("debug mode = true: reopened stdin/stdout/stderr");
                        info!("dup2 {} to fd(0)", old_stdin);
                        info!("dup2 {} to fd(1)", old_stdout);
                        info!("dup2 {} to fd(2)", old_stderr);
                    }

                    process()?;

                    info!("PID: {} DONE", getpid());
                    exit(0);
                }
                Err(_) => panic!("fork() failed"),
            }
        }
        Err(_) => panic!("fork() failed"),
    };
}
