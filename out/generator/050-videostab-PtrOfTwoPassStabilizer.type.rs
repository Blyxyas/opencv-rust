pub type PtrOfTwoPassStabilizer = core::Ptr<crate::videostab::TwoPassStabilizer>;

ptr_extern! { crate::videostab::TwoPassStabilizer,
	cv_PtrOfTwoPassStabilizer_delete, cv_PtrOfTwoPassStabilizer_get_inner_ptr, cv_PtrOfTwoPassStabilizer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::TwoPassStabilizer, cv_PtrOfTwoPassStabilizer_new }

impl PtrOfTwoPassStabilizer {
	#[inline] pub fn as_raw_PtrOfTwoPassStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTwoPassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::TwoPassStabilizerTraitConst for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::TwoPassStabilizerTrait for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceConst for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSource for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfTwoPassStabilizer, core::Ptr<dyn crate::videostab::IFrameSource>,
	cv_PtrOfTwoPassStabilizer_to_PtrOfIFrameSource,
}

impl crate::videostab::StabilizerBaseConst for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::StabilizerBase for PtrOfTwoPassStabilizer {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

