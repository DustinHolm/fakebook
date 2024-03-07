function requireString(x: unknown): string {
  if (typeof x !== "string") throw Error("Required a string!");

  return x;
}

export const env = Object.freeze({
  serverUrl: requireString(import.meta.env.VITE_SERVER_URL),
  serverWsUrl: requireString(import.meta.env.VITE_SERVER_WS_URL),
});
