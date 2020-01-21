<script lang="ts">
    import { ws } from "./ws_client"
    import { Person } from "./core"
    import { mailboxStore } from "./stores"

    import { constants } from "./constants"
    import { getPeople } from "utils/app_utils"

    const { GET_RECORDS, RESET } = constants

    export let people: Person[] = []

    const handleClick = async () => {
        people = await getPeople()
        // throwaway at some point
        // const whatItIs = { hellaLit: true }
        // mailboxStore.addMsg(JSON.stringify(people))
        // ws.send(JSON.stringify(whatItIs))
    }

    const handleClickReset = () => {
        mailboxStore.reset()
    }

    const formatListItem = (p: Person): string => {
        return `{ id: ${p.id}, name: ${p.name} }`
    }
</script>

<h2>{GET_RECORDS}</h2>

<p>{$mailboxStore.messages}</p>

<button on:click={handleClick}>{GET_RECORDS}</button>
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
