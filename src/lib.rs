#![no_std]

// misc
#[repr(transparent)]
pub struct LiminePtr<T>(*const T);

impl<T> LiminePtr<T> {
    const DEFAULT: LiminePtr<T> = Self(core::ptr::null_mut() as *const T);

    fn raw_get(&self) -> *const T {
        self.0
    }

    pub fn get(&self) -> Option<&'static T> {
        let raw_ptr = self.raw_get();

        if raw_ptr.is_null() {
            None
        } else {
            unsafe { Some(&*raw_ptr) }
        }
    }
}

impl LiminePtr<char> {
    // todo: create a to_string() helper function to convert the null terminated
    // string to a rust string.
}

// maker trait implementations for limine ptr
unsafe impl<T> Sync for LiminePtr<T> {}

/// Used to create the limine request struct.
macro_rules! make_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident: [$id1:expr, $id2:expr] => {
            $($(#[$field_meta:meta])* $field_name:ident : $field_ty:ty = $field_default:expr),*
        };
    ) => {
        $(#[$meta])*
        #[repr(C)]
        pub struct $name {
            id: [u64; 4],
            revision: u64,

            pub $($field_name: $field_ty),*
        }

        impl $name {
            pub const ID: [u64; 4] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b, $id1, $id2];

            pub const fn new(revision: u64) -> Self {
                Self {
                    id: Self::ID,
                    revision,

                    $($field_name: $field_default),*
                }
            }

            $($(#[$field_meta])* pub const fn $field_name(mut self, value: $field_ty) -> Self {
				self.$field_name = value;
				self
			})*
        }
    };
}

// misc structures:

pub struct LimineUuid {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: [u8; 8],
}

pub struct LimineFile {
    pub revision: u64,
    pub base: LiminePtr<u8>,
    pub length: u64,
    pub path: LiminePtr<char>,
    pub cmdline: LiminePtr<char>,
    pub partition_index: u64,
    pub unused: u32,
    pub tftp_ip: u32,
    pub tftp_port: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: LimineUuid,
    pub gpt_part_uuid: LimineUuid,
    pub part_uuid: LimineUuid,
}

// boot info request tag:
pub struct LimineBootInfoResponse {
    pub revision: u64,

    pub name: LiminePtr<char>,
    pub version: LiminePtr<char>,
}

make_struct!(
    struct LimineBootInfoRequest: [0xf55038d8e2a1202f, 0x279426fcf5f59740] => {
        response: LiminePtr<LimineBootInfoResponse> = LiminePtr::DEFAULT
    };
);

// stack size request tag:
#[repr(C)]
pub struct LimineStackSizeResponse {
    pub revision: u64,
}

make_struct!(
    struct LimineStackSizeRequest: [0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d] => {
        response: LiminePtr<LimineStackSizeResponse> = LiminePtr::DEFAULT,
        stack_size: u64 = 0
    };
);

// HHDM request tag:
#[repr(C)]
pub struct LimineHhdmResponse {
    pub revision: u64,
    pub offset: u64,
}

make_struct!(
    struct LimineHhdmRequest: [0x48dcf1cb8ad2b852, 0x63984e959a98244b] => {
        response: LiminePtr<LimineHhdmResponse> = LiminePtr::DEFAULT
    };
);

// framebuffer request tag:
#[repr(C)]
pub struct LimineFramebuffer {
    pub address: LiminePtr<u8>,
    pub width: u16,
    pub height: u16,
    pub pitch: u16,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: u8,
    pub edid_size: u64,
    pub edid: LiminePtr<u8>,
}

#[repr(C)]
pub struct LimineFramebufferResponse {
    pub revision: u64,
    pub framebuffer_count: u64,
    // todo: add a helper function to convert the limine framebuffer array to a rust array.
    pub framebuffers: LiminePtr<LimineFramebuffer>,
}

make_struct!(
    struct LimineFramebufferRequest: [0xcbfe81d7dd2d1977, 0x063150319ebc9b71] => {
        response: LiminePtr<LimineFramebufferResponse> = LiminePtr::DEFAULT
    };
);

// terminal request tag:
#[repr(C)]
pub struct LimineTerminalResponse {
    pub revision: u64,

    pub columns: u32,
    pub rows: u32,

    write: LiminePtr<()>,
}

impl LimineTerminalResponse {
    pub fn write(&self) -> impl Fn(&str) {
        let __fn_ptr = self.write.raw_get();
        let __term_func =
            unsafe { core::mem::transmute::<*const (), extern "C" fn(*const i8, u64)>(__fn_ptr) };

        move |txt| {
            __term_func(txt.as_ptr() as *const i8, txt.len() as u64);
        }
    }
}

make_struct!(
    struct LimineTerminalRequest: [0x0785a0aea5d0750f, 0x1c1936fee0d6cf6e] => {
        response: LiminePtr<LimineTerminalResponse> = LiminePtr::DEFAULT,
        callback: LiminePtr<()> = LiminePtr::DEFAULT
    };
);

// 5-level paging request tag:
#[repr(C)]
pub struct Limine5LevelPagingResponse {
    pub revision: u64,
}

make_struct!(
    struct Limine5LevelPagingRequest: [0x94469551da9b3192, 0xebe5e86db7382888] => {
        response: LiminePtr<Limine5LevelPagingResponse> = LiminePtr::DEFAULT
    };
);

// todo: smp request tag:
// todo: smp memory map tag:

// entry point request tag:
#[repr(C)]
pub struct LimineEntryPointResponse {
    pub revision: u64,
}

// todo: add helper function to get a rusty function pointer to the entry point.
make_struct!(
    struct LimineEntryPointRequest: [0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a] => {
        response: LiminePtr<LimineEntryPointResponse> = LiminePtr::DEFAULT,
        entry: LiminePtr<()> = LiminePtr::DEFAULT
    };
);

// kernel file request tag:
#[repr(C)]
pub struct LimineKernelFileResponse {
    pub revision: u64,
    pub kernel_file: LiminePtr<LimineFile>,
}

make_struct!(
    struct LimineKernelFileRequest: [0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69] => {
        response: LiminePtr<LimineEntryPointResponse> = LiminePtr::DEFAULT
    };
);
