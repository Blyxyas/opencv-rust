pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject>;

impl VectorOfDetectionBasedTracker_ExtObject {
	pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject, *const c_void, *mut c_void,
	cv_VectorOfDetectionBasedTracker_ExtObject_new, cv_VectorOfDetectionBasedTracker_ExtObject_delete,
	cv_VectorOfDetectionBasedTracker_ExtObject_len, cv_VectorOfDetectionBasedTracker_ExtObject_is_empty,
	cv_VectorOfDetectionBasedTracker_ExtObject_capacity, cv_VectorOfDetectionBasedTracker_ExtObject_resize,
	cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit,
	cv_VectorOfDetectionBasedTracker_ExtObject_reserve, cv_VectorOfDetectionBasedTracker_ExtObject_remove,
	cv_VectorOfDetectionBasedTracker_ExtObject_swap, cv_VectorOfDetectionBasedTracker_ExtObject_clear,
	cv_VectorOfDetectionBasedTracker_ExtObject_get, cv_VectorOfDetectionBasedTracker_ExtObject_set,
	cv_VectorOfDetectionBasedTracker_ExtObject_push, cv_VectorOfDetectionBasedTracker_ExtObject_insert,
}
vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_ExtObject }

unsafe impl Send for core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject> {}

