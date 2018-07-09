extern crate fwatcher;
extern crate glob;
extern crate notify;

use fwatcher::Fwatcher;

use std::path::PathBuf;
use glob::Pattern;
use std::time::Duration;
use notify::DebouncedEvent;

#[test]
fn test(){
    let mut dir = vec![String::from("./")].iter().map(|dir| PathBuf::from(dir)).collect();
    let mut fw = Fwatcher::<Box<Fn(&DebouncedEvent)>>::new(dir, Box::new(|x| {
        println!("Modified: {:?}", x);
    }));

    let patterns = vec![Pattern::new("*").unwrap()];
    let exclude_patterns = vec![];
    let delay = Duration::new(1, 0);
    let interval = Duration::new(1, 0);
    let restart = true;

    fw.patterns(&patterns)
            .exclude_patterns(&exclude_patterns)
            .delay(delay)
            .interval(interval)
            .restart(restart)
    .run();
}