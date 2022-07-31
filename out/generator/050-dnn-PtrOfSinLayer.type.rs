pub type PtrOfSinLayer = core::Ptr<crate::dnn::SinLayer>;

ptr_extern! { crate::dnn::SinLayer,
	cv_PtrOfSinLayer_delete, cv_PtrOfSinLayer_get_inner_ptr, cv_PtrOfSinLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SinLayer, cv_PtrOfSinLayer_new }

impl PtrOfSinLayer {
	#[inline] pub fn as_raw_PtrOfSinLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SinLayerTraitConst for PtrOfSinLayer {
	#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SinLayerTrait for PtrOfSinLayer {
	#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSinLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSinLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSinLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSinLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSinLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSinLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

