<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
     integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
     crossorigin=""/>

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import L, { circle, type LeafletEvent } from 'leaflet';
  import 'leaflet/dist/leaflet.css';
  import {
    checkPermissions,
    requestPermissions,
    getCurrentPosition,
    watchPosition
  } from '@tauri-apps/plugin-geolocation'

  import { coords, getLocation, getBusRoute, bus_position } from '$lib/utils.ts';
  import BusRoutes from "./BusRoutes.svelte";

  let greetMsg = "";

  let map;
  let userMarker;
  let bus_circle;

  onMount(() => {

    getLocation()

    // Initialize the map with a temporary center
    map = L.map('map').setView([50.0, 6.0], 16);

    bus_circle = L.circle([50.0, 6.0], {
      color: 'red',
      fillColor: '#f03',
      fillOpacity: 0.5,
      radius: 500
    }).addTo(map);


    // Add OpenStreetMap tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; OpenStreetMap contributors',
    }).addTo(map);
  });

  function update_position() {
    console.log("coords", coords);
    getLocation();
    map.setView([coords.x, coords.y], 13);
  }

  const interval = setInterval(() => {
    getBusRoute();
    bus_circle.setLatLng(bus_position.x, bus_position.y)
  }, 5000);

  // Clean up when the component is destroyed
  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>

  <div id="map"></div>

  <button on:click="{update_position}">
    Update position
  </button>
  Pos: { coords.x }, { coords.y }
  Bus Position: { bus_position.x }, { bus_position.y }
  <BusRoutes/>
</div>

<style>
  #map { height: 180px; }

  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
