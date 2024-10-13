<script>
import SymmetricName from "./components/SymmetricName.svelte";
import ChatPage from "./pages/ChatPage.svelte";
import IntroductionPage from "./pages/IntroductionPage.svelte";
import TopicPage from "./pages/TopicPage.svelte";
import {currentStage, Stage} from "./store";
</script>

<main>
    <main-content>
        {#if $currentStage === Stage.Introduction}
            <IntroductionPage />
        {:else if $currentStage === Stage.Topic}
            <TopicPage />
        {:else if $currentStage === Stage.Chat}
            <ChatPage />
        {/if}
    </main-content>

    <backdrop-content>
        <SymmetricName
            accentColor={"var(--col-green-dark)"}
        />
        <SymmetricName
            accentColor={"var(--col-orange-dark)"}
            flipped
        />
    </backdrop-content>
</main>

<svelte:head>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,100..700;1,100..700&family=Krona+One&display=swap" rel="stylesheet">
</svelte:head>

<style lang="scss">
main {
    width: 100%;
    height: 100%;

    display: grid;
    place-items: center;
    > * {
        grid-area: 1/1;
    }

    font-size: 1.5rem;
    color: #fff;

    background: radial-gradient(circle at center, #0000 50%, #0000002f 75%, #0000007f), linear-gradient(
        #FFA322,
        #F0BD43 30%,
        #BCCB84 60%,
        #63BD9E
    );

    > backdrop-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        pointer-events: none;
    }
}
</style>
