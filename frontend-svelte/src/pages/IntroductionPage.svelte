<script lang="ts">
import {currentStage, Stage} from "../store";

const advance = () => {
    $currentStage = Stage.Topic;
};
</script>

<button on:click={advance}>Play</button>

<style lang="scss">
button {
    font-size: 2rem;
}
</style>