<script lang="ts">
import { cubicOut } from "svelte/easing";
import { slide } from "svelte/transition";

export let isPlayer: boolean;

let message = "";
$: charCount = message.trim().length;

let isPlayerTurn = true;

let playerInputArea: HTMLElement | null = null;
</script>

<bubble-
    transition:slide={{duration: 500, easing: cubicOut, axis: "y"}}
    class:player={isPlayer}
>
    {#if isPlayer}
        {#if charCount === 0}
            <placeholder->Let your thoughts be free!</placeholder->
        {/if}
        <input-area
            contenteditable
            bind:innerText={message}
            bind:this={playerInputArea}
        ></input-area>

        <button disabled={!isPlayerTurn}>Send it</button>
    {:else}
        {#if charCount === 0}
            <placeholder->Your opponent listens intently…</placeholder->
        {:else}
            <input-area>{message}</input-area>
        {/if}
    {/if}
</bubble->

<style lang="scss">
bubble- {
    background: #fff;
    padding: 1rem;

    border-radius: 3rem / 2rem;
    color: var(--col-orange-dark);

    display: flex;
    flex-direction: column;

    &.player {
        color: var(--col-green-dark);
    }

    > * {
        padding: 0.5em;
    }

    > button {
        align-self: center;
    }

    &.player > placeholder- {
        position: absolute;
    }
}

placeholder- {
    opacity: 0.3;
    user-select: none;
    pointer-events: none;
}

input-area {
    display: block;
    width: 40ch;
    word-break: break-word;

    outline-color: var(--col-green-dark);
}
</style>