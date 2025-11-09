// Diesel schema definitions
// This file contains all table definitions in one place, which is the standard Diesel approach
//
// Note: Diesel uses different SQL types for different databases. The schema here is designed
// to work with the backend specified at compile time via cargo features.

// SQLite schema
#[cfg(feature = "sqlite")]
diesel::table! {
    images (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        extension -> Text,
        file_size -> BigInt,
        mime_type -> Text,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        alt_text -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[cfg(feature = "sqlite")]
diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

#[cfg(feature = "sqlite")]
diesel::table! {
    image_tags (image_id, tag_id) {
        image_id -> Integer,
        tag_id -> Integer,
    }
}

// PostgreSQL schema
#[cfg(feature = "postgres")]
diesel::table! {
    images (id) {
        id -> Int8,
        title -> Text,
        description -> Nullable<Text>,
        extension -> Text,
        file_size -> Int8,
        mime_type -> Text,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        alt_text -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[cfg(feature = "postgres")]
diesel::table! {
    tags (id) {
        id -> Int8,
        name -> Text,
    }
}

#[cfg(feature = "postgres")]
diesel::table! {
    image_tags (image_id, tag_id) {
        image_id -> Int8,
        tag_id -> Int8,
    }
}

// MySQL schema
#[cfg(feature = "mysql")]
diesel::table! {
    images (id) {
        id -> Bigint,
        title -> Text,
        description -> Nullable<Text>,
        extension -> Varchar,
        file_size -> Bigint,
        mime_type -> Varchar,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        alt_text -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[cfg(feature = "mysql")]
diesel::table! {
    tags (id) {
        id -> Bigint,
        name -> Varchar,
    }
}

#[cfg(feature = "mysql")]
diesel::table! {
    image_tags (image_id, tag_id) {
        image_id -> Bigint,
        tag_id -> Bigint,
    }
}

diesel::joinable!(image_tags -> images (image_id));
diesel::joinable!(image_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    tags,
    image_tags,
);

