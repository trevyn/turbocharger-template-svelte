use turbocharger::backend;
use turbosql::{select, Turbosql};

#[backend]
#[derive(Turbosql, Default)]
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

// N.B. Streams are not yet supported, but this is what the interface might look like:

// #[backend]
// fn stream_example() -> impl Stream<Item = u32> {
//  async_stream::stream! {
//   let mut i = 0;
//   loop {
//    yield i;
//    i += 1;
//    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
//   }
//  }
// }

// use async_stream::stream;

// use futures_core::stream::Stream;
// use futures_util::pin_mut;
// use futures_util::stream::StreamExt;

// #[tokio::main]
// async fn main() {
//  let s = zero_to_three();
//  pin_mut!(s); // needed for iteration

//  while let Some(value) = s.next().await {
//   println!("got {}", value);
//  }
// }
