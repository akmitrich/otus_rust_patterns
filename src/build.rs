#![allow(unused, dead_code)]

pub struct Block {
    name: String,
    data: Vec<u8>,
    flags: u64,
    connection: String, //mock connection
}

#[derive(Default)]
pub struct Builder {
    name: String,
    data: Vec<u8>,
    flags: u64,
    connection: Option<String>,
}

impl Block {
    pub fn builder() -> Builder {
        Default::default()
    }
}

impl Builder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn data(mut self, data: &[u8]) -> Self {
        self.data = Vec::from(data);
        self
    }

    pub fn flags(mut self, flags: u64) -> Self {
        self.flags = flags;
        self
    }

    pub fn connection(mut self, conn: &str) -> Self {
        self.connection = Some(conn.into());
        self
    }

    pub fn build(self) -> Option<Block> {
        self.connection.map(|connection| Block {
            name: self.name,
            data: self.data,
            flags: self.flags,
            connection,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_builder() {
        let b = Block::builder();
        assert_eq!("", b.name);
        assert!(b.data.is_empty());
        assert_eq!(0, b.flags);
        assert_eq!(None, b.connection);
    }

    #[test]
    fn test_failed_build() {
        assert!(Block::builder().build().is_none());
        let b = Block::builder();
        let block = b.name("Failure").data(&[42, 0, 127]).flags(1 << 4).build();
        assert!(block.is_none());
    }

    #[test]
    fn test_build() {
        let b = Block::builder();
        let block = b
            .name("Success")
            .data(&[42])
            .flags(1 << 3)
            .connection("sqlite database")
            .build()
            .unwrap();
        assert_eq!("Success", block.name);
        assert_eq!(&42, block.data.get(0).unwrap());
        assert_eq!(8, block.flags);
        assert_eq!("sqlite database", block.connection);
    }
}
