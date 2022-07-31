pub type PtrOfSoftsignLayer = core::Ptr<crate::dnn::SoftsignLayer>;

ptr_extern! { crate::dnn::SoftsignLayer,
	cv_PtrOfSoftsignLayer_delete, cv_PtrOfSoftsignLayer_get_inner_ptr, cv_PtrOfSoftsignLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SoftsignLayer, cv_PtrOfSoftsignLayer_new }

impl PtrOfSoftsignLayer {
	#[inline] pub fn as_raw_PtrOfSoftsignLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSoftsignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftsignLayerTraitConst for PtrOfSoftsignLayer {
	#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SoftsignLayerTrait for PtrOfSoftsignLayer {
	#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSoftsignLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSoftsignLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSoftsignLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSoftsignLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSoftsignLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSoftsignLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

