pub type PtrOfFeature2D = core::Ptr<crate::features2d::Feature2D>;

ptr_extern! { crate::features2d::Feature2D,
	cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr, cv_PtrOfFeature2D_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrOfFeature2D_new }

impl PtrOfFeature2D {
	#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfFeature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfFeature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFeature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFeature2D {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

