pub trait CliOutput {
    fn print<T: std::fmt::Display>(&self, output: &T);
}

pub struct CommandLine;
impl CliOutput for CommandLine {
    fn print<T: std::fmt::Display>(&self, output: &T) {
        println!("{}", output);
    }
}

pub struct DummyOutput;
impl CliOutput for DummyOutput {
    fn print<T: std::fmt::Display>(&self, _output: &T) {}
}

pub enum CliOutputs {
    CommandLine(CommandLine),
    Dummy(DummyOutput),
}

impl CliOutputs {
    pub fn from_bool(is_quiet: bool) -> Self {
        iif!(
            is_quiet,
            CliOutputs::Dummy(DummyOutput),
            CliOutputs::CommandLine(CommandLine)
        )
    }
}

impl CliOutput for CliOutputs {
    fn print<T: std::fmt::Display>(&self, output: &T) {
        match self {
            CliOutputs::CommandLine(cli) => cli.print(output),
            CliOutputs::Dummy(cli) => cli.print(output),
        }
    }
}
