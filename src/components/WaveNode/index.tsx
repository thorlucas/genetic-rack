import React, { useEffect, useMemo, useRef, useState } from 'react';
import * as THREE from 'three';
import * as Tone from 'tone';
import { useFrame } from '@react-three/fiber';
import { Vector3 } from 'three';
import type { Sim } from '@thorlucas/genetic-wasm';

const tempVec = new Vector3();
const randVec = new Vector3();

const nPoints = 100;

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [posArray, setPosArray] = useState<Float32Array>(null!);
	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
		async function makeSim() {
			const { Sim } = await import('@thorlucas/genetic-wasm');
			const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			const sim = Sim.build()
				.max_points(nPoints)
				.max_radius(40.0)
				.min_radius(20.0)
				.max_perp_momentum(100.0)
				.min_perp_momentum(60.0)
				.point_mass(10.0)
				.large_mass(50.0)
				.point_halflife(10.0)
				.create();
			const arr_ptr = sim.points_buffer_ptr();
			const arr = new Float32Array(memory.buffer, arr_ptr, nPoints * 3);

			setSim(sim);
			setPosArray(arr);
		}

		makeSim();
	}, []);

	useFrame((state, delta) => {
		if (!sim) {
			return;
		}
		sim.tick(delta);
		pRef.current.geometry.attributes.position.needsUpdate = true;
	});

	return (
		<>
			<mesh ref={ gRef } onClick={() => (Tone.start())}>
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
