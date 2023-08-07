// @generated automatically by Diesel CLI.

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    locations (id) {
        id -> Int4,
        name -> Varchar,
        coordinates -> Geometry,
        published -> Bool,
        description -> Nullable<Text>,
        osm_node_id -> Nullable<Varchar>,
        google_place_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(locations, spatial_ref_sys,);
