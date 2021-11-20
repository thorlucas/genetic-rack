import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import type { Sim } from '@thorlucas/genetic-wasm';
import useDebugFPS from '@hooks/debug_fps';
import { InterleavedBuffer, InterleavedBufferAttribute } from 'three';

const nPoints = 100;

type AttrDesc = {
	size: number,
	stride: number,
	offset: number,
}

type BufferDesc = {
	buffer: Float32Array,
	items: number,
	attrs: {
		[key: string]: AttrDesc,
	}
}

type SimData = {
	points: BufferDesc,
	sources: BufferDesc,
}

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [simData, setSimData] = useState<SimData>(null!);
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
				init_sources: [
					{
						position: [5.0, 0.0, 0.0],
						mass: 5000.0,
					}
				],
			});
			
			setSim(sim);

			const pos_desc = sim.point_pos_buffer();

			// TODO: Within rust lets specify somehow that these are together and should
			// be interleaved.
			const src_pos_desc = sim.source_pos_buffer();
			const src_mass_desc = sim.source_mass_buffer();

			setSimData({
				points: {
					buffer: new Float32Array( memory.buffer, pos_desc.buffer_ptr, pos_desc.items * pos_desc.stride ),
					items: pos_desc.items,
					attrs: {
						position: {
							size: 3,
							stride: pos_desc.stride,
							offset: pos_desc.offset,
						},
					},
				},
				sources: {
					buffer: new Float32Array( memory.buffer, src_pos_desc.buffer_ptr, src_pos_desc.items * src_pos_desc.stride ),
					items: src_pos_desc.items,
					attrs: {
						position: {
							size: 3,
							stride: src_pos_desc.stride,
							offset: src_pos_desc.offset,
						},
						mass: {
							size: 1,
							stride: src_mass_desc.stride,
							offset: src_mass_desc.offset,
						},
					},
				},
			});
		}
		makeSim();
	}, []);

	useFrame(sim ? (_state, delta) => {
		sim.tick(delta);
		pRef.current.geometry.attributes.position.needsUpdate = true;
	} : () => {});

	useDebugFPS();

	return simData ? (
		<>
			{ /*
			<mesh ref={ gRef } >
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			*/ }
			<points ref={ pRef }>
				<bufferGeometry>
					<bufferAttribute
						attachObject={[ 'attributes', 'position' ]}
						array={ simData.points.buffer }
						count={ simData.points.items }
						itemSize={ 3 }
						>
					</bufferAttribute>
				</bufferGeometry>
				<pointsMaterial color="orange" size={1.0} />
			</points>
		</>
	) : null;
}

export default WaveNode;
