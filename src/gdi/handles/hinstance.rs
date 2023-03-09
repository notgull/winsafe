#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::{IdObmStr, IdOcrStr, IdOicStr};
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{HINSTANCE, SysResult};
use crate::kernel::privs::ptr_to_sysresult;
use crate::prelude::Handle;
use crate::user::decl::{HBITMAP, HCURSOR, HICON, SIZE};
use crate::user::guard::{DestroyCursorGuard, DestroyIconGuard};

impl gdi_Hinstance for HINSTANCE {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hinstance: Handle {
	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	#[must_use]
	fn LoadImageBitmap(&self,
		name: IdObmStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		ptr_to_sysresult(
			unsafe {
				gdi::ffi::LoadImageW(
					self.as_ptr(), name.as_ptr(), 0, sz.cx, sz.cy, load.0)
			},
			|ptr| DeleteObjectGuard::new(HBITMAP(ptr)),
		)
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	#[must_use]
	fn LoadImageCursor(&self,
		name: IdOcrStr, sz: SIZE, load: co::LR) -> SysResult<DestroyCursorGuard>
	{
		ptr_to_sysresult(
			unsafe {
				gdi::ffi::LoadImageW(
					self.as_ptr(), name.as_ptr(), 2, sz.cx, sz.cy, load.0)
			},
			|ptr| DestroyCursorGuard::new(HCURSOR(ptr)),
		)
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	#[must_use]
	fn LoadImageIcon(&self,
		name: IdOicStr, sz: SIZE, load: co::LR) -> SysResult<DestroyIconGuard>
	{
		ptr_to_sysresult(
			unsafe {
				gdi::ffi::LoadImageW(
					self.as_ptr(), name.as_ptr(), 1, sz.cx, sz.cy, load.0)
			},
			|ptr| DestroyIconGuard::new(HICON(ptr)),
		)
	}
}
