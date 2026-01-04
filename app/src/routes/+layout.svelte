<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.webp';
	import { page } from "$app/stores";
	import { signIn, signOut } from "@auth/sveltekit/client";

	let { children } = $props();

	// State to control the profile dropdown
	let showUserMenu = $state(false);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="relative bg-black after:pointer-events-none after:absolute after:inset-x-0 after:bottom-0 after:h-px after:bg-white/10 h-[8vh]">
  <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
    <div class="relative flex h-16 items-center justify-between">
      <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">

        <button type="button" popovertarget="mobile-menu" class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-white/5 hover:text-white focus:outline-2 focus:-outline-offset-1 focus:outline-indigo-500">
          <span class="absolute -inset-0.5"></span>
          <span class="sr-only">Open main menu</span>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" data-slot="icon" aria-hidden="true" class="size-6 in-aria-expanded:hidden">
            <path d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" data-slot="icon" aria-hidden="true" class="size-6 not-in-aria-expanded:hidden">
            <path d="M6 18 18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
      </div>

      <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
        <div class="flex shrink-0 items-center">
          <a href="/">
            <img src={favicon} alt="nerds.is" class="h-8 w-auto" />
          </a>
        </div>
        <div class="hidden sm:ml-6 sm:block">
          <div class="flex space-x-4">
            <a href="/movies" class="rounded-md hover:bg-gray-800 px-3 py-2 text-sm font-medium text-white transition">Kvikmyndir</a>
          </div>
        </div>
      </div>

      <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
        
        {#if $page.data.session}
            <div class="flex items-center gap-4">
                <div class="hidden md:flex flex-col items-end mr-2">
                    <span class="text-sm font-medium text-white leading-none">{$page.data.session.user?.name}</span>
                </div>

                <div class="relative">
                    <button 
                        onclick={() => showUserMenu = !showUserMenu}
                        class="relative flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                    >
                        <span class="sr-only">Open user menu</span>
                        {#if $page.data.session.user?.image}
                            <img class="h-8 w-8 rounded-full border border-gray-600" src={$page.data.session.user.image} alt="" />
                        {:else}
                            <div class="h-8 w-8 rounded-full bg-indigo-500 flex items-center justify-center text-white">
                                {$page.data.session.user?.name?.charAt(0) || 'U'}
                            </div>
                        {/if}
                    </button>
                    
                    {#if showUserMenu}
                        <button 
                            class="fixed inset-0 z-10 cursor-default w-full h-full" 
                            onclick={() => showUserMenu = false} 
                            aria-label="Close menu"
                        ></button>

                        <div class="absolute right-0 z-20 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none transition-all">
                            <div class="px-4 py-2 text-xs text-gray-500 border-b">
                               {$page.data.session.user?.email}
                            </div>
                            <button 
                                onclick={() => signOut()}
                                class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                            >
                                Sign out
                            </button>
                        </div>
                    {/if}
                </div>
            </div>
        {:else}
            <button 
                onclick={() => signIn("google")}
                class="text-sm font-medium text-gray-300 hover:text-white hover:bg-white/10 px-3 py-2 rounded-md transition"
            >
                Login
            </button>
        {/if}

      </div>
    </div>
  </div>

  <div id="mobile-menu" popover class="sm:hidden w-full inset-x-0 top-[8vh] p-0 bg-black backdrop-blur-xl">
    <div class="space-y-1 px-2 pt-2 pb-3">
      <a href="/movies" class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white">Kvikmyndir</a>
    </div>
  </div>
</nav>

<div>
  {@render children()}
</div>
