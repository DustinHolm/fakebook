import exec from "k6/execution";
import encoding from "k6/encoding";
import { check } from "k6";
import { UserPosts, UserPostsComments } from "./util/requests.js";
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
    mixed_requests_spike: {
      executor: "constant-vus",
      vus: 100,
      duration: "20s",
      exec: "mixed",
      env: { PAGINATION: "3" },
    },
  },
};

const maxUserIdQuery = 5000;

const mixedRequests = [UserPosts, UserPostsComments];
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

export const handleSummary = (data) => handleSummaryFn(data, "postComments");
