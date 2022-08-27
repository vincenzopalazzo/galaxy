//! GraphQl Gralaxy server build to observer and enjoy querying your
//! Lightning node.
mod graphql;

use crate::graphql::lauch_rocket;

#[rocket::main]
async fn main() {
    lauch_rocket().await;
}
