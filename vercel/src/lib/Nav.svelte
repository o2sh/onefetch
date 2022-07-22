<script lang="ts">
  interface NavItem {
    name: string;
    href: string;
  }

  export let active: null | NavItem['name'] | NavItem['href'] = null;
  const nav: NavItem[] = [
    {
      name: "Home",
      href: "/",
    },
    {
      name: "ASCII Preview",
      href: "ascii-preview/",
    },
    {
      name: "GitHub",
      href: "https://github.com/o2sh/onefetch",
    },
  ];

  function isActive(item: NavItem): boolean {
    return active === item.name || active === item.href;
  }
</script>

<nav>
  {#each nav as item }
    <a href={item.href} class={isActive(item) ? "active" : ""}>{item.name}</a>
  {/each}
</nav>

<style>
  nav {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
    padding: 0.5rem;
    border-bottom: 1px solid #ccc;
  }

  .active {
    color: #982c61;
    border-bottom: 2px solid #4a4a4a;
  }
</style>
