#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::db::schema::users)]
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

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
  pub u_nickname: &'a str,
  pub u_email: &'a str,
  pub u_password: &'a str,
  pub u_avatar: &'a str,
  pub u_is_online: bool,
  pub u_creattime: chrono::NaiveDateTime,
  pub u_lastlogin: chrono::NaiveDateTime,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::db::schema::chatroom)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chatroom {
  pub c_id: i32,
  pub c_name: String,
  pub c_have_password: bool,
  pub c_password: Option<String>,
  pub c_owner: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::chatroom)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChatroom<'a> {
  pub c_name: &'a str,
  pub c_have_password: bool,
  pub c_password: Option<&'a str>,
  pub c_owner: i32,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::db::schema::chatroom_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatroomUser {
  pub cu_id: i32,
  pub cu_c_id: i32,
  pub cu_u_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::chatroom_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChatroomUser {
  pub cu_c_id: i32,
  pub cu_u_id: i32,
}

#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::db::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
  pub m_id: i32,
  pub m_cu_id: i32,
  pub m_content: String,
  pub m_creatime: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage<'a> {
  pub m_cu_id: i32,
  pub m_content: &'a str,
  pub m_creatime: chrono::NaiveDateTime,
}