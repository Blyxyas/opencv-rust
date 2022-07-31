pub type PtrOfOCRHMMDecoder_ClassifierCallback = core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>;

ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
	cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::text::OCRHMMDecoder_ClassifierCallback, cv_PtrOfOCRHMMDecoder_ClassifierCallback_new }

impl PtrOfOCRHMMDecoder_ClassifierCallback {
	#[inline] pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for PtrOfOCRHMMDecoder_ClassifierCallback {
	#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for PtrOfOCRHMMDecoder_ClassifierCallback {
	#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

