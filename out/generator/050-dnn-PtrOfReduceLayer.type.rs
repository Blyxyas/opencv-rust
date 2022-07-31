pub type PtrOfReduceLayer = core::Ptr<crate::dnn::ReduceLayer>;

ptr_extern! { crate::dnn::ReduceLayer,
	cv_PtrOfReduceLayer_delete, cv_PtrOfReduceLayer_get_inner_ptr, cv_PtrOfReduceLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReduceLayer, cv_PtrOfReduceLayer_new }

impl PtrOfReduceLayer {
	#[inline] pub fn as_raw_PtrOfReduceLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReduceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReduceLayerTraitConst for PtrOfReduceLayer {
	#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReduceLayerTrait for PtrOfReduceLayer {
	#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReduceLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReduceLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReduceLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReduceLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

