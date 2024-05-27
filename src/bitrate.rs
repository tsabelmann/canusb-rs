#[derive(Debug)]
pub enum Bitrate {
    Bitrate10K,
    Bitrate20K,
    Bitrate50K,
    Bitrate100K,
    Bitrate125K,
    Bitrate250K,
    Bitrate500K,
    Bitrate800K,
    Bitrate1M,
    Btr {
        btr0: u8,
        btr1: u8
    }
}