const SERVER_URL = __ENV.SERVER_URL || "localhost:3000";

export const healthUrl = `http://${SERVER_URL}/health-check`;
export const graphqlUrl = `http://${SERVER_URL}/graphql`;
