use crate::arguments_summary::ArgumentsSummary;
// Es una intefase, solo define 2 metodos
pub trait TArgument {
    fn get_text(&self) -> &String;
    fn summarize_argument(&self, summary:&mut ArgumentsSummary);
}