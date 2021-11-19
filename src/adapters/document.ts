export type DocumentID = string;
export type Document = {
	id: DocumentID,
	title: string,
	body: string,
}

const dummyDocument: Document = {
	id: 'dummy',
	title: "My Document Title",
	body: "Lorem Ipsum...",
};

/**
 * This is a silly dummy adapter. Pretend it wraps an API that fetches a document.
 * Instead we're just returning a dummy document after 1 second.
 */
function fetchDocument(id: DocumentID): Promise<Document> {
	return new Promise<Document>((resolve) => {
		setTimeout(() => resolve(dummyDocument), 1000);
	});
}

export default fetchDocument;
