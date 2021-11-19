import React, { useRef, useState } from 'react';
import * as THREE from 'three';
import { useFrame } from '@react-three/fiber';

const Box: React.FC = () => {
	// This reference will give us direct access to the THREE.Mesh object
	const ref = useRef<THREE.Mesh>(null!)
	// Hold state for hovered and clicked events
	const [hovered, hover] = useState(false)
	const [clicked, click] = useState(false)
	// Rotate mesh every frame, this is outside of React without overhead
	useFrame((state, delta) => (ref.current.rotation.x += 0.01))

	return (
		<mesh
			ref={ref}
			scale={clicked ? 1.5 : 1}
			onClick={() => click(!clicked)}
			onPointerOver={() => hover(true)}
			onPointerOut={() => hover(false)}>
			<boxGeometry args={ [1, 1, 1] } />
			<meshStandardMaterial color={hovered ? 'hotpink' : 'orange'} />
		</mesh>
	)
}

export default Box;
