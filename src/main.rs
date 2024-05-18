use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::ffi::OsString;
use std::io::{self, Read};
use std::time::Duration;
use std::{fs, process, thread};

struct Args {
    thing: Option<String>,
    infile: Option<OsString>,
    wait_sec: u64,
    delay_milli: u64,
    trim_end: bool,
}

const HELP: &str = "\
USAGE: wtype-rs [OPTIONS] STRING

OPTIONS:
  --wait, -w <SEC>      wait <SEC> seconds before typing
  --delay, -d <MILLI>   delay each keystroke by <MILLI> milliseconds
  --file, -f <FILE>     type contents of <FILE>
  --trim                trim last trailing whitespace

ARGS:
  <STRING>              type <STRING>
";

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut thing = None;
    let mut infile = None;
    let mut wait_sec = 5;
    let mut delay_milli = 120;
    let mut trim_end = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('w') | Long("wait_sec") => {
                wait_sec = parser.value()?.parse()?;
            }
            Short('d') | Long("delay") => {
                delay_milli = parser.value()?.parse()?;
            }
            Value(val) if thing.is_none() => {
                thing = Some(val.string()?);
            }
            Short('f') | Long("file") => {
                infile = Some(parser.value()?);
            }
            Long("trim") => {
                trim_end = true;
            }
            Short('h') | Long("help") => {
                println!("{HELP}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        thing,
        infile,
        wait_sec,
        delay_milli,
        trim_end,
    })
}

fn main() -> io::Result<()> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    thread::sleep(Duration::from_secs(args.wait_sec));

    let mut buffer = String::new();

    if let Some(f) = args.infile {
        buffer = fs::read_to_string(f)?;
    } else if let Some(s) = args.thing {
        buffer = s;
    } else {
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buffer)?;
    };

    if args.trim_end {
        buffer.truncate(buffer.trim_end().len());
    };

    for i in buffer.chars() {
        if enigo.key(Key::Unicode(i), Click).is_err() {
            process::exit(1)
        }
        thread::sleep(Duration::from_millis(args.delay_milli));
    }

    Ok(())
}
