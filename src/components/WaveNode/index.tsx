import React, { useEffect, useMemo, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import { Vector3 } from 'three';
import type { Sim } from '@thorlucas/genetic-wasm';

const tempVec = new Vector3();
const randVec = new Vector3();

const nPoints = 100;

var frameCount: number = 0;
var timeSinceFPSDisplay: number = 0;

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [posArray, setPosArray] = useState<Float32Array>(null!);
	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
		async function makeSim() {
			const { init, Sim, Opts } = await import('@thorlucas/genetic-wasm');
			const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			const sim = init({
				initial_points: 100,
				radius: { min: 20.0, max: 40.0 },
				momentum: { min: 60.0, max: 100.0 },
				lifetime: { half_life: 10.0 },
			});

			const pos_ptr = sim.positions_buffer_ptr();
			const pos = new Float32Array(memory.buffer, pos_ptr, nPoints * 3);

			setSim(sim);
			setPosArray(pos);
		}

		makeSim();
	}, []);

	useFrame((state, delta) => {
		if (!sim) {
			return;
		}
		timeSinceFPSDisplay += delta;
		frameCount += 1;

		if (timeSinceFPSDisplay >= 1.0) {
			console.log(`FPS: ${frameCount / timeSinceFPSDisplay}`);
			frameCount = 0;
			timeSinceFPSDisplay = 0;
		}
		
		sim.tick(delta);
		pRef.current.geometry.attributes.position.needsUpdate = true;
	});

	return (
		<>
			<mesh ref={ gRef } >
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			{ posArray ? (
				<points ref={ pRef }>
					<bufferGeometry>
						<bufferAttribute attachObject={['attributes', 'position']} args={[posArray, 3]}/>
					</bufferGeometry>
					<pointsMaterial color="orange" size={0.3} />
				</points>
			) : null }
		</>
	)
}

export default WaveNode;
