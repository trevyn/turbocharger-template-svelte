use tracked::tracked;
use turbocharger::backend;
use turbosql::Turbosql;

#[backend(js)]
#[derive(Turbosql, Default)]
pub struct Person {
 pub rowid: Option<i64>,
 pub name: Option<String>,
}

#[backend(js)]
#[tracked]
async fn insert_person(p: Person) -> Result<i64, tracked::StringError> {
 Ok(p.insert()?) // returns rowid
}

#[tracked]
#[backend(js)]
async fn get_person(rowid: i64) -> Result<Person, tracked::StringError> {
 Ok(turbosql::select!(Person "WHERE rowid = ?", rowid)?)
}

// N.B. Streams are experimental!

#[backend(js)]
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

#[backend(js)]
#[tracked]
fn stream_example_result() -> impl Stream<Item = Result<String, tracked::StringError>> {
 turbocharger::async_stream::try_stream!({
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
 })
}
