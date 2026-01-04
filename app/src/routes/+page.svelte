<script lang="ts">
  import { signIn } from "@auth/sveltekit/client";
  import { page } from "$app/stores";
</script>

<div class="h-[92vh] bg-gradient-to-r from-indigo-500 via-sky-500 to-emerald-500 animate-gradient-x flex flex-col items-center justify-center pb-10 gap-8">
  
  <h1 class="text-base text-center font-black text-gray-100 text-[96px] drop-shadow-md">
    Hæ nörd
  </h1>
</div>

<style>
  :global(body) {
    background-color: #111;
    margin: 0;
    font-family: sans-serif;
  }

  .animate-gradient-x {
    background-size: 200% 200%;
    animation: gradient-x 15s ease infinite;
  }
  
  @keyframes gradient-x {
    0% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
    100% { background-position: 0% 50%; }
  }
</style>
