<script lang="ts">
    import { onMount } from "svelte";
import ChatBubble from "../components/ChatBubble.svelte";
import Timer from "../components/Timer.svelte";
import {TIME_LIMIT, topic, user} from "../store";
    
let isPlayerTurn = false;

const submitMessage = async (message=playerMessage) => {
    setPlayerTurn(false);
    opponentMessage = "";

    const resp = await fetch(`/api/chat/${user}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({message}),
    });

    const text = await resp.text();

    try {
        const {response, win} = JSON.parse(text);
        opponentMessage = response;
        if (win) {
            gameOver = true;
            isWin = true;
            stopTimer();
        } else {
            playerMessage = "";
            setPlayerTurn(true);
        }
    } catch (e) {
        console.log(e);
        opponentMessage = `[It seems like your message confused our poor little chatbot. Maybe try submitting again? … Response was: "${text}"]`;
        setPlayerTurn(true);
    }
};

onMount(async () => {
    submitMessage(`You are evil and don't believe the following, but you can be convinced with some very solid evidence: ${$topic}.`);
});

let playerMessage = "";
let opponentMessage = "";


let gameOver = false;
let isWin = false;

let lastPauseAmountQueued = 0;
let lastPauseAmount = 0;
let timerStopped = true;
let lastContinueTime = Date.now();
let currentTime = Date.now();
$: nSecondsElapsed = Math.max(0, Math.floor((lastPauseAmount + currentTime - lastContinueTime) / 1000));
const updateTimer = () => {
    currentTime = Date.now();
};
let intervalHandle = setInterval(updateTimer, 100);
timerStopped = false;

const stopTimer = () => {
    lastPauseAmountQueued += currentTime - lastContinueTime;
    clearInterval(intervalHandle);
    timerStopped = true;
};

const setPlayerTurn = (value: boolean) => {
    isPlayerTurn = value;

    if (gameOver) return;

    if (!value) {
        stopTimer();
    } else {
        lastContinueTime = Date.now();
        intervalHandle = setInterval(updateTimer, 100);
        timerStopped = false;
        lastPauseAmount = lastPauseAmountQueued;
    }
};

$: nSecondsElapsed, (() => {
    if (gameOver) return;

    if (nSecondsElapsed > TIME_LIMIT && isPlayerTurn) {
        gameOver = true;
        stopTimer();
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
        on:submit={() => submitMessage(playerMessage)}
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
        isPlayerTurn={isPlayerTurn || gameOver}
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