<script lang="ts">
    import { onMount } from 'svelte';
    // FIXED IMPORTS BELOW:
    import { flip } from 'svelte/animate';
    import { fade, fly } from 'svelte/transition';

    const IMAGE_URL = 'https://image.tmdb.org/t/p/w500';
    const RUST_API = 'http://localhost:3000/movies';

    let savedMovies: any[] = [];
    
    // Controls
    let searchTerm = "";
    let sortOption = "newest"; // newest, oldest, title, year
    let filterUser = "all";

    // Computed Lists
    $: uniqueUsers = ["all", ...new Set(savedMovies.map(m => m.user_name))];

    // Reactive Filter & Sort Logic
    $: filteredMovies = savedMovies
        .filter(movie => {
            const matchesSearch = 
                movie.title.toLowerCase().includes(searchTerm.toLowerCase()) || 
                movie.comment.toLowerCase().includes(searchTerm.toLowerCase());
            const matchesUser = filterUser === "all" || movie.user_name === filterUser;
            return matchesSearch && matchesUser;
        })
        .sort((a, b) => {
            if (sortOption === "newest") return b.id - a.id;
            if (sortOption === "oldest") return a.id - b.id;
            if (sortOption === "title") return a.title.localeCompare(b.title);
            if (sortOption === "year") {
                const yearA = a.release_date ? parseInt(a.release_date.split('-')[0]) : 0;
                const yearB = b.release_date ? parseInt(b.release_date.split('-')[0]) : 0;
                return yearB - yearA;
            }
            return 0;
        });

    async function fetchSaved() {
        try {
            const res = await fetch(RUST_API);
            if (res.ok) savedMovies = await res.json();
        } catch (e) { console.error("API offline?"); }
    }

    async function removeMovie(id: number) {
        if(!confirm("Remove from collection?")) return;
        
        // Optimistic update
        savedMovies = savedMovies.filter(m => m.tmdb_id !== id);
        
        await fetch(`${RUST_API}/${id}`, { method: 'DELETE' });
    }

    onMount(fetchSaved);
</script>

<main class="max-w-7xl mx-auto p-8 text-gray-100 min-h-screen">
    
    <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
        <h2 class="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-purple-400">
            Full Collection <span class="text-gray-500 text-lg">({filteredMovies.length})</span>
        </h2>

        <div class="flex flex-wrap gap-3 bg-gray-800 p-2 rounded-xl border border-gray-700">
            
            <input 
                type="text" 
                placeholder="Search collection..." 
                bind:value={searchTerm}
                class="bg-gray-900 border border-gray-700 text-white text-sm rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
            />

            <select bind:value={filterUser} class="bg-gray-900 border border-gray-700 text-white text-sm rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none">
                {#each uniqueUsers as user}
                    <option value={user}>{user === 'all' ? 'All Users' : user}</option>
                {/each}
            </select>

            <select bind:value={sortOption} class="bg-gray-900 border border-gray-700 text-white text-sm rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none">
                <option value="newest">Date Added (Newest)</option>
                <option value="oldest">Date Added (Oldest)</option>
                <option value="title">Title (A-Z)</option>
                <option value="year">Release Year</option>
            </select>
        </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
        {#each filteredMovies as movie (movie.tmdb_id)}
            <div 
                animate:flip={{duration: 300}}
                in:fly={{y: 20, duration: 300}}
                class="bg-gray-800 rounded-xl overflow-hidden shadow-lg border border-gray-700 flex flex-col"
            >
                <div class="relative aspect-video bg-gray-900 overflow-hidden group">
                    {#if movie.poster_path}
                        <img src={IMAGE_URL + movie.poster_path} alt={movie.title} class="w-full h-full object-cover object-top opacity-80 group-hover:opacity-100 transition duration-500" />
                    {:else}
                        <div class="w-full h-full flex items-center justify-center text-gray-500">No Image</div>
                    {/if}
                    
                    <div class="absolute inset-0 bg-gradient-to-t from-gray-900 via-transparent to-transparent"></div>
                    
                    <button 
                        on:click={() => removeMovie(movie.tmdb_id)}
                        class="absolute top-2 right-2 bg-black/50 hover:bg-red-500 text-white p-2 rounded-full transition backdrop-blur-md"
                        title="Remove"
                    >
                        🗑️
                    </button>
                </div>

                <div class="p-4 flex-1 flex flex-col">
                    <div class="flex justify-between items-start mb-2">
                        <h3 class="font-bold text-lg text-white leading-tight">{movie.title}</h3>
                        <span class="text-xs font-mono bg-gray-700 px-2 py-1 rounded text-gray-300">
                            {movie.release_date ? movie.release_date.split('-')[0] : 'N/A'}
                        </span>
                    </div>
                    
                    <p class="text-gray-400 text-sm italic mb-4 flex-1">"{movie.comment}"</p>
                    
                    <div class="border-t border-gray-700 pt-3 flex justify-between items-center text-xs text-gray-500">
                        <span>Added by <span class="text-indigo-400">{movie.user_name}</span></span>
                    </div>
                </div>
            </div>
        {:else}
            <div class="col-span-full text-center py-20 text-gray-500">
                No movies found matching your filters.
            </div>
        {/each}
    </div>
</main>
