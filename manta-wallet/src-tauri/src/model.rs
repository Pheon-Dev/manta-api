// Send API Endpoint
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SendRequest {
    pub id: u64,
    pub cid: u64,
    pub amount: u32,
    pub receiver: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SendRequestCreate {
    pub amount: u32,
    pub receiver: String,
    pub description: String,
}
