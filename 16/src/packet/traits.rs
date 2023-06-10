pub trait Packet {
    fn version(&self) -> u8;
    fn id(&self) -> u8;
    fn compute(&self) -> u64;
    fn len(&self) -> usize;
    fn children<'a>(&'a self) -> &'a [Box<dyn Packet + 'a>];

    fn all_versions(&self) -> Vec<u8> {
        [
            vec![self.version()],
            self.children()
                .iter()
                .map(|c| c.all_versions())
                .reduce(|acc, e| [acc, e].concat())
                .unwrap_or_default(),
        ]
        .concat()
    }
}
