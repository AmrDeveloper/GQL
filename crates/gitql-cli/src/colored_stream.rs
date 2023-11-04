use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ColoredStream {
    stdout: StandardStream,
}

impl Default for ColoredStream {
    fn default() -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
        }
    }
}

impl ColoredStream {
    pub fn print(&mut self, input: &str) {
        _ = write!(&mut self.stdout, "{}", input);
    }

    pub fn println(&mut self, input: &str) {
        _ = writeln!(&mut self.stdout, "{}", input);
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        _ = self.stdout.set_color(ColorSpec::new().set_fg(color));
    }

    pub fn reset(&mut self) {
        _ = self.stdout.reset();
    }
}
