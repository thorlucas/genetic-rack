import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import type { Sim, BufferF32 } from '@thorlucas/genetic-wasm';
import useDebugFPS from '@hooks/debug_fps';
import { InterleavedBuffer, InterleavedBufferAttribute } from 'three';

const nPoints = 10000;

type SimBufMap = {
	point: {
		position: [Float32Array, number],
	}
}

function makeBuffer(memory: WebAssembly.Memory, offset: number, items: number, width: number): SimBufMap['point']['position'] {
	let f32s = new Float32Array(memory.buffer, offset, items * width);
	return [f32s, width];
}

type InitSource = {
	position: THREE.Vector3,
	mass: number,
}

const numInitSources = 20;

let initSources: InitSource[] = Array.from({length: numInitSources}, (_, i: number) => {
	console.log(i);
	return {
		position: new THREE.Vector3().randomDirection().multiplyScalar(Math.sqrt(Math.random())*100.0),
		mass: Math.pow(Math.random(), 3.0)*10000.0,
	};
});

let avgSourcePos = new THREE.Vector3();
initSources.forEach((src) => avgSourcePos.add(src.position));
avgSourcePos.divideScalar(numInitSources);

initSources.forEach((src) => src.position.sub(avgSourcePos));

const WaveNode: React.FC = () => {
	const pRef = useRef<THREE.Points>(null!)

	const [attrBufs, setAttrBufs] = useState<SimBufMap>(null!);
	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
		async function makeSim() {
			const { init } = await import('@thorlucas/genetic-wasm');
			const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			const sim = init({
				initial_points: nPoints,
				max_points: nPoints,
				radius: { max: 150.0 },
				momentum: { max: 1000.0 },
				lifetime: 10000000.0,
				point_mass: 100.0,
				init_sources: initSources.map((src) => { return {
					position: src.position.toArray(),
					mass: src.mass,
				}})
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

	const sourceMeshes = initSources.map((src: InitSource) => (
		<mesh position={ src.position}>
			<sphereGeometry args={ [Math.pow(src.mass / 100, 1/3), 32, 16] } />
			<meshStandardMaterial color="hotpink" />
		</mesh>
	));

	return attrBufs ? (
		<>
			<ambientLight intensity={0.05} />
			<pointLight position={ new THREE.Vector3(100.0, 200.0, 50.0) } intensity={1} distance={1000.0} decay={4}/> 

			{ sourceMeshes }
			
			<points ref={ pRef }>
				<bufferGeometry>
					<bufferAttribute attachObject={['attributes', 'position']} args={ attrBufs.point.position } />
				</bufferGeometry>
				<pointsMaterial color="orange" size={0.3} />
			</points>
		</>
	) : null;
}

export default WaveNode;
