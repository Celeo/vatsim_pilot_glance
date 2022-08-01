import { getOnlinePilots, getPilotTime, getV3Endpoint, Pilot } from "./api.ts";
import { filterPilotDistance } from "./staticData.ts";

/**
 * Maximum distance from the airport to get pilot data.
 */
const MAXIMUM_DISTANCE = 30;

/**
 * Maximum number of hours to alert for.
 */
const ALERT_HOURS = 30;

/**
 * Get VATSIM pilot times for the given pilots.
 *
 * Defaults to and updates the cache.
 */
async function getTimes(
  pilots: Array<Pilot>,
  cache: Record<number, number>
): Promise<Record<string, number>> {
  const ret: Record<string, number> = {};
  for (const pilot of pilots) {
    const cached = cache[pilot.cid];
    if (cached !== undefined) {
      ret[pilot.callsign] = cache[pilot.cid];
      continue;
    }
    const time = await getPilotTime(pilot.cid);
    cache[pilot.cid] = time;
    ret[pilot.callsign] = time;
  }
  return ret;
}

/**
 * Entry point.
 */
async function main(): Promise<void> {
  const v3Url = await getV3Endpoint();
  const pilots = await getOnlinePilots(v3Url);
  const pilotsInRange = filterPilotDistance(pilots, "KSAN", MAXIMUM_DISTANCE);

  console.log(
    "Pilots in range:",
    pilotsInRange.map((pilot) => pilot.callsign).join(", ")
  );

  const pilotDataCache: Record<number, number> = {};
  console.log(await getTimes(pilotsInRange, pilotDataCache));
  console.log(await getTimes(pilotsInRange, pilotDataCache));
}

if (import.meta.main) {
  await main();
}
