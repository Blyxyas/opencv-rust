pub type PtrOfIMotionStabilizer = core::Ptr<dyn crate::videostab::IMotionStabilizer>;

ptr_extern! { dyn crate::videostab::IMotionStabilizer,
	cv_PtrOfIMotionStabilizer_delete, cv_PtrOfIMotionStabilizer_get_inner_ptr, cv_PtrOfIMotionStabilizer_get_inner_ptr_mut
}

impl PtrOfIMotionStabilizer {
	#[inline] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerConst for PtrOfIMotionStabilizer {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

