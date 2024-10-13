<script lang="ts">
import { fade } from "svelte/transition";
</script>

<div>
    <period-></period->
    <period- style="--animation-delay: 1.5s;"></period->
    <period- style="--animation-delay: 3s;"></period->
</div>

<style lang="scss">
div {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 2em;
    gap: 24px;

    > period- {
        width: 12px;
        height: 12px;

        background: currentColor;

        animation: spin 4.5s infinite cubic-bezier(0.645, 0.045, 0.355, 1);

        animation-delay: var(--animation-delay);

        --animation-delay: 0s;
    }
}

@keyframes spin {
    0% {
        transform: scale(1) rotate(0);
    }

    
    50% {
        transform: scale(2) rotate(2turn);
        opacity: 0.25;
    }
    
    
    100% {
        transform: scale(1) rotate(4turn);
    }
}
</style>