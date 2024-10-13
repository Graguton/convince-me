<script lang="ts">
export let flipped = false;
export let accentColor = "#fff";
</script>

<div class:flipped>
    <h1><span style="--accent-color: {accentColor}">CONVINCE</span> ME</h1>
</div>

<style lang="scss">
div {
    padding-bottom: 30vh;
}

.flipped {
    transform: rotate(180deg);
}

span {
    color: var(--accent-color, #fff);
}
</style>