import { useEffect, useState } from "react";

/**
 * This is a very silly hook that will simply return the `before` value initially, wait
 * `time` milliseconds, and then return `after`.
 */
function useWait<T>(time: number, before: T, after: T) {
	const [value, setValue] = useState<T>(before);

	useEffect(() => {
		const timer: NodeJS.Timeout = setTimeout(() => {
			setValue(after);
		}, time);

		// This will clear the current timer in the case that either the time or after
		// changes.
		return () => {
			clearTimeout(timer);
		};
	}, [time, after]);

	return value;
}

export default useWait;
