<script>
    import Button from "$lib/util/Button.svelte";
    import AddGame from "$lib/AddGame.svelte";
    // import Icon from "@iconify/svelte";
    import { goto } from "$app/navigation";
</script>

<nav>
    <section id="sidebar__header">
        <h1>多</h1>
        <div id="sidebar__header__buttons">
            <Button onclick={() => goto("/")} icon="heroicons:squares-2x2" />
            <Button
                image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg"
            />
            <Button
                image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg"
            />
            <Button
                image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg"
            />
            <Button
                image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg"
            />

            <AddGame />
        </div>
    </section>

    <!-- TODO: Hidden until implemented for completeness -->
    <!-- <Icon
        icon="material-symbols:settings-outline-rounded"
        style="font-size: 24px; color: #fff; cursor:pointer;"
    /> -->
</nav>

<style>
    nav {
        height: 100%;
        padding-bottom: 1rem;
        background: #1b1b1b;
        display: flex;
        position: fixed;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        #sidebar__header {
            h1 {
                color: #5d5d5d;
                font-size: 50px;
                padding: 1rem;
            }

            #sidebar__header__buttons {
                text-align: center;
                margin-top: 2.5rem;
                /*كل ما تعطل اديها */
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }
        }
    }
</style>
