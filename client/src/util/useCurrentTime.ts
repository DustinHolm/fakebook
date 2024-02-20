import { add, minutesToMilliseconds } from "date-fns";
import { useEffect, useRef, useState } from "react";

type CurrentTimePrecision = "minutes";

function getTimeout(precision: CurrentTimePrecision): number {
  switch (precision) {
    case "minutes":
      return minutesToMilliseconds(1);
  }
}

export function useCurrentTime(precision: CurrentTimePrecision) {
  const [currentTime, setCurrentTime] = useState(new Date());
  const timer = useRef<number>();

  useEffect(() => {
    timer.current = window.setInterval(() => {
      setCurrentTime((old) => add(old, { [precision]: 1 }));
    }, getTimeout(precision));

    return () => {
      if (timer.current !== undefined) {
        window.clearInterval(timer.current);
      }
    };
  }, [precision]);

  return currentTime;
}
