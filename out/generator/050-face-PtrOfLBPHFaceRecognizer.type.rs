pub type PtrOfLBPHFaceRecognizer = core::Ptr<dyn crate::face::LBPHFaceRecognizer>;

ptr_extern! { dyn crate::face::LBPHFaceRecognizer,
	cv_PtrOfLBPHFaceRecognizer_delete, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr_mut
}

impl PtrOfLBPHFaceRecognizer {
	#[inline] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::LBPHFaceRecognizerConst for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FaceRecognizerConst for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

