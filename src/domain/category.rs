use regex::Regex;
pub struct Category;

impl Category{
    pub fn build_category(description: &str) -> Option<i64> {
        let rules: [(&str, i64); 10] = [
            (r"(?i)\b(salary|payroll|wage|wages|bonus)\b", 2), 
            (r"(?i)\b(lidl|kaufland|auchan|carrefour|mega|cora|shop|shopping|groceries)\b", 3),
            (r"(?i)\b(uber|bolt|taxi|bus|transport|plane|train|fuel|gasoline|diesel|petrol)\b", 4),   
            (r"(?i)\b(ikea|jysk|mobexpert|furniture|bed|table|chair|wardrobe)\b", 5),                   
            (r"(?i)\b(netflix|spotify|music|subscription|youtube)\b", 6),
            (r"(?i)\b(tax|taxes|electricity|TV|bills|bill|cable|phone)\b", 7),  
            (r"(?i)\b(scholarship)\b", 8),      
            (r"(?i)\b(business|profit|rental|dividents)\b", 9),      
            (r"(?i)\b(pension)\b", 10),
            (r"(?i)\b(inheritance|refund|gift)\b", 11)

        ];
        for (rule, category_id) in rules {
            if let Ok(re) = Regex::new(rule) {
                if re.is_match(description) {
                    return Some(category_id);
                }
            }
        }
        return None;
    }
}