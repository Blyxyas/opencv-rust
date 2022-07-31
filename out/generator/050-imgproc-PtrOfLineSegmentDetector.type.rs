pub type PtrOfLineSegmentDetector = core::Ptr<dyn crate::imgproc::LineSegmentDetector>;

ptr_extern! { dyn crate::imgproc::LineSegmentDetector,
	cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr, cv_PtrOfLineSegmentDetector_get_inner_ptr_mut
}

impl PtrOfLineSegmentDetector {
	#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::LineSegmentDetectorConst for PtrOfLineSegmentDetector {
	#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
	#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLineSegmentDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

