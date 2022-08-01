import { getOnlinePilots, getPilotTime, getV3Endpoint, Pilot } from "./api.ts";
import { filterPilotDistance, Airport, AIRPORTS } from "./staticData.ts";
import { parse as argParse } from "https://deno.land/std@0.150.0/flags/mod.ts";

/**
 * App version.
 */
const APP_VERSION = "0.1.0";

/**
 * Maximum distance from the airport to get pilot data.
 */
const MAXIMUM_DISTANCE = 30;

/**
 * Maximum number of hours to add highlight for.
 */
const HIGHLIGHT_HOURS = 30;

/**
 * CLI help text.
 */
const HELP_TEXT =
  `vatsim_pilot_glance

Show VATSIM pilots near an airport and their hours.

USAGE:
    vatsim_pilot_glance (AIRPORT) [OPTIONS]

OPTIONS:
    -h, --help        Show this menu
    -v, --version     Show the version.

Supported airports:
` + AIRPORTS.map((a) => `    ${a}`).join("\n");

/**
 * Get VATSIM pilot times for the given pilots.
 *
 * Defaults to and updates the cache.
 */
async function getTimes(
  pilots: Array<Pilot>,
  cache: Record<number, number>
): Promise<Array<[string, number]>> {
  const ret: Array<[string, number]> = [];

  async function _do(pilot: Pilot): Promise<void> {
    const cached = cache[pilot.cid];
    if (cached !== undefined) {
      ret.push([pilot.callsign, cache[pilot.cid]]);
      return;
    }
    const time = await getPilotTime(pilot.cid);
    cache[pilot.cid] = time;
    ret.push([pilot.callsign, time]);
  }

  await Promise.all(pilots.map((pilot) => _do(pilot)));
  return ret;
}

/**
 * Entry point.
 */
async function main(airport: Airport): Promise<void> {
  const v3Url = await getV3Endpoint();
  const pilots = await getOnlinePilots(v3Url);
  const pilotsInRange = filterPilotDistance(pilots, airport, MAXIMUM_DISTANCE);
  const pilotDataCache: Record<number, number> = {};
  const times = await getTimes(pilotsInRange, pilotDataCache);
  times.sort((a, b) => a[1] - b[1]);

  console.log(
    times
      .map(
        ([callsign, time]) => ` - ${callsign}: ${time.toLocaleString("en-US")}`
      )
      .join("\n")
  );
}

if (import.meta.main) {
  const args = argParse(Deno.args, {});
  if (args.h || args.help) {
    console.log(HELP_TEXT);
    Deno.exit(0);
  }
  if (args.v || args.version) {
    console.log(`vatsim_pilot_glance, version: ${APP_VERSION}`);
    Deno.exit(0);
  }
  if (args._[0] === undefined) {
    console.log("No airport specified (try -h)");
    Deno.exit(1);
  }
  await main(`${args._[0]}` as Airport);
}
