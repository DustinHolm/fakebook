import http from "k6/http";
import { check } from "k6";
import { healthUrl } from "./util/urls.js";
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
      exec: "smoke",
    },
  },
};

export const smoke = () => {
  const res = http.get(healthUrl);
  check(res, { "response did not contain error": (r) => r.status == 200 });
};

export const handleSummary = (data) => handleSummaryFn(data, "health");
