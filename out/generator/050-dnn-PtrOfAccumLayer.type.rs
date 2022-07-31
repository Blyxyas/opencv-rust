pub type PtrOfAccumLayer = core::Ptr<crate::dnn::AccumLayer>;

ptr_extern! { crate::dnn::AccumLayer,
	cv_PtrOfAccumLayer_delete, cv_PtrOfAccumLayer_get_inner_ptr, cv_PtrOfAccumLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrOfAccumLayer_new }

impl PtrOfAccumLayer {
	#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AccumLayerTraitConst for PtrOfAccumLayer {
	#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AccumLayerTrait for PtrOfAccumLayer {
	#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAccumLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAccumLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAccumLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAccumLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

