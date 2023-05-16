<script>
  import CoinInput from '../inputs/coin-input.svelte'
  import NFTPlaceholder from '../inputs/nft-placeholder.svelte'
  import Submit from '../buttons/submit.svelte'

  let coins = [
    { value: 'scrt', symbol: 'SCRT' },
    { value: 'stkd-scrt', symbol: 'stkd-SCRT' },
    { value: 'btc', symbol: 'btc' },
  ]
</script>

<style>
  form {
    border: 1px solid rgb(0, 0, 0);
    border-radius: 8px;
    display: inline-block;
    padding: 16px;
    text-align: center;
  }
</style>

<form action="#" id="mint-nft-form">
  <h1>Mint NFT</h1>
  <CoinInput id='coin-1-input' coins={coins} />

  <NFTPlaceholder />

  <Submit value='Mint NFT'/>
</form>
