
import {
  checkPermissions,
  requestPermissions,
  getCurrentPosition,
  watchPosition
} from '@tauri-apps/plugin-geolocation'

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