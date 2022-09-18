<script lang="ts">
    //region exports
	export let title: string;
    export let subtitle: string;
    export let date: Date;
    //endregion
</script>

<main>
    <h3 class="fs-md">{title}</h3>
    <h4 class="fs-sm lead">{subtitle}</h4>
    <p>{date}</p>

    <div>
        Main text goes here.
    </div>
</main>