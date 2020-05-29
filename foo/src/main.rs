use termcolor::*;
use std::io::Write;

fn main() {
    let choice = if atty::is(atty::Stream::Stderr) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };
    println!("choice={:?}", choice);
    let mut stderr = StandardStream::stderr(choice);
    println!("supports={:?}", stderr.supports_color());
    stderr.reset().unwrap();
    stderr.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Green))).unwrap();
    write!(stderr, "Foobar").unwrap();
    stderr.set_color(ColorSpec::new().set_bold(true)).unwrap();
    write!(stderr, ":").unwrap();
    stderr.reset().unwrap();
    writeln!(stderr, " testing").unwrap();


}

