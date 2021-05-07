table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        secret_2fa -> Nullable<Text>,
    }
}
