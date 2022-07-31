pub type PtrOfTrainData = core::Ptr<dyn crate::ml::TrainData>;

ptr_extern! { dyn crate::ml::TrainData,
	cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr, cv_PtrOfTrainData_get_inner_ptr_mut
}

impl PtrOfTrainData {
	#[inline] pub fn as_raw_PtrOfTrainData(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::TrainDataConst for PtrOfTrainData {
	#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::TrainData for PtrOfTrainData {
	#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

