// Since giant doesn't have any concept of an inode we have to mantain a map of synthetic inodes to a Giant URI.

struct INodeMap {
    map: Vec<String>,
    giantUri: String,
    ingestion: String,
}

impl INodeMap {
    pub fn new(giantUri: String, ingestion: String) -> INodeMap {
        INodeMap {
            map: vec![],
            giantUri,
            ingestion,
        }
    }

    pub fn build(&mut self) {
        // walk the ingestion recursively
    }
}
