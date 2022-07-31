pub type PtrOfMSER = core::Ptr<dyn crate::features2d::MSER>;

ptr_extern! { dyn crate::features2d::MSER,
	cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr, cv_PtrOfMSER_get_inner_ptr_mut
}

impl PtrOfMSER {
	#[inline] pub fn as_raw_PtrOfMSER(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::MSERConst for PtrOfMSER {
	#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::MSER for PtrOfMSER {
	#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMSER {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMSER {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfMSER {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfMSER {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMSER, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfMSER_to_PtrOfFeature2D,
}

