#![feature(core)]
#![feature(std_misc)]
#![feature(libc)]
#![feature(alloc)]
#![feature(env)]
#![feature(os)]
#![feature(old_io)]

extern crate ncurses;
extern crate hex2d;
extern crate "hex2d-dpcext" as hex2dext;
extern crate libc;
extern crate rand;
extern crate time;
#[macro_use]
extern crate log;
extern crate core;

use std::sync::mpsc;
use std::thread;

mod actor;
mod ai;
mod error;
mod game;
mod generate;
mod item;
mod ui;
mod util;

pub fn main() {
    println!("Generating map...");
    let state = game::State::new();

    let mut controller = game::Controller::new(state);

    let (pl_req_tx, pl_req_rx) = mpsc::channel();
    let (pl_rep_tx, pl_rep_rx) = mpsc::channel();

    let (ai_req_tx, ai_req_rx) = mpsc::channel();
    let (ai_rep_tx, ai_rep_rx) = mpsc::channel();

    println!("Starting game...");
    thread::spawn(move || {
        let _ = controller.run(
            pl_req_tx, pl_rep_rx,
            ai_req_tx, ai_rep_rx
            );
    });

    println!("Starting AI...");
    thread::spawn(move|| {
        let _ = ai::run(ai_req_rx, ai_rep_tx);
    });

    println!("Starting UI...");
    let mut ui = ui::curses::CursesUI::new();
    ui.display_intro();
    let mut ui = ui::Ui::new(ui);

    ui.run(pl_req_rx, pl_rep_tx);
}
