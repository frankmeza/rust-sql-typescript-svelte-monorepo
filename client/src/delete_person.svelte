<script lang="ts">
    import { ws } from "./ws_client"
    import { deletePersonById } from "utils/app_utils"
    import { constants } from "./constants"
    // constants
    const { DELETE_RECORD } = constants

    // variables
    let id: string = ""
    let response: string = ""

    // async handlers
    const handleClick = async () => {
        response = await deletePersonById(id)
        console.log(response)
    }
</script>

<h2>Delete A Person</h2>

<input bind:value={id} placeholder="enter id to delete" />
<button on:click={handleClick}>{DELETE_RECORD(id)}</button>

