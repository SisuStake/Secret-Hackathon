<svelte:head>
  <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-KK94CHFLLe+nY2dmCWGMq91rCGa5gtU4mk92HdvYe+M/SXH301p5ILy+dN9+nJOZ" crossorigin="anonymous">
</svelte:head>

<script>
  import Sidebar from '../lib/components/sidebar/sidebar.svelte'
</script>

<style type="text/css">
  main {
    margin-left: 240px;
    padding: 16px;
  }
</style>

<Sidebar/>

<main>
  <slot></slot>
</main>
