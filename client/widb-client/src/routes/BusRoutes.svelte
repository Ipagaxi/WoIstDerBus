<script lang="ts">
	import axios from 'axios';

  let resp = 'None';
  let resp_status = 0;

  async function getBusRoute() {
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
      resp = await response.text();
      console.log(response.status); // e.g. 200
      console.log(response.statusText); // e.g. "OK"
    } catch (error) {
      console.error("Error: ", error);
    }
  }
</script>

<div class="container">
  <button on:click="{getBusRoute}">
    get bus infos
  </button>
  Response: { resp }
  Status: { resp_status }
</div>

<style>
	/* styles go here */
</style>