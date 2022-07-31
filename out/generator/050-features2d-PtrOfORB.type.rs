pub type PtrOfORB = core::Ptr<dyn crate::features2d::ORB>;

ptr_extern! { dyn crate::features2d::ORB,
	cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr, cv_PtrOfORB_get_inner_ptr_mut
}

impl PtrOfORB {
	#[inline] pub fn as_raw_PtrOfORB(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfORB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::ORBConst for PtrOfORB {
	#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::ORB for PtrOfORB {
	#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfORB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfORB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfORB {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfORB {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfORB, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfORB_to_PtrOfFeature2D,
}

