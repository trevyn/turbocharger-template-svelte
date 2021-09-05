use turbocharger::{backend, server_only};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use clap::Clap;
#[cfg(not(target_arch = "wasm32"))]
use turbosql::{select, Turbosql};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clap)]
struct Opts {
 #[clap(short, long)]
 cert_path: Option<String>,
 #[clap(short, long)]
 key_path: Option<String>,
 #[clap(short, long, default_value = "8080")]
 port: u16,
}

#[backend]
#[cfg_attr(not(target_arch = "wasm32"), derive(Turbosql))]
pub struct Person {
 pub rowid: Option<i64>,
 pub name: Option<String>,
}

#[backend]
async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
 p.insert() // returns rowid
}

#[backend]
async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
 select!(Person "WHERE rowid = ?", rowid)
}

#[server_only]
#[tokio::main]
async fn main() {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();
 let opts = Opts::parse();

 log::warn!("warn enabled");
 log::info!("info enabled");
 log::debug!("debug enabled");
 log::trace!("trace enabled");

 match (opts.key_path, opts.cert_path) {
  (Some(key_path), Some(cert_path)) => {
   eprintln!("Serving HTTPS on port {}", opts.port);
   warp::serve(turbocharger::warp_routes(Frontend))
    .tls()
    .cert_path(cert_path)
    .key_path(key_path)
    .run(([0, 0, 0, 0], opts.port))
    .await;
  }
  (None, None) => {
   eprintln!("Serving (unsecured) HTTP on port {}", opts.port);
   opener::open(format!("http://127.0.0.1:{}", opts.port)).ok();
   warp::serve(turbocharger::warp_routes(Frontend)).run(([0, 0, 0, 0], opts.port)).await;
  }
  _ => eprintln!("Both key-path and cert-path must be specified for HTTPS."),
 }
}
