use super::compare_type::CompareType;
use super::compare_symbol::CompareSymbol;

#[derive(Debug, PartialEq, Clone)]
pub struct Compare {
    pub compare_types: [CompareType; 2],
    pub symbols: Vec<CompareSymbol>,
}
