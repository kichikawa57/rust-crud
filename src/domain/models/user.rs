use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct User {
  pub(crate) id: u32,
  pub(crate) name: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Users {
  pub(crate) users: Vec<User>,
}

impl User {
  pub fn new(new_user: Option<Self>) -> Self {
    new_user.unwrap_or(User {
      id: 1,
      name: String::from("user 01"),
    })
  }
}

impl Users {
  pub fn new() -> Self {
    let users: Vec<User> = vec![
      User::new(Some(User {
        id: 1,
        name: String::from("user 01"),
      })),
      User::new(Some(User {
        id: 2,
        name: String::from("user 02"),
      })),
      User::new(Some(User {
        id: 3,
        name: String::from("user 03"),
      })),
    ];

    Self { users }
  }
}
