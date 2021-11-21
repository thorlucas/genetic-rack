import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import type { Sim, BufferF32 } from '@thorlucas/genetic-wasm';
import useDebugFPS from '@hooks/debug_fps';
import { InterleavedBuffer, InterleavedBufferAttribute } from 'three';

const nPoints = 100;

type SimBufMap = {
	point: {
		position: [Float32Array, number],
	}
}

function makeBuffer(memory: WebAssembly.Memory, offset: number, items: number, width: number): SimBufMap['point']['position'] {
	let f32s = new Float32Array(memory.buffer, offset, items * width);
	return [f32s, width];
}

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [attrBufs, setAttrBufs] = useState<SimBufMap>(null!);
	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
		async function makeSim() {
			const { init } = await import('@thorlucas/genetic-wasm');
			const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			const sim = init({
				initial_points: 3,
				max_points: 3,
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

			const bufs: BufferF32[] = sim.get_buffers();
			console.log(bufs)
			for (let buf of bufs) {
				if (buf.component.type == 'point') {
					for (let attr of buf.component.attributes) {
						if (attr.name === 'position') {
							setAttrBufs({
								point: {
									position: makeBuffer(memory, buf.ptr, buf.items, attr.width)
								}
							})
						}
					}
				}
			}

			setSim(sim);
		}
		makeSim();
	}, []);

	useEffect(() => {
		console.log(attrBufs);
	}, [attrBufs]);

	useFrame(attrBufs && sim && pRef ? (_state, delta) => {
		sim.tick(delta);
		pRef.current.geometry.attributes.position.needsUpdate = true;
	} : () => {});

	useDebugFPS();

	return attrBufs ? (
		<>
			{ /*
			<mesh ref={ gRef } >
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			*/ }
			<points ref={ pRef }>
				<bufferGeometry>
					<bufferAttribute attachObject={['attributes', 'position']} args={ attrBufs.point.position } />
				</bufferGeometry>
				<pointsMaterial color="orange" size={1.0} />
			</points>
		</>
	) : null;
}

export default WaveNode;
