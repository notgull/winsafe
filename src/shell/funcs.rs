#![allow(non_snake_case)]

use crate::{co, shell};
use crate::kernel::decl::{
	GetLastError, HACCESSTOKEN, HLOCAL, SysResult, WString,
};
use crate::kernel::guard::LocalFreeGuard;
use crate::kernel::privs::{
	bool_to_sysresult, MAX_PATH, ptr_to_sysresult,
};
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult, IStream};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, ole_IBindCtx, shell_IShellItem};
use crate::shell::decl::{
	NOTIFYICONDATA, SHFILEINFO, SHFILEOPSTRUCT, SHSTOCKICONINFO,
};
use crate::shell::guard::{DestroyIconShfiGuard, DestroyIconSiiGuard};

/// [`CommandLineToArgv`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{CommandLineToArgv, GetCommandLine};
///
/// let args = CommandLineToArgv(&GetCommandLine())?;
/// for arg in args.iter() {
///     println!("{}", arg);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn CommandLineToArgv(cmd_line: &str) -> SysResult<Vec<String>> {
	let mut num_args = i32::default();
	let lp_arr = unsafe {
		shell::ffi::CommandLineToArgvW(
			WString::from_str(cmd_line).as_ptr(),
			&mut num_args,
		)
	};
	if lp_arr.is_null() {
		return Err(GetLastError());
	}

	let mut strs = Vec::with_capacity(num_args as _);
	for lp in unsafe { std::slice::from_raw_parts(lp_arr, num_args as _) }.iter() {
		strs.push(WString::from_wchars_nullt(*lp).to_string());
	}

	let _ = unsafe { LocalFreeGuard::new(HLOCAL::from_ptr(lp_arr as _)) };
	Ok(strs)
}

/// [`PathCombine`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathcombinew)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::PathCombine;
///
/// let full = PathCombine(Some("C:"), Some("One\\Two\\Three"))?;
///
/// // full = "C:\\One\\Two\\Three"
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub fn PathCombine(
	str_dir: Option<&str>, str_file: Option<&str>) -> SysResult<String>
{
	let mut buf = WString::new_alloc_buf(MAX_PATH);
	ptr_to_sysresult(
		unsafe {
			shell::ffi::PathCombineW(
				buf.as_mut_ptr(),
				WString::from_opt_str(str_dir).as_ptr(),
				WString::from_opt_str(str_file).as_ptr(),
			) as _
		},
	).map(|_| buf.to_string())
}

/// [`PathCommonPrefix`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathcommonprefixw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::PathCommonPrefix;
///
/// if let Some(common_prefix) = PathCommonPrefix(
///     "C:\\temp\\one\\foo.txt",
///     "C:\\temp\\two\\bar.txt",
/// ) {
///     println!("Common prefix: {}", common_prefix); // "C:\\temp"
/// }
/// ```
pub fn PathCommonPrefix(file1: &str, file2: &str) -> Option<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH);
	match unsafe {
		shell::ffi::PathCommonPrefixW(
			WString::from_str(file1).as_ptr(),
			WString::from_str(file2).as_ptr(),
			buf.as_mut_ptr(),
		)
	} {
		0 => None,
		_ => Some(buf.to_string()),
	}
}

/// [`PathSkipRoot`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathskiprootw)
/// function.
pub fn PathSkipRoot(str_path: &str) -> Option<String> {
	let buf = WString::from_str(str_path);
	unsafe {
		shell::ffi::PathSkipRootW(buf.as_ptr()).as_ref()
	}.map(|ptr| WString::from_wchars_nullt(ptr).to_string())
}

/// [`PathStripPath`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathstrippathw)
/// function.
pub fn PathStripPath(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe { shell::ffi::PathStripPathW(buf.as_mut_ptr()); }
	buf.to_string()
}

/// [`PathUndecorate`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathundecoratew)
/// function.
pub fn PathUndecorate(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe { shell::ffi::PathUndecorateW(buf.as_mut_ptr()); }
	buf.to_string()
}

/// [`PathUnquoteSpaces`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-pathunquotespacesw)
/// function.
pub fn PathUnquoteSpaces(str_path: &str) -> String {
	let mut buf = WString::from_str(str_path);
	unsafe { shell::ffi::PathUnquoteSpacesW(buf.as_mut_ptr()); }
	buf.to_string()
}

/// [`SHAddToRecentDocs`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shaddtorecentdocs)
/// function.
///
/// # Safety
///
/// The `pv` type varies according to `uFlags`. If you set it wrong, you're
/// likely to cause a buffer overrun.
pub unsafe fn SHAddToRecentDocs<T>(flags: co::SHARD, pv: &T) {
	shell::ffi::SHAddToRecentDocs(flags.0, pv as *const _ as _);
}

/// [`Shell_NotifyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw)
/// function.
pub fn Shell_NotifyIcon(
	message: co::NIM, data: &mut NOTIFYICONDATA) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe { shell::ffi::Shell_NotifyIconW(message.0, data as *mut _ as _) },
	)
}

/// [`SHCreateItemFromParsingName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{IBindCtx, IShellItem2, SHCreateItemFromParsingName};
///
/// let shi = SHCreateItemFromParsingName::<IShellItem2>(
///     "C:\\Temp\\foo.txt",
///     None::<&IBindCtx>,
/// )?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn SHCreateItemFromParsingName<T>(
	file_or_folder_path: &str,
	bind_ctx: Option<&impl ole_IBindCtx>,
) -> HrResult<T>
	where T: shell_IShellItem,
{
	unsafe {
		let mut ppv_queried = ComPtr::null();
		ok_to_hrresult(
			shell::ffi::SHCreateItemFromParsingName(
				WString::from_str(file_or_folder_path).as_ptr(),
				bind_ctx.map_or(std::ptr::null_mut(), |i| i.ptr().0 as _),
				&T::IID as *const _ as _,
				&mut ppv_queried as *mut _ as _,
			),
		).map(|_| T::from(ppv_queried))
	}
}

/// [`SHCreateMemStream`](https://learn.microsoft.com/en-us/windows/win32/api/shlwapi/nf-shlwapi-shcreatememstream)
/// function.
///
/// # Examples
///
/// Loading from a `Vec`:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::SHCreateMemStream;
///
/// let raw_data: Vec<u8>; // initialized somewhere
/// # let raw_data = Vec::<u8>::default();
///
/// let stream = SHCreateMemStream(&raw_data)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn SHCreateMemStream(src: &[u8]) -> HrResult<IStream> {
	let p = unsafe {
		shell::ffi::SHCreateMemStream(src.as_ptr(), src.len() as _)
	};
	if p.is_null() {
		Err(co::HRESULT::E_OUTOFMEMORY)
	} else {
		Ok(IStream::from(ComPtr(p as _)))
	}
}

/// [`SHFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shfileoperationw)
/// function.
pub fn SHFileOperation(file_op: &mut SHFILEOPSTRUCT) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { shell::ffi::SHFileOperationW(file_op as *mut _ as _) },
	)
}

/// [`SHGetFileInfo`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow)
/// function.
pub fn SHGetFileInfo(
	path: &str,
	file_attrs: co::FILE_ATTRIBUTE,
	flags: co::SHGFI,
) -> SysResult<(u32, DestroyIconShfiGuard)>
{
	let mut shfi = SHFILEINFO::default();
	unsafe {
		match shell::ffi::SHGetFileInfoW(
			WString::from_str(path).as_ptr(),
			file_attrs.0,
			&mut shfi as *mut _ as _,
			std::mem::size_of::<SHFILEINFO>() as _,
			flags.0,
		) {
			0 => Err(GetLastError()),
			n => Ok((n as _, DestroyIconShfiGuard::new(shfi))),
		}
	}
}

/// [`SHGetKnownFolderPath`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath)
/// function.
///
/// # Examples
///
/// Retrieving documents folder:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, SHGetKnownFolderPath};
///
/// let docs_folder = SHGetKnownFolderPath(
///     &co::KNOWNFOLDERID::Documents,
///     co::KF::DEFAULT,
///     None,
/// )?;
///
/// println!("Docs folder: {}", docs_folder);
/// # Ok::<_, co::HRESULT>(())
/// ```
#[must_use]
pub fn SHGetKnownFolderPath(
	folder_id: &co::KNOWNFOLDERID,
	flags: co::KF,
	token: Option<&HACCESSTOKEN>,
) -> HrResult<String>
{
	let mut pstr: *mut u16 = std::ptr::null_mut();
	ok_to_hrresult(
		unsafe {
			shell::ffi::SHGetKnownFolderPath(
				folder_id as *const _ as _,
				flags.0,
				token.map_or(std::ptr::null_mut(), |t| t.as_ptr()),
				&mut pstr,
			)
		},
	).map(|_| {
		let path = WString::from_wchars_nullt(pstr);
		CoTaskMemFree(pstr as _);
		path.to_string()
	})
}

/// [`SHGetStockIconInfo`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetstockiconinfo)
/// function.
///
/// # Examples
///
/// Loading the small (16x16 pixels) camera icon from the system:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, SHGetStockIconInfo};
///
/// let sii = SHGetStockIconInfo(
///     co::SIID::DEVICECAMERA,
///     co::SHGSI::ICON | co::SHGSI::SMALLICON,
/// )?;
///
/// println!("HICON handle: {}", sii.hIcon);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn SHGetStockIconInfo(
	siid: co::SIID, flags: co::SHGSI) -> HrResult<DestroyIconSiiGuard>
{
	let mut sii = SHSTOCKICONINFO::default();
	unsafe {
		ok_to_hrresult(
			shell::ffi::SHGetStockIconInfo(
				siid.0,
				flags.0,
				&mut sii as *mut _ as _,
			),
		).map(|_| DestroyIconSiiGuard::new(sii))
	}
}
