pub type PtrOfFisherFaceRecognizer = core::Ptr<dyn crate::face::FisherFaceRecognizer>;

ptr_extern! { dyn crate::face::FisherFaceRecognizer,
	cv_PtrOfFisherFaceRecognizer_delete, cv_PtrOfFisherFaceRecognizer_get_inner_ptr, cv_PtrOfFisherFaceRecognizer_get_inner_ptr_mut
}

impl PtrOfFisherFaceRecognizer {
	#[inline] pub fn as_raw_PtrOfFisherFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFisherFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::FisherFaceRecognizerConst for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FisherFaceRecognizer for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::BasicFaceRecognizerConst for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BasicFaceRecognizer for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FaceRecognizerConst for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizer for PtrOfFisherFaceRecognizer {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

