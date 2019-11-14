<script lang="ts">
    import { ws } from "./ws_client"
    import { Person } from "./core"
    import { getPeople } from "utils/app_utils"
    // whyyyy 😭
    import { mailbox } from "./stores"
    console.log(mailbox)
    // constants
    const PEOPLE_DATABASE = "People Database"
    const BUTTON_TEXT = "GET /people"

    // variables
    let people: Array<Person> = []
    let mail: string[] = []

    // subscriptions to store
    // mailbox.subscribe(mb => {
    //     mail = mb;
    //     // mail = [...mail.messages, newMsg];
    // })

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

{@debug mail}

<h1>{PEOPLE_DATABASE}</h1>

{#if $mailbox.length > 0}
    <p>{$mailbox}</p>
{/if}

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
