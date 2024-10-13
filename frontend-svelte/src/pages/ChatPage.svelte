<script lang="ts">
import ChatBubble from "../components/ChatBubble.svelte";
import Timer from "../components/Timer.svelte";
import {TIME_LIMIT, topic, user} from "../store";
    
let isPlayerTurn = true;

const submitMessage = async () => {
    isPlayerTurn = false;
    opponentMessage = "";

    const {response, win} = await (await fetch(`/api/chat/${user}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({message: playerMessage}),
    })).json();

    opponentMessage = response;

    if (win) {
        gameOver = true;
        isWin = true;
        pauseTimer();
        isPlayerTurn = false;
    } else {
        isPlayerTurn = true;
        playerMessage = "";
    }

};

let playerMessage = "";
let opponentMessage = "";


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
    clearInterval(intervalHandle);
};

$: isPlayerTurn, (() => {
    if (!isPlayerTurn) {
        pauseTimer();
    } else {
        intervalHandle = setInterval(updateTimer, 100);
    }
})();

$: nSecondsElapsed, (() => {
    if (nSecondsElapsed > TIME_LIMIT && isPlayerTurn) {
        gameOver = true;
        pauseTimer();
    }
})();

</script>

<chat-container>
    <div>
        <p>Convince your companion:</p>
        <topic->{$topic}</topic->
    </div>

    <ChatBubble
        isPlayer={true}
        isPlayerTurn={isPlayerTurn && !gameOver}
        submitButtonDisabled={gameOver}
        on:submit={submitMessage}
        bind:message={playerMessage}
    />

    {#if !gameOver}
        <Timer
            displayedTime={nSecondsElapsed}
            maxTime={TIME_LIMIT}
        />
    {:else if isWin}
        <p>Congratulations! You convinced the AI in {nSecondsElapsed} seconds.</p>
    {:else}
        <p>You weren't quite persuasive enough, sorry!</p>
    {/if}

    <ChatBubble
        isPlayer={false}
        isPlayerTurn={isPlayerTurn}
        message={opponentMessage}
    />
</chat-container>

<style lang="scss">
chat-container {
    display: grid;
    grid-template-rows: repeat(4, auto);
    align-items: baseline;
}

div {
    text-align: center;
}

p {
    margin: 0;
    padding: 0;
    text-align: center;
}

topic- {
    font-size: 1.5em;
}
</style>