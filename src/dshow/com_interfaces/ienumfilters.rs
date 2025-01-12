#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::dshow::decl::IBaseFilter;
use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IEnumFilters`](crate::IEnumFilters) virtual table.
#[repr(C)]
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

com_interface! { IEnumFilters: "56a86893-0ad4-11ce-b03a-0020af0ba770";
	/// [`IEnumFilters`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ienumfilters)
	/// COM interface over [`IEnumFiltersVT`](crate::vt::IEnumFiltersVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IEnumFilters for IEnumFilters {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IEnumFilters`](crate::IEnumFilters).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IEnumFilters: ole_IUnknown {
	/// Returns an iterator over the [`IBaseFilter`](crate::IBaseFilter)
	/// elements which calls
	/// [`IEnumFilters::Next`](crate::prelude::dshow_IEnumFilters::Next)
	/// internally.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IEnumFilters;
	///
	/// let filters: IEnumFilters; // initialized somewhere
	/// # let filters = IEnumFilters::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for filter in filters.iter() {
	///     let filter = filter?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> Box<dyn Iterator<Item = HrResult<IBaseFilter>> + '_> {
		Box::new(EnumFiltersIter::new(self))
	}

	/// [`IEnumFilters::Next`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumFilters::iter`](crate::prelude::dshow_IEnumFilters::iter), which
	/// is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IBaseFilter>> {
		let mut fetched = u32::default();
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IEnumFiltersVT>();
			match ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched), // retrieve only 1
			) {
				Ok(_) => Ok(Some(IBaseFilter::from(ppv_queried))),
				Err(hr) => match hr {
					co::HRESULT::S_FALSE => Ok(None), // no filter found
					hr => Err(hr), // actual error
				},
			}
		}
	}

	/// [`IEnumFilters::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IEnumFiltersVT>();
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumFilters::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ienumfilters-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumFiltersVT>();
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}

//------------------------------------------------------------------------------

struct EnumFiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	enum_filters: &'a I,
}

impl<'a, I> Iterator for EnumFiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	type Item = HrResult<IBaseFilter>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_filters.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> EnumFiltersIter<'a, I>
	where I: dshow_IEnumFilters,
{
	fn new(enum_filters: &'a I) -> Self {
		Self { enum_filters }
	}
}
