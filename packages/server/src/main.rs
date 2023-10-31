mod server;

use bits_graphql::Client;
use bits_graphql::Database;
use server::Server;
use std::env;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let addr = "0.0.0.0:8000".parse().unwrap();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL environment variable not set");

  let connection = Database::connect(&database_url)
    .await
    .expect("Fail to initialize database connection");

  let client = Client::default().connection(connection.clone());

  let schema = bits_graphql::schema(client)
    .finish()
    .expect("Fail to initialize GraphQL schema");

  let router = server::app(schema);

  println!("GraphiQL IDE: http://{addr}/graphql");

  Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .expect("Fail to start web server");
}
