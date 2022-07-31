pub type PtrOfScanSegment = core::Ptr<dyn crate::ximgproc::ScanSegment>;

ptr_extern! { dyn crate::ximgproc::ScanSegment,
	cv_PtrOfScanSegment_delete, cv_PtrOfScanSegment_get_inner_ptr, cv_PtrOfScanSegment_get_inner_ptr_mut
}

impl PtrOfScanSegment {
	#[inline] pub fn as_raw_PtrOfScanSegment(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScanSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::ScanSegmentConst for PtrOfScanSegment {
	#[inline] fn as_raw_ScanSegment(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::ScanSegment for PtrOfScanSegment {
	#[inline] fn as_raw_mut_ScanSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfScanSegment {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfScanSegment {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

