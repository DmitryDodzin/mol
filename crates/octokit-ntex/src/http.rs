use std::net::ToSocketAddrs;

use ntex::{
  http,
  web::{self, middleware, App, HttpServer},
};

use crate::octokit::{octokit_route, Octokit, OctokitConfig};

#[web::get("/")]
async fn no_params() -> &'static str {
  "Hi and Welcome to octokit-ntex bot =]\r\n"
}

pub async fn listen<T, P>(addr: P, octokit: T, config: OctokitConfig) -> std::io::Result<()>
where
  T: Octokit + Send + Sync + 'static,
  P: ToSocketAddrs + Send + Sync + 'static,
{
  std::env::set_var("RUST_LOG", "ntex=info");
  env_logger::init();

  let octokit = web::types::Data::new(octokit);
  let octokit_config = web::types::Data::new(config);

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .app_data(octokit.clone())
      .app_data(octokit_config.clone())
      .service(web::resource("/callback").route(web::post().to(octokit_route::<T>)))
      .service(no_params)
  })
  .bind(addr)?
  .workers(4)
  .keep_alive(http::KeepAlive::Disabled)
  .run()
  .await
}
