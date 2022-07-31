pub type PtrOfOCRHolisticWordRecognizer = core::Ptr<dyn crate::text::OCRHolisticWordRecognizer>;

ptr_extern! { dyn crate::text::OCRHolisticWordRecognizer,
	cv_PtrOfOCRHolisticWordRecognizer_delete, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr_mut
}

impl PtrOfOCRHolisticWordRecognizer {
	#[inline] pub fn as_raw_PtrOfOCRHolisticWordRecognizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRHolisticWordRecognizerConst for PtrOfOCRHolisticWordRecognizer {
	#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHolisticWordRecognizer for PtrOfOCRHolisticWordRecognizer {
	#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRConst for PtrOfOCRHolisticWordRecognizer {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCR for PtrOfOCRHolisticWordRecognizer {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

