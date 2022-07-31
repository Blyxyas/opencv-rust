pub type PtrOfCeilLayer = core::Ptr<crate::dnn::CeilLayer>;

ptr_extern! { crate::dnn::CeilLayer,
	cv_PtrOfCeilLayer_delete, cv_PtrOfCeilLayer_get_inner_ptr, cv_PtrOfCeilLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CeilLayer, cv_PtrOfCeilLayer_new }

impl PtrOfCeilLayer {
	#[inline] pub fn as_raw_PtrOfCeilLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCeilLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CeilLayerTraitConst for PtrOfCeilLayer {
	#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CeilLayerTrait for PtrOfCeilLayer {
	#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCeilLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCeilLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfCeilLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfCeilLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCeilLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCeilLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

