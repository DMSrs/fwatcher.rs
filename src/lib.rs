//! Auto run command when some files changed.
//!
//! `fwatcher` also has a [cli command](./cli/index.html).
//!
//! # Usage
//!
//! Dependencies in your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! glob = "0.2"
//! notify = "4.0"
//! fwatcher = "*"
//! ```
//!
//! # Example
//!
//! ```rust
//! extern crate glob;
//! extern crate fwatcher;
//!
//! use fwatcher::Fwatcher;
//! use glob::Pattern;
//! use std::path::PathBuf;
//! use std::time::Duration;
//!
//! fn main() {
//!     let dirs =vec![PathBuf::from("src")];
//!     let cmd = vec!["pytest".to_string()];
//!
//!     let mut fwatcher = Fwatcher::new(dirs, cmd);
//!     fwatcher.pattern(Pattern::new("**/*.py").unwrap())
//!             .exclude_pattern(Pattern::new("**/.git/**").unwrap())
//!             .interval(Duration::new(1, 0))
//!             .restart(false)
//!             .run();
//! }
//! ```

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://docs.rs/fwatcher")]

extern crate getopts;
extern crate glob;
extern crate notify;

use glob::Pattern;
use notify::{DebouncedEvent, RecursiveMode, Watcher, watcher};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub mod cli;

pub trait WatchingMode {
    fn run();
    fn restart_child(&mut self);
}

/// a struct save `Fwatcher` state
pub struct Fwatcher<T: WatchingMode> {
    dirs: Vec<PathBuf>,
    patterns: Vec<Pattern>,
    exclude_patterns: Vec<Pattern>,
    delay: Duration,
    interval: Duration,
    restart: bool,
    cmd: T,
    last_run: Option<Instant>,
    child: Option<Child>,
}

impl<T: WatchingMode> Fwatcher<T> {
    /// Constructs a new `Fwatcher`
    pub fn new(dirs: Vec<PathBuf>, cmd: T) -> Self {
        Fwatcher {
            dirs,
            patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            delay: Duration::new(2, 0),
            interval: Duration::new(1, 0),
            restart: false,
            cmd,
            last_run: None,
            child: None,
        }
    }

    /// add a watcher pattern
    pub fn pattern(&mut self, pat: Pattern) -> &mut Self {
        self.patterns.push(pat);
        self
    }

    /// add watcher patterns
    pub fn patterns(&mut self, pats: &[Pattern]) -> &mut Self {
        self.patterns.extend_from_slice(pats);
        self
    }

    /// add a watcher exclusive pattern
    pub fn exclude_pattern(&mut self, pat: Pattern) -> &mut Self {
        self.exclude_patterns.push(pat);
        self
    }

    /// add watcher exclusive patterns
    pub fn exclude_patterns(&mut self, pats: &[Pattern]) -> &mut Self {
        self.exclude_patterns.extend_from_slice(pats);
        self
    }

    /// set watcher interval seconds
    pub fn delay(&mut self, d: Duration) -> &mut Self {
        self.delay = d;
        self
    }

    /// set watcher interval seconds
    pub fn interval(&mut self, d: Duration) -> &mut Self {
        self.interval = d;
        self
    }

    /// set watcher restart flag
    pub fn restart(&mut self, flag: bool) -> &mut Self {
        self.restart = flag;
        self
    }

    /// run `Fwatcher`
    pub fn run(&mut self) {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, self.delay).expect("can not create a watcher");

        if self.dirs.is_empty() {
            watcher.watch(::std::env::current_dir().expect("get current dir error"),
                          RecursiveMode::Recursive)
                   .expect("can not watch dir");
        } else {
            for d in self.dirs.iter() {
                watcher.watch(d, RecursiveMode::Recursive)
                       .expect("can not watch dir");
            }
        }
        WatchingMode::restart_child(self);

        loop {
            match rx.recv() {
                Ok(event) => {
                    if self.interval_consumed() {
                        self.process_event(event)
                    }
                },
                Err(why) => println!("watch error: {:?}", why),
            }
        }
    }

    fn interval_consumed(&mut self) -> bool {
        let now = Instant::now();

        if let Some(last_run) = self.last_run {
            if now.duration_since(last_run) < self.interval {
                return false;
            }
        }

        return true;
    }

    fn process_event(&mut self, event: DebouncedEvent) {
        match event {
            DebouncedEvent::Create(ref fpath) |
            DebouncedEvent::Write(ref fpath)  |
            DebouncedEvent::Remove(ref fpath) |
            DebouncedEvent::Rename(ref fpath, _) => {
                if self.patterns
                       .iter()
                       .any(|ref pat| pat.matches_path(fpath)) &&
                   self.exclude_patterns
                       .iter()
                       .all(|ref pat| !pat.matches_path(fpath)) {
                    println!("Modified: {:?}", fpath);
                    WatchingMode::restart_child(self);
                }
            },
            _ => {},
        }
    }
}

/*impl Fwatcher<Vec<String>> {
    fn restart_child(&mut self) {
        if let Some(ref mut child) = self.child {
            if self.restart {
                let _ = child.kill();
            }
        }
        self.child = Command::new(&self.cmd[0])
            .args(&self.cmd[1..])
            .spawn()
            .ok();
        self.last_run = Some(Instant::now());
    }
}*/

impl WatchingMode for Vec<String> {
    fn run() {

    }

    fn restart_child(&mut self) {
    }
}

impl WatchingMode for Box<Fn(usize)> {
    fn run() {
    }

    fn restart_child(&mut self) {
    }
}

impl Fwatcher<Box<Fn(usize)>> {
    fn restart_child(&mut self) {

    }
}

impl Fwatcher<Vec<String>> {
    fn restart_child(&mut self) {

    }
}