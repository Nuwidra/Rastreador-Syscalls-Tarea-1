use crate::argument::t_argument::TArgument;

pub trait TParser {
    fn parse(&self, args: Vec<String>) -> Result<Vec::<Box<dyn TArgument>>, String>;
}
