use crate::argument::t_argument::TArgument;
use crate::arguments_summary::ArgumentsSummary;

pub struct ProgramArgument {
    pub program_command: String
}

impl ProgramArgument {
    pub fn new(slice: &[String]) -> ProgramArgument {
        ProgramArgument {
            program_command: slice.join(" ").to_string()
        }
    }
}

impl TArgument for ProgramArgument {
    fn get_text(&self) -> &String
    {
        &self.program_command
    }
    fn summarize_argument(&self, summary:&mut ArgumentsSummary)
    {
        summary.program_command = self.program_command.clone();
    }
}

