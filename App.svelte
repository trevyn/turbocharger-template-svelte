<script>
    import * as backend from "./turbocharger_generated";
    import { onMount } from "svelte";

    onMount(async () => {
        let person = Object.assign(new backend.Person(), { name: "Bob" });
        let rowid = await backend.insert_person(person);
        console.log("Inserted rowid ", rowid);
    });

    let stream_example_result = backend.stream_example_result();
    let stream_example = backend.stream_example();
</script>

<div class="w-ful min-h-screen flex flex-col items-center justify-center">
    <h1 class="text-indigo-500 text-3xl font-bold mb-6">
        Turbocharger Example!
    </h1>
    <div class="pre font-mono bg-gray-200 p-4 w-full max-w-prose rounded">
        stream_example → {$stream_example}<br />

        stream_example_result → {#await $stream_example_result then result}
            {result}
        {:catch error}
            <span class="font-bold">Error:</span>
            <span class="text-red-500 font-bold">
                {error}
            </span>
        {/await}<br />

        {#await backend.get_person(1n) then person}
            Name: {person.name}
        {:catch error}
            <span class="font-bold">Error:</span>
            <span class="text-red-500 font-bold">
                {error}
            </span>
        {/await}
    </div>
</div>
