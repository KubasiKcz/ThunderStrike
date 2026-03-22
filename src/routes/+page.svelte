<main>
  <header>
    <h1>ThunderStrike</h1>
    <p>
      This is the first commit, i got lazy...
    </p>
  </header>
  <aside>
    <img id="logo" src="/interface/logo.png" alt="Logo" />
  </aside>
  <section>
    <h2>Features</h2>
    <ul>
      <li>nothin so far</li>
    </ul>
  </section>
</main>

<style>
  main {
    display: grid;
    grid-template-columns: 200px 1fr;
    grid-template-rows: auto 1fr;
    grid-template-areas:
      "aside header"
      "aside section";
    height: auto;
    gap: 0;
    padding: 0;
    box-sizing: border-box;
  }

  header {
    grid-area: header;
  }

  aside {
    grid-area: aside;
    display: flex;
    flex-direction: column;
    width: auto;
  }

  section {
    grid-area: section;
  }

  header,
  aside,
  section {
    border: 1px solid #ccc;
    border-radius: 0;
    padding: 0;
  }

  #logo {
    width: 100px;
    height: 100px;
  }
</style>
