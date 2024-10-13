import { writable } from "svelte/store";
import IntroductionPage from "./pages/IntroductionPage.svelte";

export enum Stage {
    Introduction,
    Topic,
    Chat,
    Results,
}

export const currentStage = writable(Stage.Introduction);