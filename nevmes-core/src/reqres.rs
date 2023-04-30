use crate::utils;
use serde::{Deserialize, Serialize};

// All http requests and responses are here

// START XMR Structs

// params
#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcValidateAddressParams {
    pub address: String,
    pub any_net_type: bool,
    pub allow_openalias: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyParams {
    pub address: String,
    pub data: String,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCreateWalletParams {
    pub filename: String,
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcOpenWalletParams {
    pub filename: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcMakeParams {
    pub multisig_info: Vec<String>,
    pub threshold: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcFinalizeParams {
    pub multisig_info: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcImportParams {
    pub info: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSignMultisigParams {
    pub tx_data_hex: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcBalanceParams {
    pub account_index: u8,
    pub address_indices: Vec<u8>,
    pub all_accounts: bool,
    pub strict: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcAddressParams {
    pub account_index: u8,
    pub address_index: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCheckTxProofParams {
    pub address: String,
    pub message: String,
    pub signature: String,
    pub txid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcTxProofParams {
    pub address: String,
    pub message: String,
    pub txid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxProofParams {
    pub address: String,
    pub message: String,
    pub txid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxByIdParams {
    pub txid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Destination {
    pub address: String,
    pub amount: u128,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcTransferParams {
    pub destinations: Vec<Destination>,
    pub account_index: u32,
    pub subaddr_indices: Vec<u32>,
    pub priority: u8,
    pub ring_size: u32,
    pub get_tx_key: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSweepAllParams {
    pub address: String,
}

// requests
#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcValidateAddressRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcValidateAddressParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCreateRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcCreateWalletParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcOpenRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcOpenWalletParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcAddressRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcAddressParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcBalanceRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcBalanceParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcMakeRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcMakeParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcFinalizeRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcFinalizeParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcImportRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcImportParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSignMultisigRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcSignMultisigParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcVerifyParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCheckTxProofRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcCheckTxProofParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxProofRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcGetTxProofParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxByIdRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcGetTxByIdParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcTransfrerRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcTransferParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSweepAllRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcSweepAllParams,
}

// results
#[derive(Deserialize, Debug)]
pub struct XmrRpcValidateAddressResult {
    pub integrated: bool,
    pub nettype: String,
    pub openalias_address: String,
    pub subaddress: bool,
    pub valid: bool,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResult {
    pub good: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmrRpcVersionResult {
    pub version: u32,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcFinalizeResult {
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcPrepareResult {
    pub multisig_info: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcMakeResult {
    pub address: String,
    pub multisig_info: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcExportResult {
    pub info: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcImportResult {
    pub n_outputs: u8,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcSignMultisigResult {
    pub tx_hash_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SubAddressInfo {
    pub account_index: u8,
    pub address_index: u8,
    pub address: String,
    pub balance: u128,
    pub unlocked_balance: u128,
    pub label: String,
    pub num_unspent_outputs: u8,
    pub time_to_unlock: u128,
    pub blocks_to_unlock: u128,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub address: String,
    pub address_index: u8,
    pub label: String,
    pub used: bool,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcAddressResult {
    pub address: String,
    pub addresses: Vec<Address>,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcBalanceResult {
    pub balance: u128,
    pub unlocked_balance: u128,
    pub multisig_import_needed: bool,
    pub time_to_unlock: u128,
    pub blocks_to_unlock: u128,
    pub per_subaddress: Vec<SubAddressInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCheckTxProofResult {
    pub confirmations: u64,
    pub good: bool,
    pub in_pool: bool,
    pub received: u128,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxProofResult {
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubAddressIndex {
    pub major: u64,
    pub minor: u64,
}

impl Default for SubAddressIndex {
    fn default() -> Self {
        SubAddressIndex {
            major: 0,
            minor: 0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transfer {
    pub address: String,
    pub amount: u128,
    pub amounts: Vec<u128>,
    pub confirmations: u64,
    pub double_spend_seen: bool,
    pub fee: u128,
    pub height: u64,
    pub locked: bool,
    pub note: String,
    pub payment_id: String,
    pub subaddr_index: SubAddressIndex,
    pub subaddr_indices: Vec<SubAddressIndex>,
    pub suggested_confirmations_threshold: u64,
    pub timestamp: u64,
    pub txid: String,
    pub r#type: String,
    pub unlock_time: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcGetTxByIdResult {
    pub transfer: Transfer,
    pub transfers: Vec<Transfer>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcTranferResult {
    pub amount: u128,
    pub fee: u128,
    pub multisig_txset: String,
    pub tx_blob: String,
    pub tx_hash: String,
    pub tx_key: String,
    pub tx_metadata: String,
    pub unsigned_txset: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyImageList {
    key_images: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSweepAllResult {
    pub amount_list: Vec<u128>,
    pub fee_list: Vec<u128>,
    pub multisig_txset: String,
    pub spent_key_images_list: Vec<KeyImageList>,
    pub tx_hash_list: Vec<String>,
    pub unsigned_txset: String,
    pub weight_list: Vec<u128>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmrDaemonGetInfoResult {
    pub adjusted_time: u64,
    pub alt_blocks_count: u64,
    pub block_size_limit: u64,
    pub block_size_median: u64,
    pub block_weight_median: u64,
    pub bootstrap_daemon_address: String,
    pub busy_syncing: bool,
    pub credits: u64,
    pub cumulative_difficulty: u64,
    pub cumulative_difficulty_top64: u64,
    pub database_size: u64,
    pub difficulty: u64,
    pub difficulty_top64: u64,
    pub free_space: u64,
    pub grey_peerlist_size: u64,
    pub height: u64,
    pub height_without_bootstrap: u64,
    pub incoming_connections_count: u32,
    pub mainnet: bool,
    pub nettype: String,
    pub offline: bool,
    pub outgoing_connections_count: u32,
    pub restricted: bool,
    pub rpc_connections_count: u32,
    pub stagenet: bool,
    pub start_time: u64,
    pub status: String,
    pub synchronized: bool,
    pub target: u32,
    pub target_height: u32,
    pub testnet: bool,
    pub top_block_hash: String,
    pub top_hash: String,
    pub tx_count: u64,
    pub tx_pool_size: u32,
    pub untrusted: bool,
    pub update_available: bool,
    pub version: String,
    pub was_bootstrap_ever_used: bool,
    pub white_peerlist_size: u32,
    pub wide_cumulative_difficulty: String,
    pub wide_difficulty: String,
}

// responses
#[derive(Serialize, Deserialize, Debug)]
pub struct XmrDaemonGetInfoResponse {
    pub result: XmrDaemonGetInfoResult,
}

impl Default for XmrDaemonGetInfoResponse {
    fn default() -> Self {
        XmrDaemonGetInfoResponse {
            result: XmrDaemonGetInfoResult {
                adjusted_time: 0,
                alt_blocks_count: 0,
                block_size_limit: 0,
                block_size_median: 0,
                block_weight_median: 0,
                bootstrap_daemon_address: utils::empty_string(),
                busy_syncing: false,
                credits: 0,
                cumulative_difficulty: 0,
                cumulative_difficulty_top64: 0,
                database_size: 0,
                difficulty: 0,
                difficulty_top64: 0,
                free_space: 0,
                grey_peerlist_size: 0,
                height: 0,
                height_without_bootstrap: 0,
                incoming_connections_count: 0,
                mainnet: false,
                nettype: utils::empty_string(),
                offline: false,
                outgoing_connections_count: 0,
                restricted: false,
                rpc_connections_count: 0,
                stagenet: false,
                start_time: 0,
                status: utils::empty_string(),
                synchronized: false,
                target: 0,
                target_height: 0,
                testnet: false,
                top_block_hash: utils::empty_string(),
                top_hash: utils::empty_string(),
                tx_count: 0,
                tx_pool_size: 0,
                untrusted: false,
                update_available: false,
                version: utils::empty_string(),
                was_bootstrap_ever_used: false,
                white_peerlist_size: 0,
                wide_cumulative_difficulty: utils::empty_string(),
                wide_difficulty: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResponse {
    pub result: XmrRpcVerifyResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmrRpcVersionResponse {
    pub result: XmrRpcVersionResult,
}

impl Default for XmrRpcVersionResponse {
    fn default() -> Self {
        XmrRpcVersionResponse {
            result: XmrRpcVersionResult { version: 0 },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcPrepareResponse {
    pub result: XmrRpcPrepareResult,
}

impl Default for XmrRpcPrepareResponse {
    fn default() -> Self {
        XmrRpcPrepareResponse {
            result: XmrRpcPrepareResult {
                multisig_info: String::from(""),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcBalanceResponse {
    pub result: XmrRpcBalanceResult,
}

impl Default for XmrRpcBalanceResponse {
    fn default() -> Self {
        XmrRpcBalanceResponse {
            result: XmrRpcBalanceResult {
                balance: 0,
                unlocked_balance: 0,
                multisig_import_needed: false,
                time_to_unlock: 0,
                blocks_to_unlock: 0,
                per_subaddress: Vec::new(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcValidateAddressResponse {
    pub result: XmrRpcValidateAddressResult,
}

impl Default for XmrRpcValidateAddressResponse {
    fn default() -> Self {
        XmrRpcValidateAddressResponse {
            result: XmrRpcValidateAddressResult {
                integrated: false,
                nettype: utils::empty_string(),
                openalias_address: utils::empty_string(),
                subaddress: false,
                valid: false,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcAddressResponse {
    pub result: XmrRpcAddressResult,
}

impl Default for XmrRpcAddressResponse {
    fn default() -> Self {
        XmrRpcAddressResponse {
            result: XmrRpcAddressResult {
                address: utils::empty_string(),
                addresses: Vec::new(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcMakeResponse {
    pub result: XmrRpcMakeResult,
}

impl Default for XmrRpcMakeResponse {
    fn default() -> Self {
        XmrRpcMakeResponse {
            result: XmrRpcMakeResult {
                address: utils::empty_string(),
                multisig_info: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcFinalizeResponse {
    pub result: XmrRpcFinalizeResult,
}

impl Default for XmrRpcFinalizeResponse {
    fn default() -> Self {
        XmrRpcFinalizeResponse {
            result: XmrRpcFinalizeResult {
                address: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcExportResponse {
    pub result: XmrRpcExportResult,
}

impl Default for XmrRpcExportResponse {
    fn default() -> Self {
        XmrRpcExportResponse {
            result: XmrRpcExportResult {
                info: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcImportResponse {
    pub result: XmrRpcImportResult,
}

impl Default for XmrRpcImportResponse {
    fn default() -> Self {
        XmrRpcImportResponse {
            result: XmrRpcImportResult { n_outputs: 0 },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcSignMultisigResponse {
    pub result: XmrRpcSignMultisigResult,
}

impl Default for XmrRpcSignMultisigResponse {
    fn default() -> Self {
        XmrRpcSignMultisigResponse {
            result: XmrRpcSignMultisigResult {
                tx_hash_list: Vec::new(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcCheckTxProofResponse {
    pub result: XmrRpcCheckTxProofResult,
}

impl Default for XmrRpcCheckTxProofResponse {
    fn default() -> Self {
        XmrRpcCheckTxProofResponse {
            result: XmrRpcCheckTxProofResult {
                confirmations: 0,
                good: false,
                in_pool: false,
                received: 0,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcGetTxProofResponse {
    pub result: XmrRpcGetTxProofResult,
}

impl Default for XmrRpcGetTxProofResponse {
    fn default() -> Self {
        XmrRpcGetTxProofResponse {
            result: XmrRpcGetTxProofResult {
                signature: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcGetTxByIdResponse {
    pub result: XmrRpcGetTxByIdResult,
}

impl Default for XmrRpcGetTxByIdResponse {
    fn default() -> Self {
        XmrRpcGetTxByIdResponse {
            result: XmrRpcGetTxByIdResult {
                transfer: Transfer {
                    address: utils::empty_string(),
                    amount: 0,
                    amounts: Vec::new(),
                    confirmations: 0,
                    double_spend_seen: false,
                    fee: 0,
                    height: 0,
                    locked: false,
                    note: utils::empty_string(),
                    payment_id: utils::empty_string(),
                    subaddr_index: Default::default(),
                    subaddr_indices: Vec::new(),
                    suggested_confirmations_threshold: 0,
                    timestamp: 0,
                    txid: utils::empty_string(),
                    r#type: utils::empty_string(),
                    unlock_time: 0,
                },
                transfers: Vec::new(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcTransferResponse {
    pub result: XmrRpcTranferResult,
}

impl Default for XmrRpcTransferResponse {
    fn default() -> Self {
        XmrRpcTransferResponse {
            result: XmrRpcTranferResult {
                amount: 0,
                fee: 0,
                multisig_txset: utils::empty_string(),
                tx_blob: utils::empty_string(),
                tx_hash: utils::empty_string(),
                tx_key: utils::empty_string(),
                tx_metadata: utils::empty_string(),
                unsigned_txset: utils::empty_string(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcSweepAllResponse {
    pub result: XmrRpcSweepAllResult,
}

impl Default for XmrRpcSweepAllResponse {
    fn default() -> Self {
        XmrRpcSweepAllResponse {
            result: XmrRpcSweepAllResult {
                amount_list: Vec::new(),
                fee_list: Vec::new(),
                multisig_txset: utils::empty_string(),
                spent_key_images_list: Vec::new(),
                tx_hash_list: Vec::new(),
                unsigned_txset: utils::empty_string(),
                weight_list: Vec::new(),
            },
        }
    }
}
// END XMR Structs


/// Container for the message decryption
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DecryptedMessageBody {
    pub mid: String,
    pub body: String,
}

impl Default for DecryptedMessageBody {
    fn default() -> Self {
        DecryptedMessageBody {
            mid: utils::empty_string(),
            body: utils::empty_string(),
        }
    }
}

/// Invoice response for host.b32.i2p/invoice
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Invoice {
    pub address: String,
    pub pay_threshold: u128,
    pub conf_threshold: u64,
}

impl Default for Invoice {
    fn default() -> Self {
        Invoice {
            address: utils::empty_string(),
            pay_threshold: 0,
            conf_threshold: 0,
        }
    }
}

/// Not to be confused with the PaymentProof guard.
/// 
/// This is the response when proving payment
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Jwp {
    pub jwp: String,
}

impl Default for Jwp {
    fn default() -> Self {
        Jwp {
            jwp: utils::empty_string(),
        }
    }
}
