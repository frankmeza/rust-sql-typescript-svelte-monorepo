<script lang="ts">
    import { ws } from "./ws_client"
    import { Person } from "./core"
    import { mailboxStore } from "./stores"
    import { getPeople } from "utils/app_utils"
    import AddPerson from "./add_person.svelte"
    // constants
    const nil: null = null // the ts linter throws a first false error on typings
    const PEOPLE_DATABASE: string = "People Database"
    const BUTTON_TEXT: string = "GET /people"
    const RESET: string = "reset"

    // variables
    let people: Person[] = []

    // async handlers
    const handleClick = async () => {
        people = await getPeople()

        // throwaway at some point
        const whatItIs = { hellaLit: true }
        mailboxStore.addMsg(JSON.stringify(people))
        ws.send(JSON.stringify(whatItIs))
    }

    const handleClickReset = () => {
        mailboxStore.reset()
    }

    // sync utils
    const formatListItem = (p: Person): string => {
        return `{ id: ${p.id}, name: ${p.name} }`
    }
</script>

<AddPerson/>

<h1>{PEOPLE_DATABASE}</h1>

<p>{$mailboxStore.messages}</p>

<button on:click={handleClick}>{BUTTON_TEXT}</button>
<button on:click={handleClickReset}>{RESET}</button>


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
