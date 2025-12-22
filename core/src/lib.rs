pub mod error;
pub mod scanner;
pub mod disassembler;
pub mod risk;
pub mod constants;
pub mod utils;

pub use error::{LenzError, Result};
pub use scanner::AuditEngine;
pub use risk::RiskReport;
pub use constants::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        assert_eq!(constants::MAX_RISK_SCORE, 100);
    }
}
