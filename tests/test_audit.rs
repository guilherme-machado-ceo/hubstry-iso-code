// tests/test_audit.rs

extern crate hubstry_iso_code;

use hubstry_iso_code::audit::{self, LogLevel};

#[test]
fn test_audit_log_runs() {
    // Este teste apenas verifica se a função de log pode ser chamada sem pânico.
    // A verificação da saída de log real exigiria uma configuração mais complexa.
    audit::log(LogLevel::Info, "Test log message");
    // Nenhuma asserção é necessária aqui, o teste passa se não houver pânico.
}