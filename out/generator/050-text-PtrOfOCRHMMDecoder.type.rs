pub type PtrOfOCRHMMDecoder = core::Ptr<crate::text::OCRHMMDecoder>;

ptr_extern! { crate::text::OCRHMMDecoder,
	cv_PtrOfOCRHMMDecoder_delete, cv_PtrOfOCRHMMDecoder_get_inner_ptr, cv_PtrOfOCRHMMDecoder_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::text::OCRHMMDecoder, cv_PtrOfOCRHMMDecoder_new }

impl PtrOfOCRHMMDecoder {
	#[inline] pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRHMMDecoderTraitConst for PtrOfOCRHMMDecoder {
	#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRHMMDecoderTrait for PtrOfOCRHMMDecoder {
	#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRConst for PtrOfOCRHMMDecoder {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCR for PtrOfOCRHMMDecoder {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

