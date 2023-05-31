use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::PgConnection;
use crate::db::models::*;

//insert

pub fn create_user(conn: &mut PgConnection, nickname: &str, email: &str, password: &str, avatar: &str) -> User {
  use crate::db::schema::users;

  let new_user = NewUser {
    u_nickname: nickname,
    u_email: email,
    u_password: password,
    u_avatar: avatar,
    u_is_online: true,
    u_creattime: chrono::Local::now().naive_local(),
    u_lastlogin: chrono::Local::now().naive_local(),
  };

  diesel::insert_into(users::table)
    .values(&new_user)
    .get_result(conn)
    .expect("Error saving new user")
}

pub fn create_chatroom(conn: &mut PgConnection, name: &str, have_password: bool, password: Option<&str>, owner: i32) -> Chatroom {
  use crate::db::schema::chatroom;

  let new_chatroom = NewChatroom {
    c_name: name,
    c_have_password: have_password,
    c_password: password,
    c_owner: owner,
  };

  diesel::insert_into(chatroom::table)
    .values(&new_chatroom)
    .get_result(conn)
    .expect("Error saving new chatroom")
}

pub fn create_message(conn: &mut PgConnection, chatroom_user_id: i32, content: &str) -> Message {
  use crate::db::schema::message;

  let new_message = NewMessage {
    m_cu_id: chatroom_user_id,
    m_content: content,
    m_creatime: chrono::Local::now().naive_local(),
  };

  diesel::insert_into(message::table)
    .values(&new_message)
    .get_result(conn)
    .expect("Error saving new message")
}

pub fn create_chatroom_user(conn: &mut PgConnection, chatroom_id: i32, user_id: i32) -> ChatroomUser {
  use crate::db::schema::chatroom_users;

  let new_chatroom_user = NewChatroomUser {
    cu_c_id: chatroom_id,
    cu_u_id: user_id,
  };

  diesel::insert_into(chatroom_users::table)
    .values(&new_chatroom_user)
    .get_result(conn)
    .expect("Error saving new chatroom_user")
}

//select

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> User {
  use crate::db::schema::users::dsl::*;

  users.find(user_id)
    .first(conn)
    .expect("Error loading user")
}

pub fn get_user_by_email(conn: &mut PgConnection, user_email: &str) -> User {
  use crate::db::schema::users::dsl::*;

  users.filter(u_email.eq(user_email))
    .first(conn)
    .expect("Error loading user")
}

pub fn get_chatroom_by_id(conn: &mut PgConnection, chatroom_id: i32) -> Chatroom {
  use crate::db::schema::chatroom::dsl::*;

  chatroom.find(chatroom_id)
    .first(conn)
    .expect("Error loading chatroom")
}

pub fn get_chatroom_by_name(conn: &mut PgConnection, chatroom_name: &str) -> Chatroom {
  use crate::db::schema::chatroom::dsl::*;

  chatroom.filter(c_name.eq(chatroom_name))
    .first(conn)
    .expect("Error loading chatroom")
}

pub fn get_chatroom_users_by_chatroom_name(conn: &mut PgConnection, chatroom_id: i32) -> Vec<ChatroomUser> {
  use crate::db::schema::chatroom_users::dsl::*;

  chatroom_users.filter(cu_c_id.eq(chatroom_id))
    .load::<ChatroomUser>(conn)
    .expect("Error loading chatroom_user")
}

pub fn get_messages_by_chatroom_name(conn: &mut PgConnection, chatroom_id: i32) -> Vec<Message> {
  use crate::db::schema::message::dsl::*;

  message.filter(m_cu_id.eq(chatroom_id))
    .load::<Message>(conn)
    .expect("Error loading message")
}

pub fn get_messages_by_chatroom_user_id(conn: &mut PgConnection, chatroom_user_id: i32) -> Vec<Message> {
  use crate::db::schema::message::dsl::*;

  message.filter(m_cu_id.eq(chatroom_user_id))
    .load::<Message>(conn)
    .expect("Error loading message")
}

