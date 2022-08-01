import { Pilot } from "./api.ts";

export const AIRPORTS = ["KSAN", "KLAX", "KSNA"] as const;

export type Airport = typeof AIRPORTS[number];

/**
 * Locations (lat & long) of supported airports.
 */
export const AIRPORT_LOCATIONS: Record<Airport, [number, number]> = {
  KSAN: [32.7338, -117.1933],
  KLAX: [33.9416, -118.4085],
  KSNA: [33.6762, -117.8675],
};

/**
 * Calculate the Haversine Distance between two (lat & long) points.
 *
 * <https://www.movable-type.co.uk/scripts/latlong.html>
 */
function haversineDistance(
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number
): number {
  const R = 6371e3;
  const φ1 = (lat1 * Math.PI) / 180;
  const φ2 = (lat2 * Math.PI) / 180;
  const Δφ = ((lat2 - lat1) * Math.PI) / 180;
  const Δλ = ((lon2 - lon1) * Math.PI) / 180;
  const a =
    Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
    Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  const d = R * c;
  return Math.round(d * 0.00054);
}

/**
 * Filter the list of pilots down to pilots in range of the given airport.
 */
export function filterPilotDistance(
  pilots: Array<Pilot>,
  airport: Airport,
  distance: number
): Array<Pilot> {
  const airportLocation = AIRPORT_LOCATIONS[airport];
  if (airportLocation === undefined) {
    throw new Error(`Unsupported airport "${airport}"`);
  }
  return pilots.filter(
    (pilot) =>
      haversineDistance(
        pilot.latitude,
        pilot.longitude,
        airportLocation[0],
        airportLocation[1]
      ) <= distance
  );
}
