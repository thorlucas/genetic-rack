import WaveNode from '@components/WaveNode';
import { Canvas } from '@react-three/fiber';
import { Bloom, EffectComposer, SSAO } from '@react-three/postprocessing';
import React, { useEffect } from 'react';

const App: React.FC = () => {

	return (
		<Canvas
			mode="concurrent"
		
			camera={{ position: [0.0, 0.0, 200.0] }}>
			<WaveNode />
			<EffectComposer>
				<Bloom luminanceThreshold={0.1} intensity={ 0.3 }/>
				<SSAO />
			</EffectComposer>
		</Canvas>
	);
}

export default App;
