pub type PtrOfDetail_RotationWarper = core::Ptr<dyn crate::stitching::Detail_RotationWarper>;

ptr_extern! { dyn crate::stitching::Detail_RotationWarper,
	cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr, cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut
}

impl PtrOfDetail_RotationWarper {
	#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarperConst for PtrOfDetail_RotationWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_RotationWarper for PtrOfDetail_RotationWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

