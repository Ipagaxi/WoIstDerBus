<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
     integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
     crossorigin=""/>

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import L, { circle, Layer, type LeafletEvent } from 'leaflet';
  import 'leaflet/dist/leaflet.css';
  import {
    checkPermissions,
    requestPermissions,
    getCurrentPosition,
    watchPosition
  } from '@tauri-apps/plugin-geolocation'

  import { coords, getLocation, getBusRoute, bus_position } from '$lib/utils.ts';
  import { type GetBusDataResponse, type BusData } from '$lib/utils.ts';

  let log = "";

  let map;
  let userMarker;
  let bus_circle;
  let greenBus;
  let redBus;
  let blueBus;

  var greenBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">73</span>'+
          '<img class="my-div-image" src="/icons/bus_green.png"/>'          
  });

  var redBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">33</span>'+
          '<img class="my-div-image" src="/icons/bus_red.png"/>'          
  });

  var blueBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">12</span>'+
          '<img class="my-div-image" src="/icons/bus_blue.png"/>'          
  });

  let interval;
  let busesLayer = L.layerGroup;

  onMount(() => {

    getLocation()

    // Initialize the map with a temporary center
    map = L.map('map').setView([50.775, 6.084], 16);

    bus_circle = L.circle([50.775, 6.084], {
      color: 'red',
      fillColor: '#f03',
      fillOpacity: 0.5,
      radius: 50
    }).addTo(map);

    /*greenBus = L.marker([50.776, 6.084], {icon: greenBusIcon}).addTo(map);
    redBus = L.marker([50.774, 6.084], {icon: redBusIcon}).addTo(map);
    blueBus = L.marker([50.774, 6.084], {icon: redBusIcon}).addTo(map);*/

    // Add OpenStreetMap tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; OpenStreetMap contributors',
    }).addTo(map);

    busesLayer = L.layerGroup().addTo(map);

    interval = setInterval(async () => {
      
      const result = await getBusRoute() as BusData[];

      busesLayer.clearLayers(); // nukes all previous markers

      log = ""
      result.forEach(element => {
        const lat = element.pos.y / 1e6;
        const lng = element.pos.x / 1e6;

        let icon = undefined;
        if (element.name === "73") icon = greenBusIcon;
        else if (element.name === "33") icon = redBusIcon;
        else if (element.name === "12") icon = blueBusIcon;

        if (icon) L.marker([lat, lng], { icon }).addTo(busesLayer);
        log += element.name + " nach " + element.direction_text + ": " + lat + ", " + lng + "\n";
      });
    }, 5000);
  });

  function update_position() {
    invoke('frontend_log', { message: '' + bus_position.x });
    console.log("coords", coords);
    getLocation();
    map.panTo([coords.x, coords.y]);
  }

  // Clean up when the component is destroyed
  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<div class="container">
  <h1>Wo ist eigentlich der Bus?!</h1>

  <div id="map"></div>

  <button on:click="{update_position}">
    Update position
  </button>
  <pre>Log: {log}</pre>
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
