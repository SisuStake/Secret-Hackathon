<script>
  import PortfolioTableHeader from './portfolio-header.svelte'
  import PortfolioTableRow from './portfolio-row.svelte'

  let data = [
    {
      tokensProvided: '9k',
      liquidity: '2.5m',
      volume: '123.4k',
      apy: '25%',
      totalFees: '345.67k',
      total: '567.8m',
    },
    {
      tokensProvided: '11k',
      liquidity: '2.7m',
      volume: '4356.7k',
      apy: '19%',
      totalFees: '245.67k',
      total: '123.8m',
    }
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