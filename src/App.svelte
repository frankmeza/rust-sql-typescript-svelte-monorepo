<script lang="ts">
    import { ws } from "./ws_client"
    import { Person } from "./core"
    import { getPeople } from "utils/app_utils"
    import { mailbox } from "./stores"

    // constants
    const PEOPLE_DATABASE = "People Database"
    const BUTTON_TEXT = "GET /people"

    // variables
    let people: Array<Person> = []
    let messages = mailbox
    debugger

    // async handlers
    const handleClick = async () => {
        people = await getPeople()
        const whatItIs = { hellaLit: true }

        ws.send(JSON.stringify(whatItIs))
    }

    // sync utils
    const formatListItem = (p: Person): string => {
        return `{ id: ${p.id}, name: ${p.name} }`
    }
</script>

<h1>{PEOPLE_DATABASE}</h1>

<!-- {#if messages.length > 0} -->
    <!-- <p>{$mailbox}</p> -->
<!-- {/if} -->

<button on:click={handleClick}>{BUTTON_TEXT}</button>

{#if people.length === 0}
    <p>ain't nobody here yet</p>
{/if}

{#if people.length > 0}
    <ul>
    {#each people as person}
        <li>{formatListItem(person)}</li>
    {/each}
    </ul>
{/if}

<style>
    h1 {
        color: brown;
        font-family: Hasklig;
    }
</style>
