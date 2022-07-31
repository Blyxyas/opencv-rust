pub type PtrOfLayer = core::Ptr<crate::dnn::Layer>;

ptr_extern! { crate::dnn::Layer,
	cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr, cv_PtrOfLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::Layer, cv_PtrOfLayer_new }

impl PtrOfLayer {
	#[inline] pub fn as_raw_PtrOfLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

