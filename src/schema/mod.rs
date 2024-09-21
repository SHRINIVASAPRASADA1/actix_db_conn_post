use diesel::table;


table! {
    category(id){
        id -> Integer,
        title -> Text,
        subtitle -> Text,
        keywords -> Jsonb,
    }
}

table! {
    products (id) {
        id -> Integer,
        title -> Text,
        category_id -> Integer,
        price -> Integer,
        description -> Text,
        discount -> Integer,
        quantity -> Integer,
        category -> Text,
        brand -> Text,
        thumbnail -> Text,
        weight -> Float,
        height -> Float,
        is_active -> Bool,
        tag -> Jsonb,
        images -> Jsonb,
        features -> Jsonb,
    }
 }
 

table! {
    users(id){
        id -> Integer,
        username -> Text,
        email -> Text ,
        contact -> Text ,
        profile_image -> Text ,
        password -> Text,
    }
}



table! {
    user_address (id) {
        id -> Integer,
        user_id -> Integer,
        pin_code -> Integer,
        area -> Nullable<Text>,
        city -> Nullable<Text>,
        contact -> Nullable<Text>,
        added_date -> Date,
        other -> Nullable<Text>,
    }
}

table! {
    user_cart (id) {
        id -> Integer,
        user_id -> Integer,
        product_id -> Integer,
        quantity -> Nullable<Integer>,
        added_date -> Date,
        is_active -> Nullable<Bool>,
        extra -> Nullable<Jsonb>,
    }
}

table! {
    user_order (id) {
        id -> Integer,
        user_id -> Integer,
        product_id -> Integer,
        quantity -> Nullable<Integer>,
        added_date -> Date,
        is_accepted -> Nullable<Bool>,
        is_cancelled -> Nullable<Bool>,
        is_online_pay -> Bool,
        extra -> Nullable<Jsonb>,
    }
}

table! {
    order_status (id) {
        id -> Integer,
        product_id -> Integer,
        order_id -> Integer,
        last_updated -> Date,
        is_accepted -> Nullable<Bool>,
        locations -> Nullable<Jsonb>,
    }
}








