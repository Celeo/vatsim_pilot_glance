const VATSIM_STATUS_URL = "https://status.vatsim.net/status.json";

export interface StatusData {
  v3: Array<string>;
}

export interface Status {
  data: StatusData;
}

export interface Pilot {
  cid: number;
  name: string;
  callsign: string;
  latitude: number;
  longitude: number;
  altitude: number;
  transponder: string;
  logon_time: string;
}

export interface V3ResponseData {
  pilots: Array<Pilot>;
}

export interface RatingsData {
  pilot: number;
}

/**
 * Retrieve a URL to query for getting current VATSIM data.
 */
export async function getV3Endpoint(): Promise<string> {
  const response = await fetch(VATSIM_STATUS_URL);
  if (response.status !== 200) {
    throw new Error(`Got status ${response.status} from status URL`);
  }
  const data: Status = await response.json();
  const urls = data.data.v3;
  return urls[Math.floor(Math.random() * urls.length)];
}

/**
 * Query the VATSIM API for current data.
 */
export async function getOnlinePilots(url: string): Promise<Array<Pilot>> {
  const response = await fetch(url);
  if (response.status !== 200) {
    throw new Error(`Got status ${response.status} from V3 URL ${url}`);
  }
  const data: V3ResponseData = await response.json();
  data.pilots.sort((a, b): number => {
    const aS = a.callsign.toLowerCase();
    const bS = b.callsign.toLowerCase();
    return aS < bS ? -1 : aS > bS ? 1 : 0;
  });
  return data.pilots;
}

/**
 * Get a VATSIM user's total time piloting on the network.
 */
export async function getPilotTime(cid: number): Promise<number> {
  const response = await fetch(
    `https://api.vatsim.net/api/ratings/${cid}/rating_times`
  );
  if (response.status !== 200) {
    throw new Error(`Got status ${response.status} from CID ratings endpoint`);
  }
  const data: RatingsData = await response.json();
  return data.pilot;
}
