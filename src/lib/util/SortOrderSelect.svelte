<script>
  import { appState } from '../../routes/state.svelte';

  // @ts-ignore
  async function handleSortOrderSelection(e) {
    await appState.setSortOrder(e.target.value);
  }
</script>

<div class="select-container">
  <label for="sort-order">Sort by:</label>
  <select
    id="sort-order"
    value={appState.sortOrder}
    onchange={handleSortOrderSelection}
  >
    <option value="title">Title</option>
    <option value="last_played">Last Played</option>
    <option value="playtime">Playtime</option>
  </select>
</div>

<style>
  .select-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .select-container label {
    font-size: 14px;
    color: var(--main-text);
  }

  .select-container select {
    background-color: var(--main-background);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--small-radius);
    padding: 0.5rem 0.75rem;
    color: var(--main-text);
    font-size: 14px;
    cursor: pointer;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='rgba(255, 255, 255, 0.6)' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.5rem center;
    padding-right: 2rem;
  }

  .select-container select:hover {
    border-color: rgba(255, 255, 255, 0.2);
  }

  .select-container select:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 1px var(--primary);
  }
</style>
