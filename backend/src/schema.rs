// @generated automatically by Diesel CLI.

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    areas (id) {
        id -> Int4,
        name -> Varchar,
        area -> Geometry,
        area_type -> AreaType,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    locations (id) {
        id -> Int4,
        name -> Varchar,
        coordinates -> Geometry,
        published -> Bool,
        description -> Nullable<Text>,
        osm_node_id -> Nullable<Varchar>,
        google_place_id -> Nullable<Varchar>,
        imageurl -> Nullable<Text>,
        address_line -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        role -> UserRole,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::db::sql_types::*;

    visits (id) {
        id -> Int4,
        user_id -> Int4,
        location_id -> Int4,
        visited_at -> Timestamp,
    }
}

diesel::joinable!(visits -> locations (location_id));
diesel::joinable!(visits -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(areas, locations, spatial_ref_sys, users, visits,);
