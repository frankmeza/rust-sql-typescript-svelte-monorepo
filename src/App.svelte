<script lang="ts">
    import { Person } from "./core"

    let people: Array<Person> = []

    async function getPeople() {
        let res = await fetch("http://localhost:8088/people")

        if (!res.ok) {
            console.log(res.statusText)
            return null
        }

        let json = await res.json()
        people = json
    }
</script>

<h1>People Database1</h1>

<button on:click={getPeople}>GET /people</button>

<ul>
  {#each people as person}
    <li> id: {person.id}, name: {person.name}</li>
  {/each}
</ul>

<style></style>
