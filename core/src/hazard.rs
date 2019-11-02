use utils::error::Result;

/// Return, randomly, true or false
pub fn generate_hazard() -> Result<bool> {
    Ok(rand::random())
}
