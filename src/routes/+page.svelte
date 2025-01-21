<script>
    import { invoke } from "@tauri-apps/api/core";
    console.log(invoke("fetch_vn_info", { key: "steins" }));
</script>

<main class="container">
    <h1>明日、クフタさんは同性間結婚してます</h1>
</main>

<style>
    h1 {
        background: var(--main-mauve);
    }
</style>
