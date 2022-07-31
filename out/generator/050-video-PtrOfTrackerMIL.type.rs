pub type PtrOfTrackerMIL = core::Ptr<dyn crate::video::TrackerMIL>;

ptr_extern! { dyn crate::video::TrackerMIL,
	cv_PtrOfTrackerMIL_delete, cv_PtrOfTrackerMIL_get_inner_ptr, cv_PtrOfTrackerMIL_get_inner_ptr_mut
}

impl PtrOfTrackerMIL {
	#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::TrackerMILConst for PtrOfTrackerMIL {
	#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerMIL for PtrOfTrackerMIL {
	#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerConst for PtrOfTrackerMIL {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::Tracker for PtrOfTrackerMIL {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

