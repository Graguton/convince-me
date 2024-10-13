<script lang="ts">
import { TIME_LIMIT } from "../store";

export let displayedTime: number;
export let maxTime = TIME_LIMIT;
</script>

<timer->
    <time-label>TIME</time-label>
    <current-time>{displayedTime}</current-time>
    <divider-></divider->
    <max-time>{maxTime}</max-time>
</timer->

<style lang="scss">
timer- {
    padding: 0.5em;
    width: 4em;
    height: 4em;
    display: grid;
    place-items: center;
    margin: 1em auto;
    align-content: center;
    gap: 0.25em;

    background: #fff;
    color: var(--col-green-dark);
    border-radius: 50%;
    line-height: 1;


    font-family: var(--font-head);

    > time-label {
        font-size: 0.5em;
    }

    > current-time {
        font-size: 1.125em;
    }

    > divider- {
        background: currentColor;
        height: 2px;
        width: 100%;
    }

    > max-time {
        opacity: 0.75;
        font-size: 0.5em;
    }
}
</style>