import { useFrame } from "@react-three/fiber";
import { useEffect, useState } from "react";

var exists: boolean = false;
var frameCount: number = 0;
var timeAcc: number = 0;

const defaultParams = {
	every: 1.0,
	stateful: false,
};

function useDebugFPS({
	every = defaultParams.every,
	stateful = defaultParams.stateful,
}: {
	every?: number,
	stateful?: boolean
} = defaultParams): number | null {
	useEffect(() => {
		if (exists) {
			console.error("Cannot have multiple debug FPS hooks simultaneously!");
			return;
		}
		exists = true;
		return () => { exists = false; };
	});

	const [fps, setFps] = useState<number | null>(null);

	useFrame((_state, delta) => {
		timeAcc += delta;
		frameCount += 1;

		if (timeAcc > every) {
			console.debug(`FPS: ${frameCount / timeAcc}`);
			timeAcc = (frameCount = 0.0);

			if (stateful) {
				setFps(timeAcc);
			}
		}
	});

	return fps;
}

export default useDebugFPS;
