pub type PtrOfSoftplusLayer = core::Ptr<crate::dnn::SoftplusLayer>;

ptr_extern! { crate::dnn::SoftplusLayer,
	cv_PtrOfSoftplusLayer_delete, cv_PtrOfSoftplusLayer_get_inner_ptr, cv_PtrOfSoftplusLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SoftplusLayer, cv_PtrOfSoftplusLayer_new }

impl PtrOfSoftplusLayer {
	#[inline] pub fn as_raw_PtrOfSoftplusLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftplusLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftplusLayerTraitConst for PtrOfSoftplusLayer {
	#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftplusLayerTrait for PtrOfSoftplusLayer {
	#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSoftplusLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSoftplusLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSoftplusLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSoftplusLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSoftplusLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSoftplusLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

