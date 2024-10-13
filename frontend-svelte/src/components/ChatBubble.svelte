<script lang="ts">
import { createEventDispatcher, onMount } from "svelte";
import { cubicOut } from "svelte/easing";
import { fade, slide } from "svelte/transition";
import ThinkingIndicator from "./ThinkingIndicator.svelte";
    import InputArea from "./InputArea.svelte";

const emit = createEventDispatcher<{
    submit: string,
}>();

export let isPlayer: boolean;

export let message = "";
$: charCount = message.trim().length;

export let isPlayerTurn: boolean;

const submitMessage = () => {
    emit("submit", message);
};

export let placeholder: string | undefined = undefined;
export let submitButtonText = "Send it";
export let submitButtonDisabled = false;
</script>
<!-- transition:slide={{duration: 500, easing: cubicOut, axis: "y"}} -->

<bubble-
    class:player={isPlayer}
    class:backed-off={isPlayer !== isPlayerTurn}
>
    {#if isPlayer}
        <InputArea
            bind:message
            isStatic={!isPlayerTurn}
            isPlayer={true}
            placeholder={placeholder}
        />

        <button
            disabled={!isPlayerTurn || charCount === 0 || submitButtonDisabled}
            on:click={submitMessage}
        >{submitButtonText}</button>
    {:else}
        {#if isPlayerTurn}
            <InputArea
                bind:message
                isStatic={true}
                isPlayer={false}
                placeholder="Your opponent listens intently…"
            />
        {:else}
            <ThinkingIndicator />
        {/if}
    {/if}
</bubble->

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
        // opacity: 0.6666666;
        transform: scale(0.85);
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