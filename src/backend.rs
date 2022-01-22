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

#[backend]
fn stream_example() -> impl Stream<Item = u32> {
 turbocharger::async_stream::stream! {
  let mut i = 0;
  loop {
   yield i;
   i += 1;
   tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
  }
 }
}
