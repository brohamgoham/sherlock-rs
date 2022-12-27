use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcResult<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcErrorMessage<T> {
    pub message: String,
    pub code: i64,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcError<T> {
    pub jsonrpc: String,
    pub id: String,
    pub error: JsonRpcErrorMessage<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tag {
    Latest,
    Earliest,
    Pending,
}

impl From<Tag> for String {
    fn from(t: Tag) -> Self {
        String::from(match t {
            Tag::Latest => "latest",
            Tag::Earliest => "earliest",
            Tag::Pending => "pending",
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub base_fee_per_gas: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3uncles: Option<String>,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions: Vec<Transaction>,
    pub transactions_root: String,
    pub uncles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub access_list: Option<Vec<AccessList>>,
    pub block_hash: String,
    pub block_number: String,
    pub chain_id: Option<String>,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub max_fee_per_gas: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub to: String,
    pub transaction_index: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub v: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessList {
    pub address: String,
    pub storage_keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: Option<String>,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<ReceiptLog>,
    pub logs_bloom: String,
    pub status: String,
    pub to: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    #[serde(rename = "type")]
    pub receipt_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptLog {
    pub address: String,
    pub block_hash: String,
    pub block_number: String,
    pub data: String,
    pub log_index: String,
    pub removed: bool,
    pub topics: Vec<String>,
    pub transaction_hash: String,
    pub transaction_index: String,
}
