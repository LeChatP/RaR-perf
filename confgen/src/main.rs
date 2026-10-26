use clap::Parser;
use nix::unistd::Uid;

use crate::util::cap_clear;

mod doas;
mod please;
mod rar;
mod run0;
mod sudo;
pub mod util;

fn configure_tools(args: &Args) {
    doas::configure(args);
    please::configure(args);
    run0::configure(args);
    sudo::configure(args);
    rar::configure(args);
}

#[derive(Parser)]
pub struct Args {
    nb_task: u32,
    #[clap(short, long, default_value_t = env!("CONFGEN_UID").parse::<u32>().unwrap_or(Uid::current().as_raw()))]
    user_id: u32,
    #[clap(short, long, default_value = env!("SUDO_CFG_PATH"))]
    sudo_file: String,
    #[clap(short, long, default_value = env!("RAR_CFG_PATH"))]
    rar_file: String,
    #[clap(short, long, default_value = env!("PLEASE_CFG_PATH"))]
    please_file: String,
    // no short option: -r is the one of --rar-file
    #[clap(long, default_value = env!("RUN0_CFG_PATH"))]
    run0_file: String,
    #[clap(short, long, default_value = env!("DOAS_CFG_PATH"))]
    doas_file: String,
}

fn main() {
    // Set the effective capabilities to none
    cap_clear().unwrap();

    let args = Args::parse();

    // Configure the rootasrole and sudo roles for performance testing
    configure_tools(&args);
}
