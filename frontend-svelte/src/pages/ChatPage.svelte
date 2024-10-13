<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";

    
let currentMesssage = "";
let opponentMessage = "";

let isCurrentTurn = true;
</script>

<chat-container>
    <ChatBubble isPlayer={true} />

    <ChatBubble isPlayer={false} />
</chat-container>

<style lang="scss">
chat-container {
    display: grid;
    grid-template-rows: 1fr 1fr;
    align-items: baseline;
    gap: 3rem;
}
</style>