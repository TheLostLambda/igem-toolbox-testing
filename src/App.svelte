<script lang="ts">
  export let wasm;
  let dna = "";
</script>

<main>
  <h1>Count Bases</h1>
  <input bind:value={dna}/>
  <p>{wasm.count_bases(dna)}</p>
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
