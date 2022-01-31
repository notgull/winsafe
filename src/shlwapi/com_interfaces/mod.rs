mod ibindctx;
mod ipersist;
mod isequentialstream;
mod istream;

pub mod decl {
	pub use super::ibindctx::IBindCtx;
	pub use super::ipersist::IPersist;
	pub use super::isequentialstream::ISequentialStream;
	pub use super::istream::IStream;
}

pub mod traits {
	pub use super::ibindctx::ShlwapiIBindCtx;
	pub use super::ipersist::ShlwapiIPersist;
	pub use super::isequentialstream::ShlwapiISequentialStream;
	pub use super::istream::ShlwapiIStream;
}

pub mod vt {
	pub use super::ibindctx::IBindCtxVT;
	pub use super::ipersist::IPersistVT;
	pub use super::isequentialstream::ISequentialStreamVT;
	pub use super::istream::IStreamVT;
}
