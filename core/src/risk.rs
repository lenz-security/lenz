use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskReport {
    pub risk_score: u8,
    pub is_safe: bool,
    pub primary_risk: String,
    pub flags: Vec<String>,
}

impl Default for RiskReport {
    fn default() -> Self {
        Self {
            risk_score: 0,
            is_safe: true,
            primary_risk: "None".to_string(),
            flags: vec![],
        }
    }
}
