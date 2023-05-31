// @generated automatically by Diesel CLI.

diesel::table! {
    chatroom (c_id) {
        c_id -> Int4,
        #[max_length = 32]
        c_name -> Varchar,
        c_have_password -> Bool,
        #[max_length = 32]
        c_password -> Nullable<Varchar>,
        c_owner -> Int4,
    }
}

diesel::table! {
    chatroom_users (cu_id) {
        cu_id -> Int4,
        cu_c_id -> Int4,
        cu_u_id -> Int4,
    }
}

diesel::table! {
    message (m_id) {
        m_id -> Int4,
        m_cu_id -> Int4,
        #[max_length = 255]
        m_content -> Varchar,
        m_creatime -> Timestamp,
    }
}

diesel::table! {
    users (u_id) {
        u_id -> Int4,
        #[max_length = 32]
        u_nickname -> Varchar,
        #[max_length = 32]
        u_email -> Varchar,
        #[max_length = 32]
        u_password -> Varchar,
        #[max_length = 32]
        u_avatar -> Varchar,
        u_is_online -> Bool,
        u_creattime -> Timestamp,
        u_lastlogin -> Timestamp,
    }
}

diesel::joinable!(chatroom -> users (c_owner));
diesel::joinable!(chatroom_users -> chatroom (cu_c_id));
diesel::joinable!(chatroom_users -> users (cu_u_id));
diesel::joinable!(message -> chatroom_users (m_cu_id));

diesel::allow_tables_to_appear_in_same_query!(
    chatroom,
    chatroom_users,
    message,
    users,
);
