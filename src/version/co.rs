#![allow(non_camel_case_types)]

const_ordinary! { VFT: u32: "version";
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileType` (`u32`).
	=>
	=>
	UNKNOWN 0x0000_0000
	APP 0x0000_0001
	DLL 0x0000_0002
	DRV 0x0000_0003
	FONT 0x0000_0004
	VXD 0x0000_0005
	STATIC_LIB 0x0000_0007
}

const_ordinary! { VFT2: u32: "version";
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileSubType` (`u32`).
	=>
	=>
	UNKNOWN 0x0000_0000
	DRV_PRINTER 0x0000_0001
	DRV_KEYBOARD 0x0000_0002
	DRV_LANGUAGE 0x0000_0003
	DRV_DISPLAY 0x0000_0004
	DRV_MOUSE 0x0000_0005
	DRV_NETWORK 0x0000_0006
	DRV_SYSTEM 0x0000_0007
	DRV_INSTALLABLE 0x0000_0008
	DRV_SOUND 0x0000_0009
	DRV_COMM 0x0000_000a
	DRV_INPUTMETHOD 0x0000_000b
	DRV_VERSIONED_PRINTER 0x0000_000c

	FONT_RASTER 0x0000_0001
	FONT_VECTOR 0x0000_0002
	FONT_TRUETYPE 0x0000_0003
}

const_ordinary! { VOS: u32: "version";
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileOS` (`u32`).
	=>
	=>
	UNKNOWN 0x0000_0000
	DOS 0x0001_0000
	OS216 0x0002_0000
	OS232 0x0003_0000
	NT 0x0004_0000
	WINCE 0x0005_0000

	_BASE 0x0000_0000
	_WINDOWS16 0x0000_0001
	_PM16 0x0000_0002
	_PM32 0x0000_0003
	_WINDOWS32 0x0000_0004

	DOS_WINDOWS16 0x0001_0001
	DOS_WINDOWS32 0x0001_0004
	OS216_PM16 0x0002_0002
	OS232_PM32 0x0003_0003
	NT_WINDOWS32 0x0004_0004
}

const_bitflag! { VS_FF: u32: "version";
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileFlags` (`u32`).
	=>
	=>
	DEBUG 0x0000_0001
	PRERELEASE 0x0000_0002
	PATCHED 0x0000_0004
	PRIVATEBUILD 0x0000_0008
	INFOINFERRED 0x0000_0010
	SPECIALBUILD 0x0000_0020
}