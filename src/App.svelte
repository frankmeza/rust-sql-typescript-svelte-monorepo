<!-- JAVASCRIPT LOGIC -->
<script lang="ts">
    let NEW_ITEM = "Adding a new item: "
    let SUBMISSION = "submit new item"

    let handleSubmit = () => {
        todoItems = [...todoItems, newItem]
        newItem = ""
        getFromServer()
    }

    let todoItems: ReadonlyArray<string> = [
        "go for a walk",
        "write a todo app using svelte",
    ]

    let newItem: string = ""

    let list: string = ""

    async function getFromServer() {
        let res = await fetch("http://localhost:8088/again")
        let json = await res.json()

        list = json
    }
</script>

<!-- HTML MARKUP -->
<h1>Todo App</h1>

<ul>
    {#each todoItems as item}
        <li>{item}</li>
    {/each}
</ul>

<p>{JSON.stringify(list)}</p>

<h3>{NEW_ITEM} {newItem}</h3>
<textarea bind:value={newItem}></textarea>
<button on:click={handleSubmit}>{SUBMISSION}</button>

<!-- <p>{list}</p> -->

<!-- CSS STYLES -->
<style>
</style>
