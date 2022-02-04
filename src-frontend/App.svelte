<script>
 import * as backend from "./turbocharger_generated";

 (async () => {
  let person = Object.assign(new backend.Person(), { name: "Bob" });

  // let person2 = new backend.Person({ name: "bob" });

  let rowid = await backend.insert_person(person);
  console.log("Inserted rowid ", rowid);
 })();

 let stream_example_result = backend.stream_example_result();
 let stream_example = backend.stream_example();
</script>

Hello!<br />

stream_example -> {$stream_example}<br />

stream_example_result -> {#await $stream_example_result then result}
 {result}
{:catch error}
 Error: {error}
{/await}<br />

{#await backend.get_person(1n) then person}
 Name: {person.name}
{:catch error}
 Error: {error}
{/await}
