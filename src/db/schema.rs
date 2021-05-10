table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        secret_2fa -> Nullable<Text>,
        reset_token -> Nullable<Text>,
        reset_token_created_at -> Nullable<Timestamp>,
    }
}
