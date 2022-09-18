<script lang="ts">
    //region imports
    import Blog from './Blog.svelte';
    //endregion

    //region exports
	export let name: string|null;
    //endregion

    async function handleOnClick(){
        await fetch("http://localhost:8000/Blog")
        .then(res => res.json())
        .then(data => alert(data))
        .catch(err => console.error(err))
        .finally(() => console.log("Clicked button!"));
    }
</script>

<main>
	<h1>Welcome to
		{#if name.length == 0} 
			My
		{:else}
			{name}'s Website!
		{/if}
	</h1>

    <Blog 
    title={"sample_title"}
    subtitle={"sample_subtitle"}
    date={new Date()}
    />

    <button class="btn-important" on:click={handleOnClick}>Test HTTP Request</button>
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