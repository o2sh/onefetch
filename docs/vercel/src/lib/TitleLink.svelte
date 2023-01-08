<script lang="ts">
  export let name: string;
</script>

<h3 id={name}><a href="#{name}" class="title-link">{name}</a></h3>

<style>
  .title-link,
  .title-link:hover,
  .title-link:visited {
    color: #222;
  }
</style>
