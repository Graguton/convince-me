<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";
import Timer from "../components/Timer.svelte";
import {TIME_LIMIT, topic} from "../store";
    
let isPlayerTurn = true;

const submitMessage = () => {
    isPlayerTurn = false;
};

let gameOver = false;
let isWin = false;

let lastPauseAmount = 0;
let lastPauseTime = Date.now();
let currentTime = Date.now();
$: nSecondsElapsed = Math.floor(
    (isPlayerTurn
        ? lastPauseAmount + currentTime - lastPauseTime
        : lastPauseAmount
    ) / 1000
);
const updateTimer = () => {
    currentTime = Date.now();
};
let intervalHandle = setInterval(updateTimer, 100);

const pauseTimer = () => {
    lastPauseAmount = lastPauseAmount + currentTime - lastPauseTime;
    lastPauseTime = Date.now();
};

$: isPlayerTurn, (() => {
    if (!isPlayerTurn) {
        pauseTimer();
        clearInterval(intervalHandle);
    } else {
        intervalHandle = setInterval(updateTimer, 100);
    }
})();

// $: nSecondsElapsed, (() => {
//     if (nSecondsElapsed > TIME_LIMIT && isPlayerTurn) {
//         gameOver = true;
//     }
// })();

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

    {#if !gameOver}
        <Timer
            displayedTime={nSecondsElapsed}
            maxTime={TIME_LIMIT}
        />
    {:else if isWin}
        <p>Congratulations! You convinced the AI.</p>
    {:else}
        <p>Wasn't quite persuasive enough, sorry!</p>
    {/if}

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
}

div {
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