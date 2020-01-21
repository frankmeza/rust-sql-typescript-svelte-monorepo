<script lang="ts">
  import { ws } from "./ws_client";
  import { createPerson } from "utils/app_utils";
  import { constants } from "./constants";
  // constants
  // the ts linter throws a first false error on typings
  const nil: null = null;
  const { ADD_RECORD } = constants;

  // variables
  let name: string = "";
  let response: string = "";

  // async handlers
  const handleClick = async () => {
    response = await createPerson(name);
    console.log(response);
  };
</script>

<h2>Add A Person</h2>

<input bind:value={name} placeholder="enter name" />
<button on:click={handleClick}>{ADD_RECORD(name)}</button>

<p>Hello {name || 'anonymous'}!</p>
