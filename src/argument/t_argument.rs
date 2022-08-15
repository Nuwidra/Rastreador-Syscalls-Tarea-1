use crate::arguments_summary::ArgumentsSummary;

pub trait TArgument {
    fn get_text(&self) -> &String;
    fn summarize_argument(&self, summary:&mut ArgumentsSummary);
}