import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import useDebugFPS from '@hooks/debug_fps';
import { InterleavedBuffer, InterleavedBufferAttribute } from 'three';

import { init, Sim } from '@wasm';

const nPoints = 100;

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const [sim, setSim] = useState<Sim>(null!);

	useEffect(() => {
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
		setSim(sim);

		
		//async function makeSim() {
			//const { init } = await import('@thorlucas/genetic-wasm');
			//const { memory } = await import('@thorlucas/genetic-wasm/genetic_wasm_bg.wasm');
			
			//const sim = init({
				//initial_points: 3,
				//max_points: 3,
				//radius: { min: 20.0, max: 40.0 },
				//momentum: { min: 60.0, max: 100.0 },
				//lifetime: { half_life: 10.0 },
				//init_sources: [
					//{
						//position: [5.0, 0.0, 0.0],
						//mass: 5000.0,
					//}
				//],
			//});

			//const bufs: BufferF32[] = sim.get_buffers();
			//console.log(bufs)
			//for (let buf of bufs) {
				//if (buf.component.type == 'point') {
					//for (let attr of buf.component.attributes) {
						//if (attr.name === 'position') {
							//setAttrBufs({
								//point: {
									//position: makeBuffer(memory, buf.ptr, buf.items, attr.width)
								//}
							//})
						//}
					//}
				//}
			//}

			//setSim(sim);
		//}
		//makeSim();
	}, []);

	//useFrame(attrBufs && sim && pRef ? (_state, delta) => {
		//sim.tick(delta);
		//pRef.current.geometry.attributes.position.needsUpdate = true;
	//} : () => {});

	useDebugFPS();

	return false ? (
		<>
			{ /*
			<mesh ref={ gRef } >
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			
			<points ref={ pRef }>
				<bufferGeometry>
					<bufferAttribute attachObject={['attributes', 'position']} args={ attrBufs.point.position } />
				</bufferGeometry>
				<pointsMaterial color="orange" size={1.0} />
			</points>
			*/ }
		</>
	) : null;
}

export default WaveNode;
