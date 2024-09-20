// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        price -> Int4,
        description -> Nullable<Text>,
        discount -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        category -> Nullable<Varchar>,
        brand -> Nullable<Varchar>,
        thumbnail -> Nullable<Varchar>,
        weight -> Nullable<Float4>,
        height -> Nullable<Float4>,
        is_active -> Nullable<Bool>,
        tag -> Nullable<Jsonb>,
        images -> Nullable<Jsonb>,
        features -> Nullable<Jsonb>,
    }
}
