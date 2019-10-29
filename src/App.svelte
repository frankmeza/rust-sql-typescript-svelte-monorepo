<script lang="ts">
    import { Person } from "./core"
    import { getPeople } from "./app_utils"

    // constants
    const BUTTON_TEXT = "GET /people"

    // variables
    let people: Array<Person> = []

    // handlers
    const handleClick = async () => {
        people = await getPeople()
    }

    // render methods
    const renderIdAndName = (p: Person): string => {
        return `{ id: ${p.id}, name: ${p.name} }`
    }
</script>

<h1>People Database</h1>

<button on:click={handleClick}>
    {BUTTON_TEXT}
</button>

<ul>
  {#each people as person}
    <li class="people" id={`${person.id}`} >{renderIdAndName(person)}</li>
  {/each}
</ul>

<style>
    .people {
        background-color: aliceblue;
    }
</style>
