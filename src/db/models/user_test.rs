
use crate::db::{establish_connection, models::User};

#[test]
fn create_user_with_phone_and_email() {
    let conn = establish_connection();
    let email = Some("test@email.com");
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();

    assert_eq!(user.email.unwrap().as_str(), email.unwrap());
    assert_eq!(user.phone.unwrap().as_str(), phone.unwrap());
}

#[test]
fn create_user_with_phone_only() {
    let conn = establish_connection();
    let email = None;
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();

    assert!(user.email.is_none());
    assert_eq!(user.phone.unwrap().as_str(), phone.unwrap());
}

#[test]
fn create_user_with_email_only() {
    let conn = establish_connection();
    let email = Some("test@email.com");
    let phone = None;

    let user = User::create(email, phone, &conn).unwrap();

    assert_eq!(user.email.unwrap().as_str(), email.unwrap());
    assert!(user.phone.is_none());
}

#[test]
fn create_user_with_existing_email() {
    let conn = establish_connection();
    let email = Some("test@email.com");
    let phone = None;

    let user = User::create(email, phone, &conn).unwrap();
    let existing_user = User::create(email, phone, &conn).unwrap();

    assert_eq!(user.id, existing_user.id);
}

#[test]
fn create_user_with_existing_phone() {
    let conn = establish_connection();
    let email = None;
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();
    let existing_user = User::create(email, phone, &conn).unwrap();

    assert_eq!(user.id, existing_user.id);
}

#[test]
fn list_users() {
    let conn = establish_connection();
    let email = None;
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();
    let existing_users = User::list(&conn);

    assert_eq!(1, existing_users.len());
    assert_eq!(user.id, existing_users[0].id);
}

#[test]
fn get_user_by_phone() {
    let conn = establish_connection();
    let email = None;
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();
    let existing_user = User::by_phone(&phone.unwrap(), &conn).unwrap();

    assert_eq!(user.id, existing_user.id);
}

#[test]
fn get_user_by_email() {
    let conn = establish_connection();
    let email = Some("test@email.com");
    let phone = None;

    let user = User::create(email, phone, &conn).unwrap();
    let existing_user = User::by_email(&email.unwrap(), &conn).unwrap();

    assert_eq!(user.id, existing_user.id);
}

#[test]
fn get_user_by_id() {
    let conn = establish_connection();
    let email = Some("test@email.com");
    let phone = Some("123456789");

    let user = User::create(email, phone, &conn).unwrap();
    let existing_user = User::by_id(&user.id, &conn).unwrap();

    assert_eq!(user.id, existing_user.id);
}
