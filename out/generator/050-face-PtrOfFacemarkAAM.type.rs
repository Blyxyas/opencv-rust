pub type PtrOfFacemarkAAM = core::Ptr<dyn crate::face::FacemarkAAM>;

ptr_extern! { dyn crate::face::FacemarkAAM,
	cv_PtrOfFacemarkAAM_delete, cv_PtrOfFacemarkAAM_get_inner_ptr, cv_PtrOfFacemarkAAM_get_inner_ptr_mut
}

impl PtrOfFacemarkAAM {
	#[inline] pub fn as_raw_PtrOfFacemarkAAM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkAAM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::FacemarkAAMConst for PtrOfFacemarkAAM {
	#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkAAM for PtrOfFacemarkAAM {
	#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFacemarkAAM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFacemarkAAM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FacemarkConst for PtrOfFacemarkAAM {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::Facemark for PtrOfFacemarkAAM {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FacemarkTrainConst for PtrOfFacemarkAAM {
	#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrain for PtrOfFacemarkAAM {
	#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

