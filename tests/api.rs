use seventh_core::add;
use seventh_core::api::data::*;

#[test]
fn add_is_two() {
    assert_eq!(add(), 2);
}

#[test]
fn is_user() {
    let user = User {
        login: "Hello".to_string(),
        id: 2,
    };
    assert_eq!(user.login, "Hello");
    match thing() {
        Ok(()) => assert!(true),
        Err(_) => panic!(),
    };
}
