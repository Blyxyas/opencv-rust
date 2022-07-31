pub type PtrOfSoftmaxLayerInt8 = core::Ptr<crate::dnn::SoftmaxLayerInt8>;

ptr_extern! { crate::dnn::SoftmaxLayerInt8,
	cv_PtrOfSoftmaxLayerInt8_delete, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrOfSoftmaxLayerInt8_new }

impl PtrOfSoftmaxLayerInt8 {
	#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerInt8TraitConst for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerInt8Trait for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerTraitConst for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for PtrOfSoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

