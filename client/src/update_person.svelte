<script lang="ts">
    import { constants } from "./constants"
    import { updatePersonById } from "utils/app_utils"

    const { UPDATE_RECORD } = constants

    let id: string = ""
    let newName: string = ""

    // async handlers
    const handleClick = async () => {
    const response = await updatePersonById({
            id,
            ts: Date.now(),
            name: newName,
        })
        console.log(response)
    }
</script>

<h2>Update A Person By ID</h2>

<input bind:value={id} placeholder="enter existing ID" />
<input bind:value={newName} placeholder="enter new name" />
<button on:click={handleClick}>{UPDATE_RECORD(newName)}</button>