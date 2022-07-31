pub type PtrOfCharucoBoard = core::Ptr<crate::aruco::CharucoBoard>;

ptr_extern! { crate::aruco::CharucoBoard,
	cv_PtrOfCharucoBoard_delete, cv_PtrOfCharucoBoard_get_inner_ptr, cv_PtrOfCharucoBoard_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::CharucoBoard, cv_PtrOfCharucoBoard_new }

impl PtrOfCharucoBoard {
	#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::CharucoBoardTraitConst for PtrOfCharucoBoard {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::CharucoBoardTrait for PtrOfCharucoBoard {
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for PtrOfCharucoBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for PtrOfCharucoBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

