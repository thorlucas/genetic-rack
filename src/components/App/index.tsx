import Box from '@components/Box';
import { Canvas } from '@react-three/fiber';
import React from 'react';

const App: React.FC = () => {
	return (
		<Canvas mode="concurrent">
			<ambientLight />
			<pointLight position={ [10, 10, 10] } />
			<Box />
		</Canvas>
	);
}

export default App;
