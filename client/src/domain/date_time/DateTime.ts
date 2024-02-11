import { intlFormat, isValid, parseISO } from "date-fns";

export const DateTime = Object.freeze({
  parse: function (value: unknown) {
    if (typeof value !== "string") throw Error("Invalid date!");
    const parsed = parseISO(value);
    if (!isValid(parsed)) throw Error("Invalid date!");
    return parsed;
  },
  format: function (value: Date) {
    return intlFormat(value, {
      year: "numeric",
      month: "numeric",
      day: "numeric",
      hour: "numeric",
      minute: "numeric",
    });
  },
});
