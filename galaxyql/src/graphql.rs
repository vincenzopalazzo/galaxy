//! GraphQL Server Implementation.
use juniper_rocket::graphiql_source;
use rocket::response::content::RawHtml;

#[rocket::get("/")]
fn graphql_home() -> RawHtml<String> {
    graphiql_source("/graphql", None)
}

pub async fn lauch_rocket() {
    let _ = rocket::build()
        .mount("/", rocket::routes![graphql_home])
        .launch()
        .await
        .expect("server to launch");
}
