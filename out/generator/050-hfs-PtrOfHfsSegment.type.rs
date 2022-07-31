pub type PtrOfHfsSegment = core::Ptr<dyn crate::hfs::HfsSegment>;

ptr_extern! { dyn crate::hfs::HfsSegment,
	cv_PtrOfHfsSegment_delete, cv_PtrOfHfsSegment_get_inner_ptr, cv_PtrOfHfsSegment_get_inner_ptr_mut
}

impl PtrOfHfsSegment {
	#[inline] pub fn as_raw_PtrOfHfsSegment(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHfsSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::hfs::HfsSegmentConst for PtrOfHfsSegment {
	#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::hfs::HfsSegment for PtrOfHfsSegment {
	#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHfsSegment {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHfsSegment {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

