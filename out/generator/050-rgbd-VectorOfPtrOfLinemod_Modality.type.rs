pub type VectorOfPtrOfLinemod_Modality = core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>>;

impl VectorOfPtrOfLinemod_Modality {
	pub fn as_raw_VectorOfPtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Ptr<dyn crate::rgbd::Linemod_Modality>, *const c_void, *mut c_void,
	cv_VectorOfPtrOfLinemod_Modality_new, cv_VectorOfPtrOfLinemod_Modality_delete,
	cv_VectorOfPtrOfLinemod_Modality_len, cv_VectorOfPtrOfLinemod_Modality_is_empty,
	cv_VectorOfPtrOfLinemod_Modality_capacity, cv_VectorOfPtrOfLinemod_Modality_resize,
	cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit,
	cv_VectorOfPtrOfLinemod_Modality_reserve, cv_VectorOfPtrOfLinemod_Modality_remove,
	cv_VectorOfPtrOfLinemod_Modality_swap, cv_VectorOfPtrOfLinemod_Modality_clear,
	cv_VectorOfPtrOfLinemod_Modality_get, cv_VectorOfPtrOfLinemod_Modality_set,
	cv_VectorOfPtrOfLinemod_Modality_push, cv_VectorOfPtrOfLinemod_Modality_insert,
}
vector_non_copy_or_bool! { core::Ptr<dyn crate::rgbd::Linemod_Modality> }

unsafe impl Send for core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>> {}

