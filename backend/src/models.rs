use diesel::{prelude::*};

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub u_id: i32,
  pub u_nickname: String,
  pub u_email: String,
  pub u_password: String,
  pub u_avatar: String,
  pub u_is_online: bool,
  pub u_creattime: chrono::NaiveDateTime,
  pub u_lastlogin: chrono::NaiveDateTime,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::chatroom)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatroom {
  pub c_id: i32,
  pub c_name: String,
  pub c_have_password: bool,
  pub c_password: Option<String>,
  pub c_owner: Option<i32>,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::chatroom_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatroomUser {
  pub cu_id: i32,
  pub cu_c_id: Option<i32>,
  pub cu_u_id: Option<i32>,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
  pub m_id: i32,
  pub m_cu_id: Option<i32>,
  pub content: String,
  pub creatime: Option<chrono::NaiveDateTime>,
}