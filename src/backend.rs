use turbocharger::backend;
use turbosql::Turbosql;

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
 turbosql::select!(Person "WHERE rowid = ?", rowid)
}

// N.B. Streams are experimental!

// #[backend]
// async fn stream_example() -> yields!(u32) {
//  let mut i = 0;
//  loop {
//   yield i;
//   i += 1;
//   tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//  }
// }

#[backend]
fn stream_example() -> impl Stream<Item = String> {
 turbocharger::async_stream::stream! {
  let mut i = 0;
  loop {
   yield format!("e{}", i*10);
   i += 1;
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 }
}

#[backend]
#[tracked::tracked]
fn stream_example_result() -> impl Stream<Item = Result<String, tracked::Error>> {
 turbocharger::async_stream::try_stream! {
  let mut i = 0;
  loop {
   yield format!("r{}", i);
   i += 1;
   if i == 5 {
    u8::try_from(999u16)?;
    // None?;
   }
   tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  }
 }
}
