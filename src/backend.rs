use turbocharger::backend;
use turbosql::Turbosql;
use wasm_bindgen::prelude::*;

#[backend]
#[derive(Turbosql)]
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
