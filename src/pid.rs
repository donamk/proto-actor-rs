use std::fmt::{Display, Formatter, Result as FmtResult};

/// PID is a reference to an actor (or any other process). It consists of actor system address and an identifier.
pub struct PID {
    pub address: String,
    pub id: String,
    request_id: Option<String>,
}

impl Display for PID {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}/{} ({:?})", self.address, self.id, self.request_id)
    }
}

impl PID {
    /// Creates a new PID instance from address and identifier.
    /// 
    /// # Arguments
    /// * `address` - Actor system address
    /// * `id` - Actor identifier
    pub fn new(address: String, id: String) -> Self {
        PID { address, id, request_id: None }
    }
}
