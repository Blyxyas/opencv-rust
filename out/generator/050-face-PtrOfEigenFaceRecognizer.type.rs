pub type PtrOfEigenFaceRecognizer = core::Ptr<dyn crate::face::EigenFaceRecognizer>;

ptr_extern! { dyn crate::face::EigenFaceRecognizer,
	cv_PtrOfEigenFaceRecognizer_delete, cv_PtrOfEigenFaceRecognizer_get_inner_ptr, cv_PtrOfEigenFaceRecognizer_get_inner_ptr_mut
}

impl PtrOfEigenFaceRecognizer {
	#[inline] pub fn as_raw_PtrOfEigenFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEigenFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::EigenFaceRecognizerConst for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::EigenFaceRecognizer for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::BasicFaceRecognizerConst for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BasicFaceRecognizer for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FaceRecognizerConst for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FaceRecognizer for PtrOfEigenFaceRecognizer {
	#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

