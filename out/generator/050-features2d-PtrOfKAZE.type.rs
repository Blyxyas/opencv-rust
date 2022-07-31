pub type PtrOfKAZE = core::Ptr<dyn crate::features2d::KAZE>;

ptr_extern! { dyn crate::features2d::KAZE,
	cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr, cv_PtrOfKAZE_get_inner_ptr_mut
}

impl PtrOfKAZE {
	#[inline] pub fn as_raw_PtrOfKAZE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::KAZEConst for PtrOfKAZE {
	#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::KAZE for PtrOfKAZE {
	#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfKAZE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfKAZE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfKAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfKAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfKAZE, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfKAZE_to_PtrOfFeature2D,
}

