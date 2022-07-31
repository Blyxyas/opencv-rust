pub type PtrOfSoftmaxLayer = core::Ptr<crate::dnn::SoftmaxLayer>;

ptr_extern! { crate::dnn::SoftmaxLayer,
	cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr, cv_PtrOfSoftmaxLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrOfSoftmaxLayer_new }

impl PtrOfSoftmaxLayer {
	#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerTraitConst for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSoftmaxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

