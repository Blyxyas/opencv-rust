pub type PtrOfTanLayer = core::Ptr<crate::dnn::TanLayer>;

ptr_extern! { crate::dnn::TanLayer,
	cv_PtrOfTanLayer_delete, cv_PtrOfTanLayer_get_inner_ptr, cv_PtrOfTanLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::TanLayer, cv_PtrOfTanLayer_new }

impl PtrOfTanLayer {
	#[inline] pub fn as_raw_PtrOfTanLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TanLayerTraitConst for PtrOfTanLayer {
	#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::TanLayerTrait for PtrOfTanLayer {
	#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTanLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTanLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfTanLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfTanLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfTanLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfTanLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

