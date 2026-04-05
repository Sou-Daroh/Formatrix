<script lang="ts">
  let greeting = "Formatrix";
</script>

<main>
  <h1>{greeting}</h1>
  <p>File conversion, processed locally.</p>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
  }
  h1 {
    font-family: "DM Sans", sans-serif;
    color: var(--text);
    font-size: 2rem;
  }
  p {
    font-family: "IBM Plex Mono", monospace;
    color: var(--text-2);
  }
</style>
