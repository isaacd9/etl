mod catalog;
mod client;
mod core;
mod encoding;
mod error;
mod schema;
#[cfg(feature = "test-utils")]
pub mod test_utils;

pub use client::IcebergClient;
pub use core::{
    DestinationNamespace, IcebergDestination, IcebergOperationType,
    table_name_to_iceberg_table_name,
};
pub use encoding::{UNIX_EPOCH, rows_to_record_batch};
pub use iceberg::io::{S3_ACCESS_KEY_ID, S3_ENDPOINT, S3_SECRET_ACCESS_KEY};
pub use schema::{postgres_columns_to_iceberg_fields, postgres_to_iceberg_schema};
