<script lang="ts">
	export let name: string;
  const wasmExports = import('../Cargo.toml').then((wasm) => wasm.default());
  function wasmGreet() {
    wasmExports.then((exports) => {
      exports.greet('PPLINK');
    })
  }

  const fibonacciPromise = wasmExports.then((wasm) => wasm.fibonacci);

  let fibIndex = 1;
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
  <button on:click={wasmGreet}>Greet</button>
  {#await fibonacciPromise then fibonacci}
    <ol>
      {#each Array(fibIndex).fill(null) as _, index}
        <li>{ fibonacci(index + 1) }</li>
      {/each}
    </ol>
    <button on:click={() => fibIndex += 1}>Show next</button>
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
