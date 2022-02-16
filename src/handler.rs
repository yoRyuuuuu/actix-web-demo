use crate::db;
use crate::errors::ServiceError;
use crate::model::Claims;
use crate::model::{Login, Session, Signup};
use crate::state::AppState;
use crate::JWT_KEY;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[post("/login")]
async fn login(form: web::Json<Login>, state: AppState) -> Result<HttpResponse, ServiceError> {
    match db::get_user_by_email(&state.pool, &form.email).await {
        Ok(user) => {
            if form.password != user.password {
                return Err(ServiceError::Unauthorized("invalid password"));
            }

            let token = generate_jwt(user.email)?;
            Ok(HttpResponse::Ok().json(Session { token }))
        }
        Err(err) => Err(err.into()),
    }
}

#[post("/signup")]
async fn signup(form: web::Json<Signup>, state: AppState) -> Result<HttpResponse, ServiceError> {
    match db::create_user(&state.pool, &form).await {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err.into()),
    }
}

#[get("/home")]
async fn home(req: HttpRequest, state: AppState) -> Result<HttpResponse, ServiceError> {
    let claims = decode_jwt(&req)?;
    match db::get_user_by_email(&state.pool, &claims.sub).await {
        Ok(user) => Ok(HttpResponse::Ok().body(format!("Hello, {}", user.name))),
        Err(err) => Err(err.into()),
    }
}

fn generate_jwt(sub: String) -> Result<String, ServiceError> {
    use chrono::{DateTime, Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    let exp: DateTime<Utc> = Utc::now() + Duration::hours(30);

    let claims = Claims {
        sub,
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&*JWT_KEY.as_bytes()),
    )?;

    Ok(token)
}

fn decode_jwt(req: &HttpRequest) -> Result<Claims, ServiceError> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| {
            let values = h.split("Bearer").collect::<Vec<&str>>();
            let token = values.get(1).map(|w| w.trim());
            token
        });

    let token = match token {
        Some(token) => token,
        None => return Err(ServiceError::Unauthorized("invalid token")),
    };

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&*JWT_KEY.as_bytes()),
        &Validation::default(),
    )?
    .claims;

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use crate::handler;
    use crate::model::{Login, Session, Signup};
    use crate::state::State;
    use actix_web::{test, web, App};
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Arc;

    #[actix_rt::test]
    async fn test_api() {
        let name = "alice";
        let email = "alice@example.com";
        let password = "alice_password";

        dotenv().expect("failed to read .env file");

        let db_url = env::var("DATABASE_URL").expect("failed to load DATABASE_URL variable");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
            .expect("failed to create postgres pool");

        let state = web::Data::new(Arc::new(State {
            pool,
        }));

        let mut app = test::init_service(
            App::new()
                .app_data(state.clone())
                .service(handler::login)
                .service(handler::signup)
                .service(handler::home),
        )
        .await;

        let signup = Signup {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&signup)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let login = Login {
            email: email.to_string(),
            password: password.to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&login)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let session: Session = test::read_body_json(resp).await;
        let req = test::TestRequest::get()
            .uri("/home")
            .header("Authorization", format!("Bearer {}", session.token))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = test::read_body(resp).await;
        assert_eq!(
            body,
            actix_web::web::Bytes::from(format!("Hello, {}", name))
        );
    }
}
