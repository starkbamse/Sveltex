<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'

  async function doPost () {
		
		const res = await fetch('http://localhost:8000/set-data', {
			method: 'POST',
      headers: {
        "Content-Type": "application/json",
      },
			body: JSON.stringify({
				"query":{
          "request_type":"set_site_name",
          "request_query":"Sveltex"
        },
				"credentials":{
          "username":"admin",
          "password":"123"
        }
			})
		})
    console.log(res)
    console.log(res.json())

	}

  async function getSiteName(){
    		
		const res = await fetch('http://localhost:8000/get-data', {
			method: 'POST',
      headers: {
        "Content-Type": "application/json",
      },
			body: JSON.stringify({
				"query":{
          "request_type":"get_site_name",
          "request_query":""
        },
				"credentials":{
          "username":"admin",
          "password":"123"
        }
			})
		})
    console.log(res)
    console.log(res.json())
  }

  getSiteName()
</script>

<main>
  <div>
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank" rel="noreferrer">SvelteKit</a>, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
