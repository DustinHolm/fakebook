import http from "k6/http";
import exec from "k6/execution";
import encoding from "k6/encoding";
import { check } from "k6";
import { healthUrl } from "./util/urls.js";
import {
  User,
  UserFriends,
  UserFriendsPosts,
  UserFriendsPostsComments,
  UserDoubleNestedFriends,
} from "./util/requests.js";
import { handleSummaryFn } from "./util/summary.js";

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
    mixed_requests_spike: {
      executor: "constant-vus",
      vus: 1000,
      duration: "20s",
      startTime: "10s",
      exec: "mixed",
      env: { PAGINATION: "3" },
    },
    friends_of_friends: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 10,
      maxDuration: "20s",
      startTime: "30s",
      exec: "friendsOfFriends",
    },
    user: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "50s",
      exec: "user",
    },
    user_and_friends: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "60s",
      exec: "userFriends",
    },
    user_and_friends_posts: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "70s",
      exec: "userFriendsPosts",
    },
    user_and_friends_posts_3_paginated: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "80s",
      exec: "userFriendsPosts",
      env: { PAGINATION: "3" },
    },
    user_and_friends_posts_with_comments: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "90s",
      exec: "userFriendsPostsComments",
    },
    user_and_friends_posts_with_comments_3_paginated: {
      executor: "shared-iterations",
      vus: 100,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "100s",
      exec: "userFriendsPostsComments",
      env: { PAGINATION: "3" },
    },
  },
};

const maxUserIdQuery = 5000;

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

export const friendsOfFriends = () => {
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");

  const res = UserDoubleNestedFriends(id);

  check(res, {
    "response did not contain error": (r) =>
      r.status == 200 && !!r.json() && !r.json().errors,
  });
};

export const handleSummary = (data) => handleSummaryFn(data, "full");
