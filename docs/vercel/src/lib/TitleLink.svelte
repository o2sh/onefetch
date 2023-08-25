<script lang="ts">
  export let name: string;

  $: slug = name
    .toLowerCase()
    .replace(/ /g, '-')
    .replace(/#/g, '-sharp')
    .replace(/\+/g, '-plus')
    .replace(/[^a-z0-9-]/g, '');
</script>

<h3 id={slug}><a href="#{slug}" class="title-link">{name}</a></h3>

<style>
  .title-link,
  .title-link:hover,
  .title-link:visited {
    color: #222;
  }
</style>
