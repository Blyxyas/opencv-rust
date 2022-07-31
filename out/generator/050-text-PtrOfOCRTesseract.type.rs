pub type PtrOfOCRTesseract = core::Ptr<dyn crate::text::OCRTesseract>;

ptr_extern! { dyn crate::text::OCRTesseract,
	cv_PtrOfOCRTesseract_delete, cv_PtrOfOCRTesseract_get_inner_ptr, cv_PtrOfOCRTesseract_get_inner_ptr_mut
}

impl PtrOfOCRTesseract {
	#[inline] pub fn as_raw_PtrOfOCRTesseract(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRTesseractConst for PtrOfOCRTesseract {
	#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRTesseract for PtrOfOCRTesseract {
	#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRConst for PtrOfOCRTesseract {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCR for PtrOfOCRTesseract {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

