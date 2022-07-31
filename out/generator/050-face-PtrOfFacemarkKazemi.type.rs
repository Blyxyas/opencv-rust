pub type PtrOfFacemarkKazemi = core::Ptr<dyn crate::face::FacemarkKazemi>;

ptr_extern! { dyn crate::face::FacemarkKazemi,
	cv_PtrOfFacemarkKazemi_delete, cv_PtrOfFacemarkKazemi_get_inner_ptr, cv_PtrOfFacemarkKazemi_get_inner_ptr_mut
}

impl PtrOfFacemarkKazemi {
	#[inline] pub fn as_raw_PtrOfFacemarkKazemi(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkKazemi(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::FacemarkKazemiConst for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkKazemi for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FacemarkConst for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::Facemark for PtrOfFacemarkKazemi {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

