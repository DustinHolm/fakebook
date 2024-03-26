import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.2/index.js";

export const handleSummaryFn = (data, suffix) => {
  delete data.metrics["http_req_duration{expected_response:true}"];

  for (const key in data.metrics) {
    if (key.startsWith("data")) delete data.metrics[key];
    if (key.startsWith("iteration")) delete data.metrics[key];
    if (key.startsWith("vus")) delete data.metrics[key];
  }

  const outputFile = `./results/last_run_${suffix}.txt`;

  return {
    stdout: textSummary(data),
    [outputFile]: textSummary(data, {
      indent: "  ",
      enableColors: false,
    }),
  };
};
