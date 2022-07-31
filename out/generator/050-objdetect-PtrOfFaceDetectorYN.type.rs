pub type PtrOfFaceDetectorYN = core::Ptr<dyn crate::objdetect::FaceDetectorYN>;

ptr_extern! { dyn crate::objdetect::FaceDetectorYN,
	cv_PtrOfFaceDetectorYN_delete, cv_PtrOfFaceDetectorYN_get_inner_ptr, cv_PtrOfFaceDetectorYN_get_inner_ptr_mut
}

impl PtrOfFaceDetectorYN {
	#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::FaceDetectorYNConst for PtrOfFaceDetectorYN {
	#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::FaceDetectorYN for PtrOfFaceDetectorYN {
	#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

