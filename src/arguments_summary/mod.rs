use crate::argument::t_argument::TArgument;
// ArgumentsSummary toma los argumentos de agument_parser y summary

// Lo que hace recibir una lista de argumentos y resumirlos en un objeto que se llama ArgumentsSummary
// Para que el tracer no se preocupe en los argumentos y solo llame a ArgumentsSummary de la infromación que necesita
#[derive(Debug, Default)]
pub struct ArgumentsSummary {
    pub print_syscall_found : bool,
    pub pause_when_syscall_found : bool,
    pub program_command : String
}

impl ArgumentsSummary {
    pub fn summarize(arguments: Vec::<Box<dyn TArgument>>) -> ArgumentsSummary {

        let mut summary = ArgumentsSummary::default();
        for f in arguments {
            f.summarize_argument(&mut summary);
        }
        summary
    }
}
