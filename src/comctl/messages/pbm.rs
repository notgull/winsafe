use crate::co;
use crate::comctl::decl::PBRANGE;
use crate::comctl::privs::CLR_DEFAULT;
use crate::kernel::decl::{HIWORD, LOWORD, MAKEDWORD, SysResult};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::COLORREF;
use crate::user::privs::zero_as_err;

/// [`PBM_DELTAPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-deltapos)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct DeltaPos {
	pub advance_amount: u32,
}

unsafe impl MsgSend for DeltaPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::DELTAPOS.into(),
			wparam: self.advance_amount as _,
			lparam: 0,
		}
	}
}

/// [`PBM_GETBARCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getbarcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBarColor {}

unsafe impl MsgSend for GetBarColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(COLORREF(v)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETBARCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETBKCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetBkColor {}

unsafe impl MsgSend for GetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(COLORREF(v)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getpos)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetPos {}

unsafe impl MsgSend for GetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getrange)
/// message parameters.
///
/// Return type: `i32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetRange<'a> {
	pub return_low: bool,
	pub ranges: Option<&'a mut PBRANGE>,
}

unsafe impl<'a> MsgSend for GetRange<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETRANGE.into(),
			wparam: self.return_low as _,
			lparam: self.ranges.as_mut().map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`PBM_GETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getstate)
/// message, which has no parameters.
///
/// Return type: `co::PBST`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetState {}

unsafe impl MsgSend for GetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::PBST(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETSTEP`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-getstep)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetStep {}

unsafe impl MsgSend for GetStep {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTEP.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_SETBARCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setbarcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetBarColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBarColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(COLORREF(v)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETBARCOLOR.into(),
			wparam: self.color.map_or(CLR_DEFAULT as _, |color| color.0 as _),
			lparam: 0,
		}
	}
}

/// [`PBM_SETBKCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setbkcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetBkColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(COLORREF(v)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETBKCOLOR.into(),
			wparam: self.color.map_or(CLR_DEFAULT as _, |color| color.0 as _),
			lparam: 0,
		}
	}
}

/// [`PBM_SETMARQUEE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setmarquee)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetMarquee {
	pub turn_on: bool,
	pub time_ms: Option<u32>,
}

unsafe impl MsgSend for SetMarquee {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETMARQUEE.into(),
			wparam: self.turn_on as _,
			lparam: self.time_ms.unwrap_or(0) as _,
		}
	}
}

/// [`PBM_SETPOS`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setpos)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetPos {
	pub position: u32,
}

unsafe impl MsgSend for SetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETPOS.into(),
			wparam: self.position as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setrange)
/// message parameters.
///
/// Return type: `SysResult<(u16, u16)>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetRange {
	pub min: u16,
	pub max: u16,
}

unsafe impl MsgSend for SetRange {
	type RetType = SysResult<(u16, u16)>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|v| (LOWORD(v as _), HIWORD(v as _)))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETRANGE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.min, self.max) as _,
		}
	}
}

/// [`PBM_SETRANGE32`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setrange32)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetRange32 {
	pub min: u32,
	pub max: u32,
}

unsafe impl MsgSend for SetRange32 {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETRANGE32.into(),
			wparam: self.min as _,
			lparam: self.max as _,
		}
	}
}

/// [`PBM_SETSTATE`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setstate)
/// message parameters.
///
/// Return type: `co::PBST`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetState {
	pub state: co::PBST,
}

unsafe impl MsgSend for SetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::PBST(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTATE.into(),
			wparam: self.state.0 as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETSTEP`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setstep)
/// message parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetStep {
	pub step: u32,
}

unsafe impl MsgSend for SetStep {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTEP.into(),
			wparam: self.step as _,
			lparam: 0,
		}
	}
}

/// [`PBM_STEPIT`](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-stepit)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct StepIt {}

unsafe impl MsgSend for StepIt {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::STEPIT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
