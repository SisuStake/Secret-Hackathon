<script>
  import CoinInput from '../inputs/coin-input.svelte'
  import SubmitButton from '../buttons/submit.svelte'
  import SwapButton from '../buttons/swap.svelte'

  let coins = [
    { value: 'scrt', symbol: 'SCRT' },
    { value: 'stkd-scrt', symbol: 'stkd-SCRT' },
    { value: 'btc', symbol: 'btc' },
  ]
</script>

<style>
  form {
    display: inline-block;
    border: 1px solid rgb(0, 0, 0);
    border-radius: 8px;
    padding: 16px;
    text-align: center;
  }

  fieldset {
    border: 1px solid rgb(0, 0, 0);
    border-radius: 8px;
    padding: 8px;
    margin-bottom: 16px ;
  }
</style>

<form action="#" id="wrap-lp-form">
  <h1>Wrap LP</h1>

  <fieldset>
    <legend>From</legend>
    <CoinInput id='from-input' coins={coins} />
  </fieldset>

  <SwapButton />

  <fieldset>
    <legend>To</legend>
    <CoinInput id='to-input' coins={coins} />
  </fieldset>

  <SubmitButton value='Wrap LP'/>
</form>
