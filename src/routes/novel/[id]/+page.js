import { appState } from "../../state.svelte";

/** @type {import("./$types").PageLoad} */
export function load({ params }) {
  const novel = appState.gamesList[params.id];

  return {
    novel: {
      id: params.id,
      ...novel,
    },
  };
}
