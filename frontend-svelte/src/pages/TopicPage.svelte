<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";
import { currentStage, Stage } from "../store";

const submitTopic = () => {
    $currentStage = Stage.Chat;
};
</script>

<h2>Think of a topic…</h2>
<p>You find your AI companion rather unconvinced of this fact:</p>
<ChatBubble
    isPlayer={true}
    isPlayerTurn={true}
    placeholder="What do you feel quite strongly about?"
    submitButtonText="Let’s debate"
    on:submit={submitTopic}
/>