import React, { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import * as Tone from 'tone';
import { useFrame } from '@react-three/fiber';
import { Vector3 } from 'three';

type OrbitNode = {
	p: Vector3,
	r: Vector3,
	osc: Tone.Oscillator,
}

const freqs = ['C3', 'E3', 'G3', 'B3'];
const nOrbit = freqs.length;
const compressor = new Tone.Compressor().toDestination();

const data = Array.from(freqs, (freq) => {
	const r = new Vector3().randomDirection();
	const p = new Vector3().randomDirection().cross(r).normalize();

	const rlen = 30.0 + Math.random() * 20.0;

	const osc = new Tone.Oscillator({ type: 'sine', frequency: freq }).connect(compressor).start();
	osc.volume.value = -100;

	return {
		p: p.multiplyScalar(15.0 + Math.random() * 15.0),
		r: r.multiplyScalar(rlen),
		osc: osc,
	}
});

const tempObject = new THREE.Object3D();

const c1 = 1.0;
const c2 = 25000.0;
const minDelta = 1 / 120.0;
let lastUpdate = 0.0;

const freqAxis = new Vector3(0, 0, 1);

function dr(x: OrbitNode, dt: number): Vector3 {
	return new Vector3().copy(x.p).multiplyScalar(c1 * dt);
}

function dp(x: OrbitNode, dt: number): Vector3 {
	return new Vector3().copy(x.r).multiplyScalar((-c2 / Math.pow(x.r.length(), 3)) * dt);
}

function vol(x: OrbitNode): number {
	return -x.r.lengthSq() / 40.0;
}

function updateOrbitNode(x: OrbitNode, dt: number) {
	x.p.add(dp(x, dt));
	x.r.add(dr(x, dt));
	x.osc.volume.value = vol(x);
}


const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const oRef = useRef<THREE.InstancedMesh>(null!);

	useFrame((state) => {
		const t = state.clock.getElapsedTime();
		if (t - lastUpdate > minDelta || t === 0) {
			const dt = t - lastUpdate;
			lastUpdate = t;

			for (let i = 0; i < nOrbit; ++i) {
				updateOrbitNode(data[i], dt);

				tempObject.position.copy(data[i].r);
				tempObject.updateMatrix();
				oRef.current.setMatrixAt(i, tempObject.matrix);
				oRef.current.instanceMatrix.needsUpdate = true;
			}
		}
	});

	return (
		<>
			<mesh ref={ gRef } onClick={() => (Tone.start())}>
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
