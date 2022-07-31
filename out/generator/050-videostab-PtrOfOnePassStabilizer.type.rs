pub type PtrOfOnePassStabilizer = core::Ptr<crate::videostab::OnePassStabilizer>;

ptr_extern! { crate::videostab::OnePassStabilizer,
	cv_PtrOfOnePassStabilizer_delete, cv_PtrOfOnePassStabilizer_get_inner_ptr, cv_PtrOfOnePassStabilizer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::OnePassStabilizer, cv_PtrOfOnePassStabilizer_new }

impl PtrOfOnePassStabilizer {
	#[inline] pub fn as_raw_PtrOfOnePassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOnePassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::OnePassStabilizerTraitConst for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::OnePassStabilizerTrait for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfOnePassStabilizer, core::Ptr<dyn crate::videostab::IFrameSource>,
	cv_PtrOfOnePassStabilizer_to_PtrOfIFrameSource,
}

impl crate::videostab::StabilizerBaseConst for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::StabilizerBase for PtrOfOnePassStabilizer {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

