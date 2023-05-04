Dynamic indexing is a technique used in full text search to efficiently search large volumes of text data. The basic idea behind dynamic indexing is to build an index on the fly as search queries are executed, rather than precomputing an index for the entire document collection ahead of time. This allows for faster and more efficient searches, as only the relevant portions of the index need to be created and maintained.

Here is a high-level algorithm for dynamic indexing in full text search:

1. Receive a search query from the user.
2. Parse the query to extract search terms (e.g. by splitting on whitespace and removing stop words).
3. For each search term, look up the term in the index.
4. If the term is not in the index, create a new posting list for the term and add it to the index.
5. For each document containing the term, add the document to the posting list with the frequency of the term in the document.
6. Rank the documents based on their relevance to the search query (e.g. using a scoring function like TF-IDF).
7. Return the top-ranked documents to the user.

The key advantage of dynamic indexing is that it can handle large and constantly changing document collections without requiring significant upfront processing. The downside is that it can be slower than precomputed indexing for certain use cases, as the index must be created and updated at search time. However, with efficient algorithms and data structures, dynamic indexing can still provide fast and scalable full text search for many applications.