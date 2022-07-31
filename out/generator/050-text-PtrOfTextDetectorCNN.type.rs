pub type PtrOfTextDetectorCNN = core::Ptr<dyn crate::text::TextDetectorCNN>;

ptr_extern! { dyn crate::text::TextDetectorCNN,
	cv_PtrOfTextDetectorCNN_delete, cv_PtrOfTextDetectorCNN_get_inner_ptr, cv_PtrOfTextDetectorCNN_get_inner_ptr_mut
}

impl PtrOfTextDetectorCNN {
	#[inline] pub fn as_raw_PtrOfTextDetectorCNN(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTextDetectorCNN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::TextDetectorCNNConst for PtrOfTextDetectorCNN {
	#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::TextDetectorCNN for PtrOfTextDetectorCNN {
	#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::TextDetectorConst for PtrOfTextDetectorCNN {
	#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::TextDetector for PtrOfTextDetectorCNN {
	#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

