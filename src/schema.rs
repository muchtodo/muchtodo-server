table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
        client_id -> Int4,
        user_id -> Int4,
        grant_id -> Int4,
        scope -> Varchar,
        issued_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

table! {
    auth_codes (id) {
        id -> Int4,
        client_id -> Int4,
        name -> Varchar,
        scope -> Varchar,
        expires_at -> Timestamptz,
        redirect_uri -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

table! {
    client_redirect_uris (id) {
        id -> Int4,
        client_id -> Int4,
        redirect_uri -> Varchar,
    }
}

table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        url -> Nullable<Varchar>,
        description -> Nullable<Text>,
        icon -> Nullable<Text>,
        user_id -> Int4,
        identifier -> Varchar,
        secret -> Varchar,
        response_type -> Varchar,
    }
}

table! {
    grant_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    refresh_tokens (id) {
        id -> Int4,
        token -> Varchar,
        client_id -> Int4,
        access_token_id -> Int4,
        scope -> Varchar,
        issued_at -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        user_id -> Int4,
        created -> Timestamptz,
        modified -> Timestamptz,
        name -> Text,
        notes -> Nullable<Text>,
        completable -> Bool,
        completed -> Bool,
        start_date -> Nullable<Date>,
        due_date -> Nullable<Date>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Text,
        first_name -> Varchar,
        date_joined -> Timestamptz,
        is_staff -> Bool,
        is_active -> Bool,
        avatar -> Nullable<Text>,
    }
}

joinable!(access_tokens -> clients (client_id));
joinable!(access_tokens -> grant_types (grant_id));
joinable!(access_tokens -> users (user_id));
joinable!(auth_codes -> clients (client_id));
joinable!(auth_codes -> users (user_id));
joinable!(client_redirect_uris -> clients (client_id));
joinable!(clients -> users (user_id));
joinable!(refresh_tokens -> access_tokens (access_token_id));
joinable!(refresh_tokens -> clients (client_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    auth_codes,
    client_redirect_uris,
    clients,
    grant_types,
    refresh_tokens,
    tasks,
    users,
);
