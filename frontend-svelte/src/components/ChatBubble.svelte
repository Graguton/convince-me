<script lang="ts">
import { createEventDispatcher } from "svelte";
import { cubicOut } from "svelte/easing";
import { slide } from "svelte/transition";
import ThinkingIndicator from "./ThinkingIndicator.svelte";

const emit = createEventDispatcher<{
    submit: string,
}>();

export let isPlayer: boolean;

let message = "";
$: charCount = message.trim().length;

export let isPlayerTurn = true;

let playerInputArea: HTMLElement | null = null;
const submitMessage = () => {
    emit("submit", message);
};
</script>

<bubble-
    transition:slide={{duration: 500, easing: cubicOut, axis: "y"}}
    class:player={isPlayer}
    class:backed-off={isPlayer !== isPlayerTurn}
>
    {#if isPlayer}
        {#if charCount === 0}
            <placeholder->Let your thoughts be free!</placeholder->
        {/if}
        {#if isPlayerTurn}
            <input-area
                contenteditable
                bind:innerText={message}
                bind:this={playerInputArea}
            ></input-area>
        {:else}
            <input-area>{message}</input-area>
        {/if}

        <button
            disabled={!isPlayerTurn}
            on:click={submitMessage}
        >Send it</button>
    {:else}
        {#if isPlayerTurn}
            <placeholder->Your opponent listens intently…</placeholder->
        {:else}
            <ThinkingIndicator />
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

    transition: opacity .1s cubic-bezier(0.215, 0.610, 0.355, 1),
        transform .3s cubic-bezier(0.215, 0.610, 0.355, 1);

    &.player {
        color: var(--col-green-dark);
    }

    &.backed-off {
        opacity: 0.75;
        transform: scale(0.9);
        pointer-events: none;
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