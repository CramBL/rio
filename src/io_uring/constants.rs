#![allow(unused)]
pub const IORING_OP_NOP: u8 = 0;
pub const IORING_OP_READV: u8 = 1;
pub const IORING_OP_WRITEV: u8 = 2;
pub const IORING_OP_FSYNC: u8 = 3;
pub const IORING_OP_READ_FIXED: u8 = 4;
pub const IORING_OP_WRITE_FIXED: u8 = 5;
pub const IORING_OP_POLL_ADD: u8 = 6;
pub const IORING_OP_POLL_REMOVE: u8 = 7;
pub const IORING_OP_SYNC_FILE_RANGE: u8 = 8;
pub const IORING_OP_SENDMSG: u8 = 9;
pub const IORING_OP_RECVMSG: u8 = 10;
pub const IORING_OP_TIMEOUT: u8 = 11;
pub const IORING_OP_TIMEOUT_REMOVE: u8 = 12;
pub const IORING_OP_ACCEPT: u8 = 13;
pub const IORING_OP_ASYNC_CANCEL: u8 = 14;
pub const IORING_OP_LINK_TIMEOUT: u8 = 15;
pub const IORING_OP_CONNECT: u8 = 16;
pub const IORING_OP_FALLOCATE: u8 = 17;
pub const IORING_OP_OPENAT: u8 = 18;
pub const IORING_OP_CLOSE: u8 = 19;
pub const IORING_OP_FILES_UPDATE: u8 = 20;
pub const IORING_OP_STATX: u8 = 21;
pub const IORING_OP_READ: u8 = 22;
pub const IORING_OP_WRITE: u8 = 23;
pub const IORING_OP_FADVISE: u8 = 24;
pub const IORING_OP_MADVISE: u8 = 25;
pub const IORING_OP_SEND: u8 = 26;
pub const IORING_OP_RECV: u8 = 27;
pub const IORING_OP_OPENAT2: u8 = 28;
pub const IORING_OP_EPOLL_CTL: u8 = 29;
pub const IORING_OP_LAST: u8 = 30;
pub const IOSQE_FIXED_FILE: u8 = 1;
pub const IOSQE_IO_DRAIN: u8 = 2;
pub const IOSQE_IO_LINK: u8 = 4;
pub const IOSQE_IO_HARDLINK: u8 = 8;
pub const IOSQE_ASYNC: u8 = 16;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_SETUP_CQSIZE: u32 = 8;
pub const IORING_SETUP_CLAMP: u32 = 16;
pub const IORING_FSYNC_DATASYNC: u8 = 1;
pub const IORING_TIMEOUT_ABS: u32 = 1;
pub const IORING_OFF_SQ_RING: i64 = 0;
pub const IORING_OFF_CQ_RING: i64 = 0x0800_0000;
pub const IORING_OFF_SQES: i64 = 0x1000_0000;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_FEAT_NODROP: u32 = 2;
pub const IORING_FEAT_SUBMIT_STABLE: u32 = 4;
pub const IORING_FEAT_RW_CUR_POS: u32 = 8;
pub const IORING_REGISTER_BUFFERS: u32 = 0;
pub const IORING_UNREGISTER_BUFFERS: u32 = 1;
pub const IORING_REGISTER_FILES: u32 = 2;
pub const IORING_UNREGISTER_FILES: u32 = 3;
pub const IORING_REGISTER_EVENTFD: u32 = 4;
pub const IORING_UNREGISTER_EVENTFD: u32 = 5;
pub const IORING_REGISTER_FILES_UPDATE: u32 = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: u32 = 7;
