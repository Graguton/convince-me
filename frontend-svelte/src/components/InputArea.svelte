<script lang="ts">
import { onMount } from "svelte";

export let message = "";
$: charCount = message.trim().length;

export let isPlayer: boolean;
export let isStatic = false;

let playerInputArea: HTMLElement | null = null;
$: playerInputArea, (() => {
    if (playerInputArea !== null) {
        playerInputArea.focus();
    }
})();

export let placeholder = "Let your thoughts be free!";
</script>


{#if !isStatic}
    {#if charCount === 0}
        <placeholder- class:is-player={isPlayer}>{placeholder}</placeholder->
    {/if}
    <input-area
        contenteditable
        bind:innerText={message}
        bind:this={playerInputArea}
    ></input-area>
{:else}
    <input-area>{message}</input-area>
{/if}

<style lang="scss">
bubble- {
    background: #fff;
    padding: 1rem;

    min-height: 2em;

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
}

placeholder- {
    opacity: 0.3;
    user-select: none;
    pointer-events: none;
    padding: 0.5rem;

    &.is-player {
        position: absolute;
    }
}

input-area {
    display: block;
    width: 40ch;
    word-break: break-word;
    padding: 0.5rem;

    outline-color: var(--col-green-dark);
}
</style>