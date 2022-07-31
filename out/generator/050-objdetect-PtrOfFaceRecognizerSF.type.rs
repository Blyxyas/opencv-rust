pub type PtrOfFaceRecognizerSF = core::Ptr<dyn crate::objdetect::FaceRecognizerSF>;

ptr_extern! { dyn crate::objdetect::FaceRecognizerSF,
	cv_PtrOfFaceRecognizerSF_delete, cv_PtrOfFaceRecognizerSF_get_inner_ptr, cv_PtrOfFaceRecognizerSF_get_inner_ptr_mut
}

impl PtrOfFaceRecognizerSF {
	#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::FaceRecognizerSFConst for PtrOfFaceRecognizerSF {
	#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::FaceRecognizerSF for PtrOfFaceRecognizerSF {
	#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

