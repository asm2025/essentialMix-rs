// Diesel schema definitions
// This file contains all table definitions in one place, which is the standard Diesel approach

use diesel::prelude::*;

diesel::table! {
    images (id) {
        id -> BigInt,
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

diesel::table! {
    tags (id) {
        id -> BigInt,
        name -> Text,
    }
}

diesel::table! {
    image_tags (image_id, tag_id) {
        image_id -> BigInt,
        tag_id -> BigInt,
    }
}

diesel::joinable!(image_tags -> images (image_id));
diesel::joinable!(image_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    tags,
    image_tags,
);

