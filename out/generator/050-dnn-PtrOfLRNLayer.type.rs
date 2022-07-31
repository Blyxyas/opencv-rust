pub type PtrOfLRNLayer = core::Ptr<crate::dnn::LRNLayer>;

ptr_extern! { crate::dnn::LRNLayer,
	cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr, cv_PtrOfLRNLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrOfLRNLayer_new }

impl PtrOfLRNLayer {
	#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LRNLayerTraitConst for PtrOfLRNLayer {
	#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LRNLayerTrait for PtrOfLRNLayer {
	#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLRNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLRNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfLRNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfLRNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

