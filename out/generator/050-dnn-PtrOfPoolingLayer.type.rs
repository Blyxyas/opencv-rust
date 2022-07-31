pub type PtrOfPoolingLayer = core::Ptr<crate::dnn::PoolingLayer>;

ptr_extern! { crate::dnn::PoolingLayer,
	cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr, cv_PtrOfPoolingLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrOfPoolingLayer_new }

impl PtrOfPoolingLayer {
	#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerTraitConst for PtrOfPoolingLayer {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerTrait for PtrOfPoolingLayer {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPoolingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPoolingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPoolingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPoolingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

