use std::env;
use std::error::Error;
use std::sync::Arc;

use warp::Filter;

use identity::domain::user::UserRepository;

use identity::infrastructure::persistence::inmem::InMemUserRepository;

struct Context {
    // auth_serv: Arc<AuthService>,
    // event_pub: Arc<InMemEventBus<'static>>,
    user_repo: Arc<InMemUserRepository>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port: u16 = match env::var("PORT") {
        Ok(port) => match port.parse() {
            Ok(port) => port,
            _ => 80,
        },
        _ => 80,
    };

    let user_repo = Arc::new(InMemUserRepository::new());
    // let token_enc = Arc::new(FakeTokenEncoder::new());
    // let token_repo = Arc::new(InMemTokenRepository::new());
    // let token_serv = Arc::new(TokenService::new(token_enc.clone(), token_repo.clone()));
    // let password_hasher = Arc::new(FakePasswordHasher::new());
    // let auth_serv = Arc::new(AuthService::new(
    //     user_repo.clone(),
    //     token_serv.clone(),
    //     password_hasher.clone(),
    // ));
    // let event_pub = Arc::new(InMemEventBus::new());

    let ctx = Arc::new(Context { user_repo });

    let register = warp::path("register").and(warp::post()).map(move || {
        let ctx = ctx.clone();
        let id = ctx.user_repo.next_id().unwrap();
        format!("id: {}", id)
    });

    let health = warp::path::end().map(|| {
        println!("health");
        "health"
    });

    let user = warp::path("user").and(register);

    let routes = warp::path("api").and(health.or(user));

    println!("Listening on {}", port);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
