pub type PtrOfDetectionOutputLayer = core::Ptr<crate::dnn::DetectionOutputLayer>;

ptr_extern! { crate::dnn::DetectionOutputLayer,
	cv_PtrOfDetectionOutputLayer_delete, cv_PtrOfDetectionOutputLayer_get_inner_ptr, cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrOfDetectionOutputLayer_new }

impl PtrOfDetectionOutputLayer {
	#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DetectionOutputLayerTraitConst for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::DetectionOutputLayerTrait for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfDetectionOutputLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

