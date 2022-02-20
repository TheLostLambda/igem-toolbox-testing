<script lang="ts">
  import wasm from "../wasm-syn-zeug/Cargo.toml";
  let dna = "";
</script>

<svelte:head>
  <base href="https://thelostlambda.github.io/igem-toolbox-testing/">
</svelte:head>

<main>
  <h1>Count Bases</h1>
  <input bind:value={dna} />
  <!-- Ngl to you dawg, this is unbelievably fucked -->
  {#await wasm( { serverPath: "build/assets/" } ).then( (w) => w.count_bases(dna) ) then count}
    <p>{count}</p>
  {/await}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
