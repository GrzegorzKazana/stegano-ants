use super::{CycleSummary, EpochSummary};

pub trait CliOutput {
    fn print_cycle_summary(&self, output: &CycleSummary) {
        println!("{}", output);
    }

    fn print_epoch_summary(&self, output: &EpochSummary) {
        println!("{}", output);
    }
}

pub struct CommandLine;

impl CliOutput for CommandLine {}

pub struct DummyOutput;

impl CliOutput for DummyOutput {
    fn print_cycle_summary(&self, _output: &CycleSummary) {}

    fn print_epoch_summary(&self, _output: &EpochSummary) {}
}
