import fetchDocument, { Document, DocumentID } from "@adapters/document";
import { useEffect, useState } from "react";

function useDocument(id: DocumentID): (Document | null) {
	const [document, setDocument] = useState<Document | null>(null);

	useEffect(() => {
		fetchDocument(id)
		.then((response: Document) => {
			setDocument(response);
		});
	}, [id]);

	return document;
}

export default useDocument;
