use std::collections::HashMap;

type PostingList = HashMap<usize, usize>; // Document ID -> Term frequency

struct Index {
    index: HashMap<String, PostingList>, // Term -> Posting list
}

impl Index {
    fn new() -> Self {
        Index {
            index: HashMap::new(),
        }
    }

    fn search(&mut self, query: &str) -> Vec<usize> {
        let mut doc_scores = HashMap::new(); // Document ID -> Score

        // Parse query into search terms
        let terms: Vec<&str> = query.split_whitespace().collect();

        // Iterate over search terms
        for term in terms {
            // Look up term in index, or create new posting list
            let posting_list = self.index.entry(term.to_lowercase()).or_insert_with(HashMap::new);

            // Iterate over documents in posting list
            for (doc_id, freq) in posting_list.iter() {
                // Update document score based on term frequency
                let score = doc_scores.entry(*doc_id).or_insert(0);
                *score += freq;
            }
        }

        // Sort documents by score and return top N
        let mut doc_scores: Vec<(usize, usize)> = doc_scores.into_iter().collect();
        doc_scores.sort_by(|a, b| b.1.cmp(&a.1));
        doc_scores.iter().take(10).map(|&(doc_id, _)| doc_id).collect()
    }

    fn index_doc(&mut self, doc_id: usize, text: &str) {
        // Parse document into terms
        let terms: Vec<String> = text.split_whitespace().map(|t| t.to_lowercase()).collect();

        // Compute term frequencies
        let mut term_freqs = HashMap::new();
        for term in terms {
            let freq = term_freqs.entry(term).or_insert(0);
            *freq += 1;
        }

        // Update index with new terms and document ID
        for (term, freq) in term_freqs {
            let posting_list = self.index.entry(term).or_insert_with(HashMap::new);
            posting_list.insert(doc_id, freq);
        }
    }
}

fn main() {
    // Create index and index some documents
    let mut index = Index::new();
    index.index_doc(0, "Rust is a programming language");
    index.index_doc(1, "Python is a popular language for data science");
    index.index_doc(2, "Java is widely used in enterprise applications");

    // Search for documents containing "language"
    let results = index.search("language");

    // Print top search results
    println!("Search results:");
    for doc_id in results {
        println!("Doc {}", doc_id);
    }
}
