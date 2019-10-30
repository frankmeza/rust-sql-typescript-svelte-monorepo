<script lang="ts">
    import { Person } from "./core"
    import { getPeople } from "./app_utils"

    let people: Array<Person> = []

    const handleClick = async () => {
        people = await getPeople()
    }
</script>

<h1>People Database</h1>

<button on:click={handleClick}>GET /people</button>

<ul>
  {#each people as person}
    <li> id: {person.id}, name: {person.name}</li>
  {/each}
</ul>

<style></style>
