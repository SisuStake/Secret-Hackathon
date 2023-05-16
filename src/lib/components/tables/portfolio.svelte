<script>
  import PortfolioTableHeader from './portfolio-header.svelte'
  import PortfolioTableRow from './portfolio-row.svelte'

  let data = [
    {
      tokensProvided: 'SILK + CMST',
      liquidity: '$451.58k',
      volume: '$51.99K',
      apy: '70.6%',
      value: '$113,641.03',
      totalFees: '0.14%',
    },
    {
      tokensProvided: 'SILK + USDT',
      liquidity: '$738.01k',
      volume: '$45,350',
      apy: '27.7%',
      value: '$577,747.15',
      totalFees: '0.14%',
    },
    {
      tokensProvided: 'SILK + USDC',
      liquidity: '$1.04M',
      volume: '$233.08k',
      apy: '19.6%',
      value: '$231,738.38',
      totalFees: '0.37%',
    },
    {
      tokensProvided: 'saUSDC + saUSDT',
      liquidity: '$171.20k',
      volume: '$1,254.87',
      apy: '30.07%',
      value: '$171,188.69',
      totalFees: '$3.76',
    },
  ]
</script>

<table id="portfolio-table" class="table table-bordered">
  <PortfolioTableHeader />

  <tbody>
    {#each data as row}
      <PortfolioTableRow data={row} />
    {/each}
  </tbody>

</table>
