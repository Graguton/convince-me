import { writable } from "svelte/store";
import IntroductionPage from "./pages/IntroductionPage.svelte";

export enum Stage {
    Introduction,
    Topic,
    Chat,
    Results,
}

export const TIME_LIMIT = 300;

export const currentStage = writable(Stage.Introduction);
export const topic = writable("");