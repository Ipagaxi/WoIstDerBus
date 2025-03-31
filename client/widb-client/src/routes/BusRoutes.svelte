<script lang="ts">
	import axios from 'axios';

type User = {
  id: number;
  email: string;
  first_name: string;
};

type GetUsersResponse = {
  data: User[];
};

let resp = 'None';

async function getBusRoute() {
  const url = 'http://127.0.0.1:3000/bus_route'
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
    // 👇️ const data: GetUsersResponse
    const { data, status } = await axios.post(
      'https://auskunft.avv.de/bin/mgate.exe?rnd=1736690873299',
      request_data,
      {
        headers: {
          "User-Agent": 'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0',
          Accept: 'application/json',
          "Accept-Language": 'de,en-US;q=0.7,en;q=0.3',
          "Accept-Encoding": 'gzip, deflate, br, zstd',
          DNT: 1,
          Connection: 'keep-alive',
          Referer: 'https://auskunft.avv.de/?L=vs_aseag',
          'Sec-Fetch-Dest': 'empty',
          'Sec-Fetch-Mode': 'cors',
          'Sec-Fetch-Site': 'same-origin'
        }
      },
    );

    console.log(JSON.stringify(data, null, 4));

    // 👇️ "response status is: 200"
    console.log('response status is: ', status);
    resp = data
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.log('error message: ', error.message);
      resp = error.message;
      return error.message;
    } else {
      console.log('unexpected error: ', error);
      resp = 'Unexptected error';
      return 'An unexpected error occurred';
    }
  }
}
</script>

<div class="container">
  <button on:click="{getBusRoute}">
    get bus infos
  </button>
  Response: { resp }
</div>

<style>
	/* styles go here */
</style>