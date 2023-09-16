import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.2/index.js";
import http from "k6/http";
import exec from "k6/execution";
import { check } from "k6";
import { healthUrl } from "./urls.js";
import { User, UserFriends, UserThriceNestedFriends } from "./requests.js";

export const options = {
  thresholds: {
    checks: ["rate == 1"],
    http_req_duration: ["max < 500", "p(99) < 300", "med < 200"],
  },
  scenarios: {
    smoke: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 100,
      maxDuration: "1s",
      startTime: "1s",
      exec: "smoke",
    },
    normal_requests: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 10000,
      maxDuration: "10s",
      startTime: "2s",
      exec: "normal",
    },
    normal_requests_spike: {
      executor: "constant-vus",
      vus: 1000,
      duration: "30s",
      startTime: "10s",
      exec: "normal",
    },
    mean_requests: {
      executor: "shared-iterations",
      vus: 10,
      iterations: 10000,
      maxDuration: "30s",
      startTime: "40s",
      exec: "mean",
    },
  },
};

const maxUserId = 10000;

export const smoke = () => {
  const res = http.get(healthUrl);
  check(res, { "response did not contain error": (r) => r.status == 200 });
};

const normalRequests = [User, UserFriends];
export const normal = () => {
  const i = exec.scenario.iterationInInstance % normalRequests.length;
  const id = (exec.scenario.iterationInInstance % maxUserId) + 1;

  const res = normalRequests[i](id);

  check(res, {
    "response did not contain error": (r) => r.status == 200 && !r.body.errors,
  });
};

const meanRequests = [UserThriceNestedFriends];
export const mean = () => {
  const i = exec.scenario.iterationInInstance % meanRequests.length;
  const id = (exec.scenario.iterationInInstance % maxUserId) + 1;

  const res = meanRequests[i](id);

  check(res, {
    "response did not contain error": (r) => r.status == 200 && !r.body.errors,
  });
};

export const handleSummary = (data) => {
  return {
    stdout: textSummary(data),
    "./last_run.txt": textSummary(data, { indent: "  ", enableColors: false }),
  };
};
