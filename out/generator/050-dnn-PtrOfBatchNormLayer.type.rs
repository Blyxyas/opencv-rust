pub type PtrOfBatchNormLayer = core::Ptr<crate::dnn::BatchNormLayer>;

ptr_extern! { crate::dnn::BatchNormLayer,
	cv_PtrOfBatchNormLayer_delete, cv_PtrOfBatchNormLayer_get_inner_ptr, cv_PtrOfBatchNormLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrOfBatchNormLayer_new }

impl PtrOfBatchNormLayer {
	#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerTraitConst for PtrOfBatchNormLayer {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for PtrOfBatchNormLayer {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBatchNormLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBatchNormLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfBatchNormLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfBatchNormLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfBatchNormLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfBatchNormLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

