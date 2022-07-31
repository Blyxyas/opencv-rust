pub type VectorOfVectorOfPoint3i = core::Vector<core::Vector<core::Point3i>>;

impl VectorOfVectorOfPoint3i {
	pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point3i>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
	cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
	cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_resize,
	cv_VectorOfVectorOfPoint3i_shrink_to_fit,
	cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
	cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
	cv_VectorOfVectorOfPoint3i_get, cv_VectorOfVectorOfPoint3i_set,
	cv_VectorOfVectorOfPoint3i_push, cv_VectorOfVectorOfPoint3i_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point3i> }

unsafe impl Send for core::Vector<core::Vector<core::Point3i>> {}

impl core::ToInputArray for VectorOfVectorOfPoint3i {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfPoint3i {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfPoint3i {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfPoint3i }

