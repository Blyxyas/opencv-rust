pub type PtrOfAKAZE = core::Ptr<dyn crate::features2d::AKAZE>;

ptr_extern! { dyn crate::features2d::AKAZE,
	cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr, cv_PtrOfAKAZE_get_inner_ptr_mut
}

impl PtrOfAKAZE {
	#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::AKAZEConst for PtrOfAKAZE {
	#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::AKAZE for PtrOfAKAZE {
	#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAKAZE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAKAZE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfAKAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAKAZE, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfAKAZE_to_PtrOfFeature2D,
}

