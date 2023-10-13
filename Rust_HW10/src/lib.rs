fn make_document(text: &str) -> String {
    let paragraph: Vec<&str> = text.split("\n\n").collect();

    let mut document = String::new();

    for para in paragraph {
        document.push_str("\t");
        document.push_str(para);
        document.push_str("\n\n");
    }
    document
}

fn rank_documents(docs: Vec<String>) -> Vec<String> {
    let mut docs_with_para_count: Vec<(usize, String)> = Vec::new();

    for doc in docs {
        let para_count = doc.split("\n\n").count();
        docs_with_para_count.push((para_count, doc));
    }

    docs_with_para_count.sort_by(|a, b| b.0.cmp(&a.0));

    docs_with_para_count.into_iter().map(|(_, doc)| doc).collect()
}
    
    

#[test]
fn test_documents() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}


