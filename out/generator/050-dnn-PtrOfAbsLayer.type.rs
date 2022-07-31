pub type PtrOfAbsLayer = core::Ptr<crate::dnn::AbsLayer>;

ptr_extern! { crate::dnn::AbsLayer,
	cv_PtrOfAbsLayer_delete, cv_PtrOfAbsLayer_get_inner_ptr, cv_PtrOfAbsLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrOfAbsLayer_new }

impl PtrOfAbsLayer {
	#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AbsLayerTraitConst for PtrOfAbsLayer {
	#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AbsLayerTrait for PtrOfAbsLayer {
	#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAbsLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAbsLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAbsLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAbsLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAbsLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAbsLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

