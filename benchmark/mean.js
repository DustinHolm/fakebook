import exec from "k6/execution";
import encoding from "k6/encoding";
import { check } from "k6";
import { UserThriceNestedFriends } from "./util/requests.js";
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
    mean_requests: {
      executor: "shared-iterations",
      vus: 1,
      iterations: 10,
      maxDuration: "20s",
      exec: "mean",
    },
  },
};

const maxUserIdQuery = 5000;

const meanRequests = [UserThriceNestedFriends];
export const mean = () => {
  const i = exec.scenario.iterationInInstance % meanRequests.length;
  let id = (exec.scenario.iterationInInstance % maxUserIdQuery) + 1;
  id = encoding.b64encode(id + "AppUser", "url");

  const res = meanRequests[i](id);

  check(res, {
    "response was denied": (r) =>
      r.status == 200 && !!r.json() && !!r.json().errors,
  });
};

export const handleSummary = (data) => handleSummaryFn(data, "mean");
