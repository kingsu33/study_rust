use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password: String,
}