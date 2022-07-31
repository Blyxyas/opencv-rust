pub type PtrOfTrackerCSRT = core::Ptr<dyn crate::tracking::TrackerCSRT>;

ptr_extern! { dyn crate::tracking::TrackerCSRT,
	cv_PtrOfTrackerCSRT_delete, cv_PtrOfTrackerCSRT_get_inner_ptr, cv_PtrOfTrackerCSRT_get_inner_ptr_mut
}

impl PtrOfTrackerCSRT {
	#[inline] pub fn as_raw_PtrOfTrackerCSRT(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerCSRTConst for PtrOfTrackerCSRT {
	#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerCSRT for PtrOfTrackerCSRT {
	#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerConst for PtrOfTrackerCSRT {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::Tracker for PtrOfTrackerCSRT {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

