use crate::argument::program_argument::ProgramArgument as ProgramArgument;
use crate::argument::tracer_option_argument::TracerOptionArgument as TracerOptionArgument;
use crate::argument_parser::t_parser::TParser;
use crate::argument::t_argument::TArgument;
 // Recibe una linea de texto y parsea una lista de strings con sigo una lista de argumentos
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}

impl TParser for Parser {
    fn parse(&self, args: Vec<String>) -> Result<Vec::<Box<dyn TArgument>>, String> 
    {
        let mut program_index = 2;
        let mut vector = Vec::<Box<dyn TArgument>>::new();
        if args.len() < 2 
        {
            return Err("Invalid amount of parameters".to_string());
        }

        if args[1].to_lowercase().eq("-v")
        {
            vector.push(Box::new(TracerOptionArgument::new(args[1].clone())));
            
        }
        else {
            program_index = 1;
        }
        vector.push(Box::new(ProgramArgument::new(&args[program_index..args.len()])));
        Ok(vector)
    }
}