pub type PtrOfPoolingLayerInt8 = core::Ptr<crate::dnn::PoolingLayerInt8>;

ptr_extern! { crate::dnn::PoolingLayerInt8,
	cv_PtrOfPoolingLayerInt8_delete, cv_PtrOfPoolingLayerInt8_get_inner_ptr, cv_PtrOfPoolingLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrOfPoolingLayerInt8_new }

impl PtrOfPoolingLayerInt8 {
	#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerInt8TraitConst for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerInt8Trait for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::PoolingLayerTraitConst for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PoolingLayerTrait for PtrOfPoolingLayerInt8 {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

