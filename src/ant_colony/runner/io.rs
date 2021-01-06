pub trait CliOutput {
    fn print<Summary: std::fmt::Display>(&self, output: &Summary) {
        println!("{}", output);
    }
}

pub struct CommandLine;

impl CliOutput for CommandLine {
    fn print<Summary: std::fmt::Display>(&self, output: &Summary) {
        println!("{}", output);
    }
}

pub struct DummyOutput;

impl CliOutput for DummyOutput {
    fn print<Summary: std::fmt::Display>(&self, _output: &Summary) {}
}
