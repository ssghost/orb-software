//! Data link between Worldcoin App and Orb through QR-codes.
//!
//! Worldcoin App needs to transfer considerable amount of data to an Orb to
//! begin a new signup. Encoding all the data into a single or a series or QR-
//! codes would compromise QR scanning performance. On the other hand just
//! leting the Orb to download all the data from the backend would compromise
//! security.
//!
//! Therefore we employ a hybrid approach, where we transfer the data via the
//! backend for performance, and transfer a hash of the data via a QR-code for
//! security.
//!
//! # Usage
//!
//! The library is shared between Worldcoin App and Orb Core.
//!
//! ## Encode (App)
//!
//! Worldcoin App uploads user data and generates a QR-code.
//!
//! ```rust
//! use orb_qr_link::{encode_qr, DataPolicy, UserData};
//! use uuid::Uuid;
//!
//! // Generate a new session id and user data.
//! let session_id = Uuid::new_v4();
//! let user_data = UserData {
//!     identity_commitment: String::new(),
//!     self_custody_public_key: String::new(),
//!     data_policy: DataPolicy::OptOut,
//! };
//!
//! // Upload `user_data` to the backend by the `session_id` key.
//! // ...
//!
//! // Calculate a variable-length hash of `user_data`.
//! let user_data_hash = user_data.hash(16);
//! // Encode a new QR-code.
//! let qr = encode_qr(&session_id, user_data_hash);
//!
//! // Allow the Orb to scan the generated QR-code.
//! // ...
//! ```
//!
//! ## Decode (Orb)
//!
//! The Orb scans a QR-code and downloads the user data.
//!
//! ```rust
//! use orb_qr_link::{decode_qr, DataPolicy, UserData};
//!
//! // Scan QR-code generated by the App.
//! let qr = "3WVd+tbAtSgyH0Ce9uiKT9i063t/xG2HxTIhuNa+gNnM";
//!
//! // Decode the QR-code string.
//! let (session_id, user_data_hash) = decode_qr(qr).unwrap();
//!
//! // Download `user_data` from the backend by the `session_id` key.
//! let user_data = UserData {
//!     identity_commitment: String::new(),
//!     identity_commitment: String::new(),
//!     data_policy: DataPolicy::OptOut,
//! };
//!
//! // Verify that the `user_data_hash` from the QR-code matches `user_data`
//! // from the backend.
//! let success = user_data.verify(user_data_hash);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "decode")]
mod decode;
#[cfg(feature = "encode")]
mod encode;
mod user_data;

#[cfg(feature = "decode")]
pub use decode::{decode_qr, DecodeError};
#[cfg(feature = "encode")]
pub use encode::encode_qr;
pub use user_data::{DataPolicy, UserData};
