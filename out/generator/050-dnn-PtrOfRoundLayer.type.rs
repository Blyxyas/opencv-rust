pub type PtrOfRoundLayer = core::Ptr<crate::dnn::RoundLayer>;

ptr_extern! { crate::dnn::RoundLayer,
	cv_PtrOfRoundLayer_delete, cv_PtrOfRoundLayer_get_inner_ptr, cv_PtrOfRoundLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrOfRoundLayer_new }

impl PtrOfRoundLayer {
	#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RoundLayerTraitConst for PtrOfRoundLayer {
	#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RoundLayerTrait for PtrOfRoundLayer {
	#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRoundLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRoundLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfRoundLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfRoundLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfRoundLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfRoundLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

