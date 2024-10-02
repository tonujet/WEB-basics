use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    pub take: Option<u32>,
    pub offset: Option<u32>,
}
