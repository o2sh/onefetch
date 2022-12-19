<script lang="ts">
  export let id: string;
  export let color: string;
  export let width: number;
  export let height: number;
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class="chip"
  viewBox="0 0 100 100"
  {id}
  {width}
  {height}
  fill={color}
  stroke="black"
  stroke-width="5">
  <circle cx="50" cy="50" r="45" />
</svg>
