<script>
 import * as backend from "./turbocharger_generated";

 (async () => {
  let person = Object.assign(new backend.Person(), { name: "Bob" });

  // let person2 = new backend.Person({ name: "bob" });

  let rowid = await backend.insert_person(person);
  console.log("Inserted rowid ", rowid);
 })();

 let stream_example_store = backend.stream_example();
</script>

Hello!<br />

stream_example_store -> {$stream_example_store}

{#await backend.get_person(1n) then person}
 Name: {person.name}
{:catch error}
 Error: {error}
{/await}
