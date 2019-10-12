<!-- JAVASCRIPT LOGIC -->
<script lang="ts">
    let todoItems: ReadonlyArray<string> = [
        "go for a walk",
        "write a todo app using svelte",
    ]
</script>

<!-- HTML MARKUP -->
<h1>Todo App</h1>

<ul>
    {#each todoItems as item}
        <li>{item}</li>
    {/each}
</ul>

<!-- CSS STYLES -->
<style>
</style>
