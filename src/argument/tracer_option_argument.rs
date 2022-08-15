use crate::argument::t_argument::TArgument;
use crate::arguments_summary::ArgumentsSummary;

pub struct TracerOptionArgument {
    pub tracer_argument_text: String
}

impl TracerOptionArgument {
    pub fn new(option: String) -> TracerOptionArgument {
        TracerOptionArgument {
            tracer_argument_text: option
        }
    }
}

impl TArgument for TracerOptionArgument {
    fn get_text(&self) -> &String
    {
        &self.tracer_argument_text
    }
    fn summarize_argument(&self, summary:&mut ArgumentsSummary)
    {
        summary.print_syscall_found = true;
        let ch = self.tracer_argument_text.chars().nth(1).unwrap();
        summary.pause_when_syscall_found = ch.is_uppercase();
    }
}
