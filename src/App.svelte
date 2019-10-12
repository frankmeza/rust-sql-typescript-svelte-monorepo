<!-- JAVASCRIPT LOGIC -->
<script lang="ts">
    const NEW_ITEM = "Adding a new item: "
    const SUBMISSION = "submit new item"

    const handleSubmit = () => {
        todoItems = [...todoItems, newItem]
        newItem = ""
    }

    let todoItems: ReadonlyArray<string> = [
        "go for a walk",
        "write a todo app using svelte",
    ]

    let newItem: string = ""
</script>

<!-- HTML MARKUP -->
<h1>Todo App</h1>

<ul>
    {#each todoItems as item}
        <li>{item}</li>
    {/each}
</ul>

<h3>{NEW_ITEM} {newItem}</h3>
<textarea bind:value={newItem}></textarea>
<button on:click={handleSubmit}>{SUBMISSION}</button>

<!-- CSS STYLES -->
<style>
</style>
