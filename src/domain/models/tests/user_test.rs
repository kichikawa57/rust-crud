#[cfg(test)]
mod tests {
  use crate::domain::models::user::{User, Users};

  #[test]
  fn test_user_new_with_none() {
    let expected = User {
      id: 1,
      name: "user 01".to_string(),
    };
    let result = User::new(None);
    assert_eq!(expected, result);
  }

  #[test]
  fn test_user_new_with_some() {
    let user_data = User {
      id: 10,
      name: "test user".to_string(),
    };
    let result = User::new(Some(user_data.clone()));
    assert_eq!(user_data, result);
  }

  #[test]
  fn test_users_new() {
    let expected_users = vec![
      User {
        id: 1,
        name: String::from("user 01"),
      },
      User {
        id: 2,
        name: String::from("user 02"),
      },
      User {
        id: 3,
        name: String::from("user 03"),
      },
    ];
    let users = Users::new();
    assert_eq!(expected_users, users.users);
  }
}
