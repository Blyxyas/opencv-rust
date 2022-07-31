pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector>;

ptr_extern! { dyn crate::objdetect::DetectionBasedTracker_IDetector,
	cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut
}

impl PtrOfDetectionBasedTracker_IDetector {
	#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::DetectionBasedTracker_IDetectorConst for PtrOfDetectionBasedTracker_IDetector {
	#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::DetectionBasedTracker_IDetector for PtrOfDetectionBasedTracker_IDetector {
	#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

