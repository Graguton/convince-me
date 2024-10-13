<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";

    
let currentMesssage = "";
let opponentMessage = "";

let isPlayerTurn = true;

const submitMessage = () => {
    isPlayerTurn = false;
};
</script>

<chat-container>
    <ChatBubble
        isPlayer={true}
        isPlayerTurn={isPlayerTurn}
        on:submit={submitMessage}
    />

    <ChatBubble
        isPlayer={false}
        isPlayerTurn={isPlayerTurn}
    />
</chat-container>

<style lang="scss">
chat-container {
    display: grid;
    grid-template-rows: 1fr 1fr;
    align-items: baseline;
    gap: 3rem;
}
</style>