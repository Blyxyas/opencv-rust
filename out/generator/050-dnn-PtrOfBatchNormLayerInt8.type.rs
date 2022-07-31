pub type PtrOfBatchNormLayerInt8 = core::Ptr<crate::dnn::BatchNormLayerInt8>;

ptr_extern! { crate::dnn::BatchNormLayerInt8,
	cv_PtrOfBatchNormLayerInt8_delete, cv_PtrOfBatchNormLayerInt8_get_inner_ptr, cv_PtrOfBatchNormLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrOfBatchNormLayerInt8_new }

impl PtrOfBatchNormLayerInt8 {
	#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerInt8TraitConst for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerInt8Trait for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::BatchNormLayerTraitConst for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfBatchNormLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

