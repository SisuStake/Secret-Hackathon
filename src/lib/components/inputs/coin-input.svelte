<script>
  import CoinTypeSelect from './coin-type-select.svelte'
  import CoinValueInput from './coin-value-input.svelte'

  export let id
  export let coins
</script>

<div {id} class="mb-3">
  <CoinTypeSelect name={id + '-value'} id={id + '-value'} {coins} />  
  <CoinValueInput name={id + '-value'} id={id + '-value'} />  
</div>
