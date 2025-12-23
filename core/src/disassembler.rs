use crate::error::{LenzError, Result};

pub struct Disassembler;

impl Disassembler {
    pub fn parse(data: &[u8]) -> Result<Vec<String>> {
        if data.len() < 4 {
            // Minimal mock check
            return Ok(vec!["NO_OP".to_string()]);
        }
        // Placeholder for SBF parsing logic
        Ok(vec![
            "LD_DW_IMM".to_string(),
            "ALU64_ADD_K".to_string(),
            "EXIT".to_string()
        ])
    }
}
