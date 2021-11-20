import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import type { Sim } from '@thorlucas/genetic-wasm';
import useDebugFPS from '@hooks/debug_fps';

const nPoints = 100;

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [posArray, setPosArray] = useState<Float32Array>(null!);
	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
		async function makeSim() {
			const { init } = await import('@thorlucas/genetic-wasm');
			const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			const sim = init({
				initial_points: 8,
				max_points: nPoints,
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

	useFrame(sim ? (_state, delta) => {
		sim.tick(delta);
		pRef.current.geometry.attributes.position.needsUpdate = true;
	} : () => {});

	useDebugFPS();

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
					<pointsMaterial color="orange" size={1.0} />
				</points>
			) : null }
		</>
	)
}

export default WaveNode;
