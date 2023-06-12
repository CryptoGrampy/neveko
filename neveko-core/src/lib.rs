pub mod args;       // command line arguments
pub mod auth;       // internal auth repo/service layer
pub mod contact;    // contact repo/service layer
pub mod dispute;    // Dispute repo/service layer
pub mod db;         // lmdb interface
pub mod gpg;        // gpgme interface
pub mod i2p;        // i2p repo/service layer
pub mod message;    // message repo/service layer
pub mod models;     // db structs
pub mod monero;     // monero-wallet-rpc interface
pub mod order;      // Order repo/service layer
pub mod product;    // Product repo/service layer
pub mod proof;      // external auth/payment proof module
pub mod reqres;     // http request/responses
pub mod user;       // misc.
pub mod utils;      // user rep/service layer

pub const APP_NAME: &str = "neveko";
pub const NEVEKO_JWP_SECRET_KEY: &str = "NEVEKO_JWP_SECRET_KEY";
pub const NEVEKO_JWT_SECRET_KEY: &str = "NEVEKO_JWT_SECRET_KEY";

/// Environment variable for injecting wallet password
pub const MONERO_WALLET_PASSWORD: &str = "MONERO_WALLET_PASSWORD";

/// The latest monero release download
pub const MONERO_RELEASE_VERSION: &str = "monero-linux-x64-v0.18.2.2.tar.bz2";
pub const MONERO_RELEASE_HASH: &str =
    "186800de18f67cca8475ce392168aabeb5709a8f8058b0f7919d7c693786d56b";
/// The latest i2p-zero release version
pub const I2P_ZERO_RELEASE_VERSION: &str = "v1.21";
pub const I2P_ZERO_RELEASH_HASH: &str =
    "14f34052ad6abb0c24b048816b0ea86b696ae350dd139dd1e90a67ca88e1d07a";
// DO NOT EDIT BELOW THIS LINE
