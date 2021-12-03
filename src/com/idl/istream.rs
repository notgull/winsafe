#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::idl;
use crate::com::idl::isequentialstream::{
	ISequentialStreamT,
	ISequentialStreamVT,
};
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::ffi::{HRES, PVOID};
use crate::privs::ok_to_hrresult;

/// [`ISequentialStream`](crate::idl::ISequentialStream) virtual table.
#[repr(C)]
pub struct IStreamVT {
	pub ISequentialStreamVT: ISequentialStreamVT,
	pub Seek: fn(ComPtr, i64, u32, *mut u64) -> HRES,
	pub SetSize: fn(ComPtr, u64) -> HRES,
	pub CopyTo: fn(ComPtr, ComPtr, u64, *mut u64, *mut u64) -> HRES,
	pub Commit: fn(ComPtr, u32)-> HRES,
	pub Revert: fn(ComPtr) -> HRES,
	pub LockRegion: fn(ComPtr, u64, u64, u32) -> HRES,
	pub UnlockRegion: fn(ComPtr, u64, u64, u32) -> HRES,
	pub Stat: fn(ComPtr, PVOID, u32) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IStream`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-istream)
/// COM interface over [`IStreamVT`](crate::idl::vt::IStreamVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IStream(ComPtr);

impl_iunknown!(IStream, 0x0000000c, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl ISequentialStreamT for IStream {}
impl IStreamT for IStream {}

/// Exposes the [`IStream`](crate::idl::IStream) methods.
pub trait IStreamT: IUnknownT {
	/// [`IStream::Commit`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-commit)
	/// method.
	fn Commit(&self, flags: idl::co::STGC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.Commit)(self.ptr(), flags.0)
			},
		)
	}

	/// [`IStream::CopyTo`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-copyto)
	/// method.
	///
	/// Returns the number of bytes read and written.
	fn CopyTo(&self, dest: &IStream, num_bytes: u64) -> HrResult<(u64, u64)> {
		let (mut read, mut written) = (u64::default(), u64::default());
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.CopyTo)(
					self.ptr(),
					dest.ptr(),
					num_bytes,
					&mut read,
					&mut written,
				)
			},
		).map(|_| (read, written))
	}

	/// [`IStream::LockRegion`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-lockregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`IStream::UnlockRegion`](crate::prelude::IStreamT::UnlockRegion) call.
	fn LockRegion(&self,
		offset: u64, length: u64, lock_type: idl::co::LOCKTYPE) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.LockRegion)(self.ptr(), offset, length, lock_type.0)
			},
		)
	}

	/// [`IStream::Revert`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-revert)
	/// method.
	fn Revert(&self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.Revert)(self.ptr())
			},
		)
	}

	/// [`IStream::Seek`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-seek)
	/// method.
	///
	/// Returns the new absolute offset.
	fn Seek(&self,
		displacement: i64, origin: idl::co::STREAM_SEEK) -> HrResult<u64>
	{
		let mut new_off = u64::default();
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.Seek)(self.ptr(), displacement, origin.0, &mut new_off)
			},
		).map(|_| new_off)
	}

	/// [`IStream::SetSize`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-setsize)
	/// method.
	fn SetSize(&self, new_size: u64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.SetSize)(self.ptr(), new_size)
			},
		)
	}

	/// [`IStream::UnlockRegion`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-istream-unlockregion)
	/// method.
	fn UnlockRegion(&self,
		offset: u64, length: u64, lock_type: idl::co::LOCKTYPE) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IStreamVT);
				(vt.UnlockRegion)(self.ptr(), offset, length, lock_type.0)
			},
		)
	}
}
