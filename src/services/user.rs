use crate::models::user::User;

pub fn get_users() -> Vec<User> {
    let users = vec![
        User {
            name: "kevin".to_string(),
            email: "test@gmail.com".to_string(),
            password: "testpassword".to_string(),
            age: 21
        },
        User {
            name: "kevin".to_string(),
            email: "test@gmail.com".to_string(),
            password: "testpassword".to_string(),
            age: 21
        },
    ];

    users
}
