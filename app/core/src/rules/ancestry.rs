use serde::{Deserialize, Serialize};

/// Your characters ancestral heritage.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ancestry {
    pub id: String,
    pub name: String,
}
