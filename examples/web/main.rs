use reqwest::{Client, Request};
use tower::{ServiceBuilder, ServiceExt};
use tower_crawl::exclude::store::Store;
use tower_crawl::ServiceBuilderExt;

#[tokio::main]
async fn main() {
    // let exclude_policy = Except::builder()
    //     .allow("/images/*")
    //     .disallow("/users/*")
    //     .build();

    // let exclude_policy =StrictPolicy::default();

    let mut service = ServiceBuilder::default()
        .exclude_pages(|_, x| x, Store::default())
        .include_pages(42, Store::default())
        .service(Client::default());

    service.ready();
}
