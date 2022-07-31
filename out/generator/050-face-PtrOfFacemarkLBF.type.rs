pub type PtrOfFacemarkLBF = core::Ptr<dyn crate::face::FacemarkLBF>;

ptr_extern! { dyn crate::face::FacemarkLBF,
	cv_PtrOfFacemarkLBF_delete, cv_PtrOfFacemarkLBF_get_inner_ptr, cv_PtrOfFacemarkLBF_get_inner_ptr_mut
}

impl PtrOfFacemarkLBF {
	#[inline] pub fn as_raw_PtrOfFacemarkLBF(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemarkLBF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::FacemarkLBFConst for PtrOfFacemarkLBF {
	#[inline] fn as_raw_FacemarkLBF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkLBF for PtrOfFacemarkLBF {
	#[inline] fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFacemarkLBF {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFacemarkLBF {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FacemarkConst for PtrOfFacemarkLBF {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::Facemark for PtrOfFacemarkLBF {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::FacemarkTrainConst for PtrOfFacemarkLBF {
	#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::FacemarkTrain for PtrOfFacemarkLBF {
	#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

