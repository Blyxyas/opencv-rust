pub type PtrOfInnerProductLayer = core::Ptr<crate::dnn::InnerProductLayer>;

ptr_extern! { crate::dnn::InnerProductLayer,
	cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr, cv_PtrOfInnerProductLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrOfInnerProductLayer_new }

impl PtrOfInnerProductLayer {
	#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerTraitConst for PtrOfInnerProductLayer {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for PtrOfInnerProductLayer {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfInnerProductLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfInnerProductLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfInnerProductLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfInnerProductLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

