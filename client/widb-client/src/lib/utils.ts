
import {
  checkPermissions,
  requestPermissions,
  getCurrentPosition,
  watchPosition
} from '@tauri-apps/plugin-geolocation'

import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';

let resp = 'None';
let resp_status = 0;

export type BusData = {
  name: String;
  direction_text: String;
  pos: Position;
}

export type Position = {
  x: number;
  y: number;
}

export type GetBusDataResponse = {
  data: BusData[];
}

export let bus_position = { x: 0, y: 0}

export let coords = { x: 0, y: 0};

export async function getLocation() {
  console.log("get location")
  try {
    let permissions = await checkPermissions();

    if (
      permissions.location === 'prompt' ||
      permissions.location === 'prompt-with-rationale'
    ) {
      permissions = await requestPermissions(['location']);
    }

    if (permissions.location === 'granted') {
      // Get current position
      let position = await getCurrentPosition();
      coords = { x: position.coords.latitude, y: position.coords.longitude};
      console.log('Current Position:', coords);

      // Watch for location updates
      await watchPosition(
        { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },
        (pos) => {
          console.log('Updated Position:', pos);
        }
      );
    } else {
      console.error('Location permission denied');
    }
  } catch (error) {
    console.error('Error getting location:', error);
  }
}

export async function getBusRoute() {
  const url = 'http://192.168.178.41:3000/bus_route'
  const headers = {
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "Content-Type": "application/json"
  }
  const request_data = `{
      "out_frwd": true,
      "arr_station": {
        "name": "Halifaxstraße",
        "lid": "A=1@O=Halifaxstraße@X=6058001@Y=50779534@U=80@L=1427@B=1@p=1749593764@i=A×de:05334:1427@",
        "type": "A",
        "ext_id": "1427",
        "coord_x": 6058001,
        "coord_y": 50779534
      },
      "dep_station": {
        "name": "Ponttor, AC",
        "lid": "A=1@O=Ponttor, AC@X=6077841@Y=50781709@U=80@L=1055@B=1@p=1749593764@i=A×de:05334:1055@",
        "type": "A",
        "ext_id": "1055",
        "coord_x": 6077841,
        "coord_y": 50781709
      }
    }`
    try {
      const response = await fetch(url, {
        method: 'POST',
        headers: headers,
        body: request_data
      });
      
      resp_status = response.status;
      
      if (!response.ok) {
        // Try to get the error details from the response body
        let errorBody;
        try {
          errorBody = await response.json();
          // Return the specific error message from the API if available
          return `Request failed (${response.status}): ${JSON.stringify(errorBody)}`;
        } catch {
          // If response is not JSON, get it as text
          errorBody = await response.text();
          return `Request failed (${response.status}): ${errorBody}`;
        }
      }

      const raw = await response.json();
      console.log("Raw response from backend: ", raw);
  
      
      const bus_data = raw as BusData[];
      console.log("Bus data", bus_data);
      bus_position.y = Math.trunc(bus_data[0].pos.x / 1)/1000000;
      bus_position.x = Math.trunc(bus_data[0].pos.y / 1)/1000000;
      console.log("Bus position (request): ", bus_position);
      resp = bus_position.x + ", " + bus_position.y;
      console.log(response.status); // e.g. 200
      console.log(response.statusText); // e.g. "OK"
      return bus_data;
      
    } catch (error) {
        // Handle network errors, CORS issues, etc.
        // Return the actual error name and message
        return `Network error: ${error.name} - ${error.message}`;
    }
}