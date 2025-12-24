#[cfg(test)]
mod tests {
    use lenz_core::scanner::AuditEngine;
    #[test]
    fn test_empty_bytecode() {
        let res = AuditEngine::scan(&[]);
        assert!(res.is_err());
    }
}