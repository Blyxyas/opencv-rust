pub type VectorOfVectorOfPoint3d = core::Vector<core::Vector<core::Point3d>>;

impl VectorOfVectorOfPoint3d {
	pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point3d>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
	cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
	cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_resize,
	cv_VectorOfVectorOfPoint3d_shrink_to_fit,
	cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
	cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
	cv_VectorOfVectorOfPoint3d_get, cv_VectorOfVectorOfPoint3d_set,
	cv_VectorOfVectorOfPoint3d_push, cv_VectorOfVectorOfPoint3d_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point3d> }

unsafe impl Send for core::Vector<core::Vector<core::Point3d>> {}

impl core::ToInputArray for VectorOfVectorOfPoint3d {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfPoint3d {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfPoint3d {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfPoint3d }

