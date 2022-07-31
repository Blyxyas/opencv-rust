pub type PtrOfPermuteLayer = core::Ptr<crate::dnn::PermuteLayer>;

ptr_extern! { crate::dnn::PermuteLayer,
	cv_PtrOfPermuteLayer_delete, cv_PtrOfPermuteLayer_get_inner_ptr, cv_PtrOfPermuteLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrOfPermuteLayer_new }

impl PtrOfPermuteLayer {
	#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PermuteLayerTraitConst for PtrOfPermuteLayer {
	#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PermuteLayerTrait for PtrOfPermuteLayer {
	#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPermuteLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPermuteLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPermuteLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPermuteLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

