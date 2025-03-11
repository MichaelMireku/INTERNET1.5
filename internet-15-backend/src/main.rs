mod routes;

use warp::Filter;
use routes::get_routes;

#[tokio::main]  // ✅ This will now work because we enabled full Tokio features
async fn main() {
    let routes = get_routes();
    println!("🚀 Server running on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
