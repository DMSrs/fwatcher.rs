# Fwatcher

Auto run command when some files changed.


## Install

`fwatcher` is implemented by rust language,
so you need `cargo` command:

```
cargo install fwatcher
```

`fwatcher` will be installed in your cargo binary directory(`~/.cargo/bin/`).


## CLI

`fwarcher` can be use as a command:

```
$ fwatcher -h
Usage:
    fwatcher [options] CMD

Options:
    -h, --help          Display this message
    -v, --version       Print version info
    -r, --restart       Auto restart command, default to false
    -d, --directory <dir>
                        Watch directory, default to current directory
    -p, --pattern <pattern>
                        Watch file glob pattern, default to "*"
    -P, --exclude_pattern <exclude_pattern>
                        Watch file glob pattern exclusively, default null
        --delay <second>
                        Delay in seconds for watcher, default to 2
    -i, --interval <second>
                        Interval in seconds to scan filesystem, default to 1
```

For example to search recursively for python files in the current directory
and run pytest when a file is updated:

```
fwatcher -p "**/*.py" pytest --maxfail=2
```

you can also use more than one directory/pattern option:

```
fwatcher -d src -d test -p "**/*.py" -p "**/*.html" pytest --maxfail=2
```

The `--restart` option kills the command
if it's still running when a filesystem change happens.
Can be used to restart locally running webservers on updates,
or kill long running tests and restart on updates:

```
fwatcher -d src -p "**/*.py" --restart run_forever_cmd
```


## Rust Lib

Dependencies in your project's `Cargo.toml`:

```toml
[dependencies]
glob = "0.2"
notify = "4.0"
fwatcher = "*"
```

The following example shows simple usage:

```rust
extern crate glob;
extern crate fwatcher;

use fwatcher::Fwatcher;
use glob::Pattern;
use std::path::PathBuf;
use std::time::Duration;


fn main() {
    let dirs =vec![PathBuf::from("src")];
    let cmd = vec!["pytest".to_string()];

    let mut fwatcher = Fwatcher::new(dirs, cmd);
    fwatcher.pattern(Pattern::new("**/*.py").unwrap())
            .exclude_pattern(Pattern::new("**/.git/**").unwrap())
            .interval(Duration::new(1, 0))
            .restart(false)
            .run();
}
```
