// @generated automatically by Diesel CLI.

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::sql_types::*;

    locations (id) {
        id -> Int4,
        name -> Varchar,
        coordinates -> Geometry,
        published -> Bool,
        description -> Nullable<Text>,
        osm_node_id -> Nullable<Varchar>,
        google_place_id -> Nullable<Varchar>,
        imageurl -> Nullable<Text>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    use crate::sql_types::*;

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
    use crate::sql_types::*;

    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        role -> UserRole,
    }
}

diesel::allow_tables_to_appear_in_same_query!(locations, spatial_ref_sys, users,);
