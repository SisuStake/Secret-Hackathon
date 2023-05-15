<script>
  export let id
  export let name
  export let coins
</script>

<select {name} {id}>
  {#each coins as coin}
  <option value={coin.value}>{coin.symbol}</option>
  {/each}
</select>
