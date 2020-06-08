use actix_web::{
    App,
    test::{read_body_json, read_body, init_service, TestRequest}
};

use crate::{db::{models::User, establish_connection}, services::user::init_routes};

#[actix_rt::test]
async fn create_user_from_api() {
    let test_email   = "test@email.com";
    let test_phone   = "123456789";
    let request_body = json!({ "email": test_email, "phone": test_phone });
    let conn_pool    = establish_connection();
    let mut app      = init_service(App::new().data(conn_pool.clone()).configure(init_routes)).await;

    let resp = TestRequest::post()
      .uri("/users")
      .set_json(&request_body)
      .send_request(&mut app)
      .await;

    assert!(resp.status().is_success(), "Failed to create user");

    let user: User = read_body_json(resp).await;

    assert_eq!(user.email.unwrap(), test_email);
    assert_eq!(user.phone.unwrap(), test_phone);
}

#[actix_rt::test]
async fn get_user_from_api_by_id() {
    let test_email   = "test@email.com";
    let test_phone   = "123456789";
    let request_body = json!({ "email": test_email, "phone": test_phone });
    let conn_pool    = establish_connection();
    let mut app      = init_service(App::new().data(conn_pool.clone()).configure(init_routes)).await;

    let create_resp = TestRequest::post()
      .uri("/users")
      .set_json(&request_body)
      .send_request(&mut app)
      .await;

    assert!(create_resp.status().is_success(), "Failed to create user");

    let created_user: User = read_body_json(create_resp).await;
    println!("/users/{}", created_user.id);
    
    let resp = TestRequest::get()
      .uri(format!("/users/{}", created_user.id).as_str())
      .send_request(&mut app)
      .await;

    assert!(resp.status().is_success(), "Failed to get user");

    let retrieved_user: User = read_body_json(resp).await;

    assert_eq!(created_user.id, retrieved_user.id);
}

#[actix_rt::test]
async fn list_users_from_api() {
    let test_email   = "test@email.com";
    let test_phone   = "123456789";
    let request_body = json!({ "email": test_email, "phone": test_phone });
    let conn_pool    = establish_connection();
    let mut app      = init_service(App::new().data(conn_pool.clone()).configure(init_routes)).await;

    let mut list_resp = TestRequest::get().uri("/users").send_request(&mut app).await;
    
    assert!(list_resp.status().is_success(), "Failed to list users");

    let mut body = read_body(list_resp).await;  
    let mut retrieved_users: Vec<User> = serde_json::from_slice::<Vec<User>>(&body).unwrap();

    assert_eq!(retrieved_users.len(), 0);

    let create_resp = TestRequest::post()
      .uri("/users")
      .set_json(&request_body)
      .send_request(&mut app)
      .await;

    assert!(create_resp.status().is_success(), "Failed to create user");
    
    list_resp = TestRequest::get().uri("/users").send_request(&mut app).await;

    assert!(list_resp.status().is_success(), "Failed to list users");

    body = read_body(list_resp).await;    
    retrieved_users = serde_json::from_slice::<Vec<User>>(&body).unwrap();

    assert_eq!(retrieved_users.len(), 1);
}