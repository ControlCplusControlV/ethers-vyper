pub struct JsonCompilerConfig {
    language: String,
    sources: Vec<Contract>
}

impl Default for JsonCompilerConfig {
    fn default() -> Self {
        JsonCompilerConfig { 
            language: "Vyper".to_string()
        }
    }
}