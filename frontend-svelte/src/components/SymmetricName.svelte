<script lang="ts">
import thoughtBubble from "../assets/thought-bubble.svg";
import {currentStage, Stage} from "../store";

export let flipped = false;
export let accentColor = "#fff";
</script>


<div
    class:flipped
    class:small={$currentStage !== Stage.Introduction}
>
    <img src={thoughtBubble} alt="" />

    <h1><span style="--accent-color: {accentColor}">CONVINCE</span> ME</h1>
</div>

<style lang="scss">
div {
    display: grid;
    place-items: center;
    position: absolute;

    transition: transform .3s cubic-bezier(0.215, 0.610, 0.355, 1),
        top .5s cubic-bezier(0.215, 0.610, 0.355, 1),
        bottom .5s cubic-bezier(0.215, 0.610, 0.355, 1);

    --offset: 10vh;

    &:not(.flipped) {
        top: var(--offset);
    }
    &.flipped {
        transform: rotate(180deg);
        bottom: var(--offset);
    }

    &.small {
        transform: scale(50%);
        --offset: 0;

        &.flipped {
            transform: scale(50%) rotate(180deg);
        }
    }

    > * {
        grid-area: 1/1;
    }

    > img {
        mask: linear-gradient(#0000, #0000007f);
    }

    > h1 {
        z-index: 1;
    }
}


span {
    color: var(--accent-color, #fff);
}
</style>