import WaveNode from '@components/WaveNode';
import { Canvas } from '@react-three/fiber';
import React, { useEffect } from 'react';

const App: React.FC = () => {

	return (
		<Canvas
			mode="concurrent"
		
			camera={{ position: [0.0, 0.0, 100.0] }}>
			<ambientLight />
			<WaveNode />
		</Canvas>
	);
}

export default App;
