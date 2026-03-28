#![allow(dead_code)]
#![deny(unused_assignments)]

enum Opts {
    None,
    Map(i32),
    Cgroup(i32, i32),
}

#[derive(Default)]
struct LinkInfo {
    map_fd: i32,
    order: i32,
    cgroup_fd: i32,
}

fn opaque_use<T>(ptr: *mut T, len: usize) {
    std::hint::black_box((ptr, len));
}

fn used_via_raw_pointer_call_escape(opts: Opts) {
    let mut linkinfo = LinkInfo::default();
    let (linkinfo_ptr, linkinfo_len) = match &opts {
        Opts::None => (std::ptr::null_mut(), 0),
        Opts::Map(_) | Opts::Cgroup(_, _) => (&raw mut linkinfo, std::mem::size_of::<LinkInfo>()),
    };

    match opts {
        Opts::Map(fd) => {
            linkinfo.map_fd = fd;
        }
        Opts::Cgroup(order, fd) => {
            linkinfo.order = order;
            linkinfo.cgroup_fd = fd;
        }
        Opts::None => {}
    }

    opaque_use(linkinfo_ptr, linkinfo_len);
}

fn unused_without_raw_pointer_escape(opts: Opts) {
    let mut linkinfo = LinkInfo::default();
    let (_linkinfo_ptr, _linkinfo_len) = match &opts {
        Opts::None => (std::ptr::null_mut(), 0),
        Opts::Map(_) | Opts::Cgroup(_, _) => (&raw mut linkinfo, std::mem::size_of::<LinkInfo>()),
    };

    match opts {
        Opts::Map(fd) => {
            linkinfo.map_fd = fd;
            //~^ ERROR value assigned to `linkinfo` is never read
        }
        Opts::Cgroup(order, fd) => {
            linkinfo.order = order;
            //~^ ERROR value assigned to `linkinfo` is never read
            linkinfo.cgroup_fd = fd;
            //~^ ERROR value assigned to `linkinfo` is never read
        }
        Opts::None => {}
    }
}

fn main() {
    used_via_raw_pointer_call_escape(Opts::Cgroup(1, 2));
    unused_without_raw_pointer_escape(Opts::Cgroup(1, 2));
}
