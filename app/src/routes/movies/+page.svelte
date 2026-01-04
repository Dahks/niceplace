<script>
  import { onMount } from 'svelte';

  // --- CONFIGURATION ---
  // Access the variable from the .env file (must start with VITE_)
  const API_KEY = import.meta.env.VITE_TMDB_API_KEY;
  const BASE_URL = 'https://api.themoviedb.org/3';
  const IMAGE_URL = 'https://image.tmdb.org/t/p/w500';

  // --- STATE ---
  let movies = [];
  let searchQuery = '';
  let loading = false;
  let error = null;

  // Future feature: "Collection" storage
  // For now, we just track IDs in a simple Set to show the UI working
  let collection = new Set();

  // --- FUNCTIONS ---

  async function fetchMovies() {
    // Safety check for the API key
    if (!API_KEY) {
      error = "Missing API Key! Make sure VITE_TMDB_API_KEY is in your .env file and you have restarted the server.";
      return;
    }

    loading = true;
    error = null;

    try {
      // If search is empty, fetch popular movies. If not, search.
      const endpoint = searchQuery 
        ? `${BASE_URL}/search/movie?api_key=${API_KEY}&query=${encodeURIComponent(searchQuery)}`
        : `${BASE_URL}/movie/popular?api_key=${API_KEY}`;

      const res = await fetch(endpoint);
      const data = await res.json();

      if (res.ok) {
        movies = data.results || [];
      } else {
        throw new Error(data.status_message || 'Failed to fetch');
      }
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  // Handle the "Save to Collection" button
  function toggleCollection(movie) {
    // Svelte reactivity requires reassignment for Sets/Arrays to update view
    const newCollection = new Set(collection);
    
    if (newCollection.has(movie.id)) {
      newCollection.delete(movie.id);
    } else {
      newCollection.add(movie.id);
      // This is where we will eventually add the 'vibe' note
      console.log(`Saved ${movie.title} to collection`);
    }
    
    collection = newCollection;
  }

  function handleSearch() {
    fetchMovies();
  }

  // Load popular movies on first render
  onMount(() => {
    fetchMovies();
  });
</script>

<main>
  <header>
    <h1>🎬 Movie Explorer</h1>
    
    <div class="search-bar">
      <input 
        type="text" 
        placeholder="Search for a movie..." 
        bind:value={searchQuery} 
        on:keydown={(e) => e.key === 'Enter' && handleSearch()}
      />
      <button on:click={handleSearch} disabled={loading}>
        {loading ? 'Searching...' : 'Search'}
      </button>
    </div>
  </header>

  {#if error}
    <div class="error">⚠️ {error}</div>
  {/if}

  <div class="movie-grid">
    {#each movies as movie (movie.id)}
      <div class="card">
        <div class="poster-wrapper">
          {#if movie.poster_path}
            <img src={IMAGE_URL + movie.poster_path} alt={movie.title} />
          {:else}
            <div class="no-image">No Image</div>
          {/if}
          
          <button 
            class="save-btn {collection.has(movie.id) ? 'saved' : ''}"
            on:click={() => toggleCollection(movie)}
            title="Save to Collection"
          >
            {collection.has(movie.id) ? '♥' : '♡'}
          </button>
        </div>

        <div class="info">
          <h3>{movie.title}</h3>
          <p class="date">{movie.release_date ? movie.release_date.split('-')[0] : 'Unknown'}</p>
          <p class="overview">{movie.overview.slice(0, 100)}...</p>
        </div>
      </div>
    {:else}
      {#if !loading && !error}
        <p>No movies found. Try a different search.</p>
      {/if}
    {/each}
  </div>
</main>

<style>
  :global(body) {
    font-family: sans-serif;
    background: #1a1a1a;
    color: #fff;
    margin: 0;
  }

  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }

  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 40px;
  }

  h1 { margin-bottom: 20px; }

  .search-bar {
    display: flex;
    gap: 10px;
    width: 100%;
    max-width: 500px;
  }

  input {
    flex: 1;
    padding: 10px;
    border-radius: 4px;
    border: none;
    font-size: 16px;
  }

  button {
    padding: 10px 20px;
    background: #ff3e00;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
  }

  button:disabled { opacity: 0.7; cursor: not-allowed; }

  /* Grid Layout */
  .movie-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 2rem;
  }

  /* Card Styling */
  .card {
    background: #2a2a2a;
    border-radius: 8px;
    overflow: hidden;
    transition: transform 0.2s;
    position: relative;
  }

  .card:hover {
    transform: translateY(-5px);
  }

  .poster-wrapper {
    position: relative;
    aspect-ratio: 2/3;
    background: #333;
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .no-image {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
  }

  .info {
    padding: 15px;
  }

  h3 {
    margin: 0 0 5px 0;
    font-size: 1.1rem;
  }

  .date {
    color: #888;
    font-size: 0.9rem;
    margin-bottom: 10px;
  }

  .overview {
    font-size: 0.85rem;
    color: #ccc;
    line-height: 1.4;
  }

  /* Save Button Styling */
  .save-btn {
    position: absolute;
    top: 10px;
    right: 10px;
    background: rgba(0,0,0,0.6);
    border-radius: 50%;
    width: 35px;
    height: 35px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    line-height: 0;
    transition: background 0.2s;
    border: none;
    color: white;
    cursor: pointer;
  }

  .save-btn:hover { background: rgba(0,0,0,0.9); }
  .save-btn.saved { color: #ff3e00; }
  
  .error {
    color: #ffcccc;
    background: #cc0000;
    padding: 10px;
    border-radius: 4px;
    text-align: center;
    margin-bottom: 20px;
  }
</style>
