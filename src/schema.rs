//! ORM types generated by Diesel.
//!
//! Each child module represents a table in the database.

#![allow(missing_docs)]

table! {
    access_tokens {
        id -> BigSerial,
        user_id -> Text,
        value -> Text,
        revoked -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users {
        id -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
