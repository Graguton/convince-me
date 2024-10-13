<script lang="ts">
import {currentStage, Stage} from "../store";
import ChatPage from "./ChatPage.svelte";

const advance = () => {
    $currentStage = Stage.Chat;
};
</script>

<button on:click={advance}>Play</button>

<style lang="scss">
button {
    font-size: 2rem;
}
</style>