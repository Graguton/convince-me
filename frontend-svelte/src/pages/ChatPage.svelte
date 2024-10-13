<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";
import Timer from "../components/Timer.svelte";
import {topic} from "../store";
    
let isPlayerTurn = false;

const submitMessage = () => {
    isPlayerTurn = false;
};

let lastPauseAmount = 0;
let lastPauseTime = Date.now();
let currentTime = Date.now();
$: nSecondsElapsed = isPlayerTurn
    ? Math.floor((lastPauseAmount + currentTime - lastPauseTime) / 1000)
    : lastPauseAmount;
const updateTime = () => {
    currentTime = Date.now();
    requestAnimationFrame(updateTime);
};
updateTime();

const pauseTimer = () => {
    lastPauseAmount = nSecondsElapsed;
    lastPauseTime = Date.now();
}
</script>

<chat-container>
    <div>
        <p>Convince your companion:</p>
        <topic->{$topic}</topic->
    </div>

    <ChatBubble
        isPlayer={true}
        isPlayerTurn={isPlayerTurn}
        on:submit={submitMessage}
    />

    <Timer displayedTime={nSecondsElapsed} />

    <ChatBubble
        isPlayer={false}
        isPlayerTurn={isPlayerTurn}
    />
</chat-container>

<style lang="scss">
chat-container {
    display: grid;
    grid-template-rows: 1fr 1fr auto 1fr 1fr;
    align-items: baseline;
    text-align: center;
}

p {
    margin: 0;
    padding: 0;
}

topic- {
    font-size: 1.5em;
}
</style>