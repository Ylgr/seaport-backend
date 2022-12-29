// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type"))]
    pub struct TokenType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenType;

    considerations (id) {
        id -> Uuid,
        order_id -> Uuid,
        recipient -> Varchar,
        #[sql_name = "type"]
        type_ -> TokenType,
        token_address -> Nullable<Varchar>,
        amount -> Nullable<Varchar>,
        end_amount -> Nullable<Varchar>,
        identifier -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenType;

    offers (id) {
        id -> Uuid,
        order_id -> Uuid,
        #[sql_name = "type"]
        type_ -> TokenType,
        token_address -> Nullable<Varchar>,
        amount -> Nullable<Varchar>,
        end_amount -> Nullable<Varchar>,
        identifier -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Uuid,
        create_by -> Varchar,
        signature -> Text,
    }
}

diesel::joinable!(considerations -> orders (order_id));
diesel::joinable!(offers -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    considerations,
    offers,
    orders,
);
