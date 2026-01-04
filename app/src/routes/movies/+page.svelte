<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from "$app/stores";
  import { fade, slide } from 'svelte/transition';

  const API_KEY = import.meta.env.VITE_TMDB_API_KEY;
  const BASE_URL = 'https://api.themoviedb.org/3';
  const IMAGE_URL = 'https://image.tmdb.org/t/p/w500';
  const RUST_API = 'http://localhost:3000/movies';

  let movies: any[] = [];
  let searchQuery = '';
  let loading = false;
  let savedMovies: any[] = []; 

  async function fetchMovies() {
    loading = true;
    try {
      const endpoint = searchQuery 
        ? `${BASE_URL}/search/movie?api_key=${API_KEY}&query=${encodeURIComponent(searchQuery)}`
        : `${BASE_URL}/movie/popular?api_key=${API_KEY}`;
      const res = await fetch(endpoint);
      const data = await res.json();
      if (res.ok) movies = data.results || [];
    } catch (e) { console.error(e); } 
    finally { loading = false; }
  }

  async function fetchSaved() {
    try {
        const res = await fetch(RUST_API);
        if (res.ok) savedMovies = await res.json();
    } catch (e) { console.error("API offline?"); }
  }

  async function toggleCollection(movie: any) {
    const savedEntry = savedMovies.find(m => m.tmdb_id === movie.id);

    if (savedEntry) {
        // Remove
        savedMovies = savedMovies.filter(m => m.tmdb_id !== movie.id);
        await fetch(`${RUST_API}/${movie.id}`, { method: 'DELETE' });
    } else {
        // Add
        const comment = prompt(`Add a note for "${movie.title}":`, "Must watch!") || "";
        if (!comment) return;

        const newSave = {
            tmdb_id: movie.id,
            title: movie.title,
            comment: comment,
            user_name: $page.data.session?.user?.name || "Anonymous",
            poster_path: movie.poster_path,
            release_date: movie.release_date
        };
        savedMovies = [newSave, ...savedMovies];

        await fetch(RUST_API, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(newSave)
        });
    }
  }

  function handleSearch() { fetchMovies(); }

  onMount(() => { 
    fetchMovies(); 
    fetchSaved(); 
  });
</script>

<main class="max-w-7xl mx-auto p-8 text-gray-100">
  
  {#if savedMovies.length > 0}
      <div class="mb-12" transition:slide>
        <div class="flex justify-between items-end mb-6">
            <h2 class="text-2xl font-bold text-white">Just Added</h2>
            <a href="/collection" class="text-indigo-400 text-sm hover:text-indigo-300">View All &rarr;</a>
        </div>
        
        <div class="flex gap-4 overflow-x-auto pb-4 scrollbar-hide snap-x">
            {#each savedMovies.slice(0, 5) as saved (saved.tmdb_id)}
                <div 
                    class="relative flex-shrink-0 w-[140px] bg-gray-800 rounded-lg overflow-hidden border border-gray-700 shadow-xl snap-start group"
                    transition:fade
                >
                    <div class="aspect-[2/3] bg-gray-900 relative">
                        {#if saved.poster_path}
                            <img src={IMAGE_URL + saved.poster_path} alt={saved.title} class="w-full h-full object-cover" />
                        {:else}
                            <div class="w-full h-full flex items-center justify-center text-xs text-gray-400">{saved.title}</div>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
      </div>
  {/if}

  <div class="text-center mb-10">
      <h1 class="text-4xl font-extrabold mb-4">Discover Movies</h1>
      <div class="search-bar flex gap-2 max-w-xl mx-auto">
        <input 
          type="text" 
          class="flex-1 p-4 rounded-xl border border-gray-700 bg-gray-800 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition"
          placeholder="Search TMDB..." 
          bind:value={searchQuery} 
          on:keydown={(e) => e.key === 'Enter' && handleSearch()}
        />
        <button 
          class="px-8 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-bold transition"
          on:click={handleSearch} 
          disabled={loading}
        >
          {loading ? '...' : 'Search'}
        </button>
      </div>
  </div>

  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
    {#each movies as movie (movie.id)}
      <div class="bg-gray-800 rounded-xl overflow-hidden shadow-lg hover:-translate-y-2 transition duration-300 relative group border border-gray-700">
        <div class="relative aspect-[2/3] bg-gray-900">
          {#if movie.poster_path}
            <img src={IMAGE_URL + movie.poster_path} alt={movie.title} class="w-full h-full object-cover" />
          {:else}
            <div class="flex items-center justify-center h-full text-gray-500 text-sm">No Image</div>
          {/if}
          
          <button 
            class="absolute top-2 right-2 w-10 h-10 rounded-full flex items-center justify-center text-2xl transition shadow-xl backdrop-blur-sm"
            class:bg-white={!savedMovies.some(m => m.tmdb_id === movie.id)}
            class:text-gray-300={!savedMovies.some(m => m.tmdb_id === movie.id)}
            class:bg-red-500={savedMovies.some(m => m.tmdb_id === movie.id)}
            class:text-white={savedMovies.some(m => m.tmdb_id === movie.id)}
            on:click={() => toggleCollection(movie)}
          >
            {savedMovies.some(m => m.tmdb_id === movie.id) ? '♥' : '♡'}
          </button>
        </div>
        <div class="p-4">
            <h3 class="font-bold text-white text-sm mb-1 truncate">{movie.title}</h3>
            <p class="text-gray-500 text-xs">{movie.release_date?.split('-')[0] || 'Unknown'}</p>
        </div>
      </div>
    {/each}
  </div>
</main>
