#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::SysResult;
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::Handle;
use crate::user;
use crate::user::guard::DestroyIconGuard;

impl_handle! { HICON;
	/// Handle to an
	/// [icon](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
}

impl user_Hicon for HICON {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HICON`](crate::HICON).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hicon: Handle {
	/// [`CopyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// method.
	#[must_use]
	fn CopyIcon(&self) -> SysResult<DestroyIconGuard> {
		unsafe {
			ptr_to_sysresult_handle(user::ffi::CopyIcon(self.as_ptr()))
				.map(|h| DestroyIconGuard::new(h))
		}
	}
}
