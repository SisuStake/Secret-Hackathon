<script>
  import { page } from '$app/stores'
</script>

<style>
  div {
    display: flex;
    flex-flow: column nowrap;
    flex-shrink: 0;
    padding: 16px;
    position: absolute;
    width: 240px;
    min-height: 100vh;
    border-right: solid 1px rgb(0, 0, 0);
  }
</style>

<div>
  <a href="/" class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-decoration-none"><span>Defi Dashboard</span></a>
  <hr>
  <ul class="nav nav-pills flex-column mb-auto">
    <li class="nav-item"><a href="/mint-nft" class="nav-link" class:active={$page.url.pathname === '/mint-nft'}>Mint NFT</a></li>
    <li class="nav-item"><a href="/wrap-lp" class="nav-link" class:active={$page.url.pathname === '/wrap-lp'}>Wrap LP</a></li>
    <li class="nav-item"><a href="/portfolio" class="nav-link" class:active={$page.url.pathname === '/portfolio'}>Portfolio</a></li>
  </ul>
</div>
