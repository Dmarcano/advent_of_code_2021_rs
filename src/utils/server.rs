//! This module contains a server that at Midnight Eastern time it will query the AOC website for the current day's
//! Puzzle.
//!
//!
//!
//!
use crossbeam_channel::{unbounded, Receiver, Sender};

/// An Synchronous HTTP Server which will query the Advent of Code website for the current day's Puzzle.
struct SyncServer {
    input_channel: Sender<String>,
}
