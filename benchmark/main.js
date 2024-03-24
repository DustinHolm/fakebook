import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.2/index.js";
import http from "k6/http";
import exec from "k6/execution";
import encoding from "k6/encoding";
import { check } from "k6";
import { healthUrl } from "./urls.js";
import {
  AddFriend,
  CreateUser,
  User,
  UserFriends,
  UserFriendsPosts,
  UserFriendsPostsComments,
  UserThriceNestedFriends,
} from "./requests.js";

export const options = {
  thresholds: {
    checks: [
      {
        threshold: "rate == 1",
        abortOnFail: true,
      },
    ],
    http_req_duration: ["max < 500", "p(95) < 330", "med < 150"],
  },
  scenarios: {
    smoke: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 100,
      maxDuration: "1s",
      startTime: "5s",
      exec: "smoke",
    },
    mutations: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "10s",
      exec: "mutation",
    },
    mixed_requests_spike: {
      executor: "constant-vus",
      vus: 1000,
      duration: "20s",
      startTime: "20s",
      exec: "mixed",
      env: { PAGINATION: "3" },
    },
    mean_requests: {
      executor: "shared-iterations",
      vus: 1,
      iterations: 10,
      maxDuration: "20s",
      startTime: "40s",
      exec: "mean",
    },
    user: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "60s",
      exec: "user",
    },
    user_and_friends: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "70s",
      exec: "userFriends",
    },
    user_and_friends_posts: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "80s",
      exec: "userFriendsPosts",
    },
    user_and_friends_posts_3_paginated: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "90s",
      exec: "userFriendsPosts",
      env: { PAGINATION: "3" },
    },
    user_and_friends_posts_with_comments: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "100s",
      exec: "userFriendsPostsComments",
    },
    user_and_friends_posts_with_comments_3_paginated: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "110s",
      exec: "userFriendsPostsComments",
      env: { PAGINATION: "3" },
    },
  },
};

const maxUserIdQuery = 5000;
const maxUserIdMutation = 10000;

export const smoke = () => {
  const res = http.get(healthUrl);
  check(res, { "response did not contain error": (r) => r.status == 200 });
};

export const user = () => {
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");
  const res = User(id);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

export const userFriends = () => {
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");
  const res = UserFriends(id);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

export const userFriendsPosts = () => {
  const pagination = __ENV.PAGINATION
    ? Number.parseInt(__ENV.PAGINATION)
    : undefined;
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");
  const res = UserFriendsPosts(id, pagination);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

export const userFriendsPostsComments = () => {
  const pagination = __ENV.PAGINATION
    ? Number.parseInt(__ENV.PAGINATION)
    : undefined;
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");
  const res = UserFriendsPostsComments(id, pagination);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

const mixedRequests = [
  User,
  UserFriends,
  UserFriendsPosts,
  UserFriendsPostsComments,
];
export const mixed = () => {
  const i = exec.scenario.iterationInInstance % mixedRequests.length;
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");

  const res = mixedRequests[i](id);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

const mutationRequests = [CreateUser, AddFriend];
export const mutation = () => {
  const i = exec.scenario.iterationInInstance % mutationRequests.length;
  let id1 =
    (exec.scenario.iterationInInstance % maxUserIdMutation) +
    maxUserIdQuery +
    1;
  let id2 =
    ((exec.scenario.iterationInInstance + 1) % maxUserIdMutation) +
    maxUserIdQuery +
    1;
  id1 = encoding.b64encode(id1 + "AppUser", "url");
  id2 = encoding.b64encode(id2 + "AppUser", "url");

  const res = mutationRequests[i](id1, id2);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

const meanRequests = [UserThriceNestedFriends];
export const mean = () => {
  const i = exec.scenario.iterationInInstance % meanRequests.length;
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");

  const res = meanRequests[i](id);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

export const handleSummary = (data) => {
  delete data.metrics["http_req_duration{expected_response:true}"];

  for (const key in data.metrics) {
    if (key.startsWith("data")) delete data.metrics[key];
    if (key.startsWith("iteration")) delete data.metrics[key];
    if (key.startsWith("vus")) delete data.metrics[key];
  }

  return {
    stdout: textSummary(data),
    "./last_run.txt": textSummary(data, { indent: "  ", enableColors: false }),
  };
};
