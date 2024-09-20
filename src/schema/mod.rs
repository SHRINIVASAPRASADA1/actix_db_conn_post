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