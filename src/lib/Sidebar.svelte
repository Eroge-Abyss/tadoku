<script>
import Button from "$lib/util/Button.svelte";
import Icon from "@iconify/svelte";
</script>

<nav>
  <section id="sidebar__header">
    <h1>多</h1>
    <div id="sidebar__header__buttons">
      <Button icon="heroicons:squares-2x2"/>
      <Button image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg" />
      <Button image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg" />
<Button image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg" />
<Button image="https://i.pinimg.com/736x/e2/ea/a3/e2eaa30319b978175c9156ae1a2fe1e5.jpg" />

      <button id="btn__add">
        +
      </button>
    </div>
  </section>

  <Icon icon="material-symbols:settings-outline-rounded" style="font-size: 24px; color: #fff; cursor:pointer;" />
</nav>

<style>
  nav {
    height: 100%;
    padding-bottom: 1rem; 
    background: #1B1B1B;
    display: flex;
    position: fixed;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    
    #sidebar__header { 
      h1 {
        color: #5D5D5D;
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

        & #btn__add {
          border: 0;
          background: #313131;
          color: #5D5D5D;
          border: 2px solid #5D5D5D;
          height: 56px;
          width: 56px;
          font-size: 2.5rem;
          text-align: center;
          margin: auto;
          border-radius: 5px;
          cursor: pointer;
        }
      }
    }
  } 
</style>
