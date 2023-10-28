use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Token(pub String);

#[derive(Default)]
pub struct Client {
  pub connection: DatabaseConnection,
  pub token: Option<Token>,
}

impl Client {
  pub fn new(connection: DatabaseConnection) -> Self {
    Self {
      connection,
      ..Default::default()
    }
  }

  pub fn connection(self, connection: DatabaseConnection) -> Self {
    Self { connection, ..self }
  }

  pub fn token(self, token: Token) -> Self {
    Self {
      token: Some(token),
      ..self
    }
  }
}
