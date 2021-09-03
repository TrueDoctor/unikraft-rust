use crate::fs::Node;
extern crate alloc;
use alloc::vec;

macro_rules! fs {
    () => {
        unsafe {
            match &mut ROOTFS {
                Some(rootfs) => rootfs,
                None => return -1,
            }
        }
    };
}

fn cs_to_slice(base: *const u8) -> &'static str {
    unsafe {
        let len = (base as usize..).position(|c| *(c as *const u8) == 0);
        let slice = core::slice::from_raw_parts(base, len.unwrap());
        core::str::from_utf8(slice).expect("Passed not utf-8 string")
    }
}

#[no_mangle]
pub extern "C" fn rustfs_mount() -> i32 {
    unsafe {
        ROOTFS = Some(Node::Directory(vec![]));
    }
    0
}

#[no_mangle]
pub extern "C" fn rustfs_unmount() -> i32 {
    unsafe {
        ROOTFS = None;
    }
    0
}

#[no_mangle]
pub extern "C" fn rustfs_create(ptr: &mut &Node, path: *const u8) -> i32 {
    let path = cs_to_slice(path);
    match fs!().create(path) {
        None => -1,
        Some(node) => {
            *ptr = node;
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn rustfs_remove(ptr: &Node) -> i32 {
    match fs!().remove(ptr) {
        Some(_) => 0,
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn rustfs_lookup(ptr: &mut &Node, path: *const u8) -> i32 {
    let path = cs_to_slice(path);
    match fs!().lookup(path) {
        Some(vnode) => {
            *ptr = vnode;
            0
        }
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn rustfs_read(node: &Node, uio: &mut UIO) -> i32 {
    match node {
        Node::Vnode(vn) => {
            unsafe { *uio.uio_iov = IOVec::from_slice(vn.read()) };
            0
        }
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn rustfs_write(node: &mut Node, uio: &mut UIO) -> i32 {
    match node {
        Node::Vnode(vn) => {
            vn.write(uio);
            0
        }
        _ => -1,
    }
}
/*
#[repr(C)]
struct VNode {
    v_ino: u64		/* inode number */
    struct uk_list_head v_link;	/* link for hash list */
    struct mount	*v_mount;	/* mounted vfs pointer */
    struct vnops	*v_op;		/* vnode operations */
    v_refcnt: i32;	/* reference count */
    v_type: i32;		/* vnode type */
    v_flags: i32;	/* vnode flag */
    v_mode: u32;		/* file mode */
    v_size: u64;		/* file size */
    struct uk_mutex	v_lock;		/* lock for this vnode */
    struct uk_list_head v_names;	/* directory entries pointing at this */
    void		*v_data;	/* private data for fs */
};
*/

static mut ROOTFS: Option<Node> = None;

#[repr(C)]
pub struct IOVec {
    base: *const u8,
    len: usize,
}
impl IOVec {
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.base, self.len) }
    }
    fn from_slice(data: &[u8]) -> Self {
        Self {
            base: data.as_ptr(),
            len: data.len(),
        }
    }
}

#[repr(C)]
pub struct UIO {
    uio_iov: *mut IOVec, /* scatter/gather list */
    uio_iovcnt: i32,     /* length of scatter/gather list */
    uio_offset: u64,     /* offset in target object */
    uio_resid: isize,    /* remaining bytes to process */
    uio_rw: u32,         /* operation */
}
impl UIO {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { (*self.uio_iov).as_slice() }
    }
}
