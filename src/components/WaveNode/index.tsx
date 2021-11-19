import React, { useEffect, useMemo, useRef, useState } from 'react';
import * as THREE from 'three';
import * as Tone from 'tone';
import { useFrame } from '@react-three/fiber';
import { Vector3 } from 'three';

const tempVec = new Vector3();
const randVec = new Vector3();

const WaveNode: React.FC = () => {
	const gRef = useRef<THREE.Mesh>(null!)
	const pRef = useRef<THREE.Points>(null!)

	const posArray = useMemo(() => {
		return Float32Array.from(
			new Array(1000).fill(0).flatMap((_, i) => {
				return new Vector3().randomDirection().multiplyScalar(Math.random()*50.0).toArray();
			})
		);
	}, []);


	useFrame((state) => {
		for (let i = 0; i < 1000; ++i) {
			randVec.randomDirection().multiplyScalar(0.1);
			tempVec.fromArray(posArray, i * 3);
			tempVec.add(randVec);
			tempVec.toArray(posArray, i * 3);
		}
		pRef.current.geometry.attributes.position.needsUpdate = true;
	});

	return (
		<>
			<mesh ref={ gRef } onClick={() => (Tone.start())}>
				<sphereGeometry args={ [2.0, 32, 16] } />
				<meshStandardMaterial color="hotpink" />
			</mesh>
			<points ref={ pRef }>
				<bufferGeometry>
					<bufferAttribute attachObject={['attributes', 'position']} args={[posArray, 3]}/>
				</bufferGeometry>
				<pointsMaterial color="orange" size={0.3} />
			</points>
		</>
	)
}

export default WaveNode;
