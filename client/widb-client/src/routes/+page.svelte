<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
     integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
     crossorigin=""/>

<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, tick } from 'svelte';
  import L, { circle, Layer, type LeafletEvent } from 'leaflet';
  import 'leaflet/dist/leaflet.css';
  import 'polyline-encoded/Polyline.encoded.js'
  import {
    checkPermissions,
    requestPermissions,
    getCurrentPosition,
    watchPosition
  } from '@tauri-apps/plugin-geolocation'

  import { coords, getLocation, getBusRoute, bus_position} from '$lib/utils.ts';
  import { type GetBusDataResponse, type BusData } from '$lib/utils.ts';
  import busIcon from "$lib/icons/bus.svg?raw";

  let log = "";

  let map;
  let pos_marker;

  const arr_station = {
    pos_x: 50.779534,
    pos_y: 6.058001
  };

  const dep_station = {
    pos_x: 50.781709,
    pos_y: 6.077841
  };

  var greenBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">73</span>'+
          '<img class="my-div-image" src="/icons/bus.svg" width="32px"/>'          
  });

  var redBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">33</span>'+
          '<img class="my-div-image" src="/icons/bus.svg" width="32px"/>'          
  });

  var blueBusIcon = new L.DivIcon({
    className: 'my-div-icon',
    html: '<span class="my-div-span">12</span>'+
          '<img class="my-div-image" src="/icons/bus.svg" width="32px"/>'          
  });

  let interval;
  let busesLayer = L.layerGroup;

  onMount(async () => {
    await tick();

    // Initialize the map with a temporary center
    map = L.map('map', {preferCanvas: false}).setView([50.775, 6.084], 16);

    show_position();

    // Add OpenStreetMap tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; OpenStreetMap contributors',
      subdomains: ['a', 'b', 'c'],
      crossOrigin: true
    }).addTo(map);

    var arr_station_marker = L.marker([arr_station.pos_x, arr_station.pos_y]).addTo(map);
    var dep_station_marker = L.marker([dep_station.pos_x, dep_station.pos_y]).addTo(map);

    busesLayer = L.layerGroup().addTo(map);

    map.invalidateSize();

    const resizeHandler = () => map.invalidateSize();
    window.addEventListener('resize', resizeHandler);

    let encoded = "sp|tH{hed@e@GBFo@V_AXAG@FwBt@mHjOO@??NAiHtOMfAQ|ICbOKC??JBs@bI@xBp@hKvBnJRVh@`BMN??LOtD~Hh@j@G^??F_@|An@k@lDmEnIZ`@j@bAx@T?T???U\\Lz@tA\\`AFhAkBzKWt@ENIE??HD[v@GVsBhGmChMCGNa@";
    let polyline = L.Polyline.fromEncoded(encoded);

    interval = setInterval(async () => {
      
      const result = await getBusRoute() as BusData[];

      busesLayer.clearLayers();

      console.log("Polyline: ", polyline.getLatLngs());

      log = ""
      result.forEach(async (element) => {
        const lat = element.pos.y / 1e6;
        const lng = element.pos.x / 1e6;

        let icon = undefined;

        icon = L.divIcon({
          html: busIcon,
          className: `bus-icon-${element.name}`,
          iconSize: [32, 32],
        });
        L.marker([lat, lng], { icon }).addTo(busesLayer);
        const rect = document.querySelector(`.bus-icon-${element.name} rect`);
        if (rect) {
          let color = bus_id_to_color(element.name);
          rect.style.fill=color;
        }

        log += element.name + " nach " + element.direction_text + ": " + lat + ", " + lng + "\n";
      });
      
    }, 5000);
  });

  function bus_id_to_color(bus_id: String) {
    var hash = 0;
    if (bus_id.length === 0) return hash;
    for (var i = 0; i < bus_id.length; i++) {
        hash = bus_id.charCodeAt(i) + ((hash << 5) - hash);
        hash = hash & hash;
    }
    var color = '#';
    for (var i = 0; i < 3; i++) {
        var value = (hash >> (i * 8)) & 255;
        color += ('00' + value.toString(16)).substr(-2);
    }
    return color;
  }

  async function show_position() {
    if (await update_position()) {
      map.panTo([coords.x, coords.y]);
    }
  }

  async function update_position() {
    invoke('frontend_log', { message: '' + bus_position.x });
    console.log("coords", coords);
    let retrieved_location = await getLocation();
    if (retrieved_location) {
      if (pos_marker) {
        pos_marker.setLatLng([coords.x, coords.y]);
      } else {
        pos_marker = L.marker([coords.x, coords.y]).addTo(map);
      }
    } else {
      //pos_marker = undefined;
    }
    return retrieved_location;
  }

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
  #map { 
    height: 180px;
    min-height: 180px;
  }

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
