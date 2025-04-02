
import {
  checkPermissions,
  requestPermissions,
  getCurrentPosition,
  watchPosition
} from '@tauri-apps/plugin-geolocation'

let resp = 'None';
let resp_status = 0;

  type Position = {
    x: number;
    y: number;
  }

  type GetPositionResponse = {
    data: Position[];
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
        "lid": "A=1@O=Halifaxstraße@X=6058001@Y=50779534@U=80@L=1427@B=1@p=1739229385@",
        "type": "S",
        "ext_id": "1427",
        "coord_x": 6058001,
        "coord_y": 50779534
      },
      "dep_station": {
        "name": "Bushof, AC",
        "lid": "A=1@O=Bushof, AC@X=6089661@Y=50776477@U=80@L=1001@B=1@p=1740697285@",
        "type": "S",
        "ext_id": "1001",
        "coord_x": 6089661,
        "coord_y": 50776477
      }
    }`
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: headers,
      body: request_data
    });
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }
    resp_status = response.status;
    const position_data = (await response.json()) as GetPositionResponse;
    console.log("Bus data", position_data);
    bus_position.x = position_data.data[0].x;
    bus_position.y = position_data.data[0].y;
    resp = position_data.data[0].x + ", " + position_data.data[0].y;
    console.log(response.status); // e.g. 200
    console.log(response.statusText); // e.g. "OK"
  } catch (error) {
    console.error("Error: ", error);
  }
}