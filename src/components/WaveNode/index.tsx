import React, { useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';
import { Vector3 } from 'three';

type OrbitState = {
	p: Vector3,
	r: Vector3,
}

const nOrbit = 4;

const data = Array.from({ length: nOrbit }, () => {
	const r = new Vector3().randomDirection();
	const p = new Vector3().randomDirection().cross(r).normalize();
	return {
		p: p.multiplyScalar(15.0 + Math.random() * 15.0),
		r: r.multiplyScalar(30.0 + Math.random() * 20.0),
	}
});

const tempObject = new THREE.Object3D();

const c1 = 1.0;
const c2 = 25000.0;
const minDelta = 1 / 120.0;
let lastUpdate = 0.0;

function dr(x: OrbitState, dt: number): Vector3 {
	return new Vector3().copy(x.p).multiplyScalar(c1 * dt);
}

function dp(x: OrbitState, dt: number): Vector3 {
	return new Vector3().copy(x.r).multiplyScalar((-c2 / Math.pow(x.r.length(), 3)) * dt);
}

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const oRef = useRef<THREE.InstancedMesh>(null!);

	console.log(data);

	useFrame((state) => {
		const t = state.clock.getElapsedTime();
		if (t - lastUpdate > minDelta || t === 0) {
			const dt = t - lastUpdate;
			lastUpdate = t;

			for (let i = 0; i < nOrbit; ++i) {
				data[i].p.add(dp(data[i], dt));
				data[i].r.add(dr(data[i], dt));

				tempObject.position.copy(data[i].r);
				tempObject.updateMatrix();
				oRef.current.setMatrixAt(i, tempObject.matrix);
				oRef.current.instanceMatrix.needsUpdate = true;
			}
		}
	});

	return (
		<>
			<mesh ref={ gRef }>
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			<instancedMesh ref={ oRef } args={ [undefined, undefined, nOrbit] }>
				<sphereGeometry args={ [0.5, 16, 8] } />
				<meshStandardMaterial color="orange" />
			</instancedMesh>
		</>
	)
}

export default WaveNode;
