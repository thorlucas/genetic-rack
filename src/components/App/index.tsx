import React from 'react';

import lightning from '@assets/lightning.svg';
import Button from '@UI/Button';
import { MyContext } from '@contexts';
import { useWait } from '@hooks';
import { Document } from '@adapters/document';
import useDocument from '@hooks/document';

const App: React.FC = () => {
	// This is an example of the usage of a custom hook. We can extract logic this way.
	// This value will be null initially, then become "Hello again, world!" after 2
	// seconds.
	const wait: string | null = useWait(2000, null, "Hello again, world!");

	// This is a hook that calls an adapter which then fetches a document for us using
	// an API. The document will be made available after one second.
	const document: Document | null = useDocument('dummy');

	return (
	// This is a context provider. All subsequent components are able to use this
	// context value by either using a <MyContext.Consumer> element, or by using a
	// useContext hook.
	<MyContext.Provider value="Hello, world!">

		<div className="w-100 h-screen flex justify-center items-center">
			<div className="px-12 py-8 bg-white rounded-lg shadow-md">
				<img src={ lightning } alt="Lightning!" className="h-12 mb-4 mx-auto"/>

				<h1 className="mb-2 text-3xl font-semibold">
					Hello, world!
				</h1>

				<div className="mb-4">

					<p>Welcome to my React Application!</p>
					
					{ /* Here we are using a Consumer to use the context value. */ }
					<MyContext.Consumer>
						{ value => (
							<p>{ value }</p>
						)}
					</MyContext.Consumer>

					{ /* Here we are conditionally rendering only if wait is not null. */ }
					{ wait ? (
						<p>{ wait }</p>
					) : null }

				</div>

				{ document ? (
					<div className="my-4 py-4 px-3 bg-gray-100 rounded-md shadow-md">
						<h2 className="mb-2 text-2xl font-semibold">{ document.title }</h2>
						<p>{ document.body }</p>
					</div>
				) : null }

				<div>
					{ /* This is an example button component */ }
					<Button role='primary'>
						Hello!
					</Button>
				</div>

			</div>
		</div>

	</MyContext.Provider>
	);
}

export default App;
