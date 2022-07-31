#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Face Analysis
//! 
//! - @ref face_changelog
//! - @ref tutorial_face_main
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PredictCollectorConst, super::PredictCollector, super::StandardCollectorTraitConst, super::StandardCollectorTrait, super::FaceRecognizerConst, super::FaceRecognizer, super::BasicFaceRecognizerConst, super::BasicFaceRecognizer, super::EigenFaceRecognizerConst, super::EigenFaceRecognizer, super::FisherFaceRecognizerConst, super::FisherFaceRecognizer, super::LBPHFaceRecognizerConst, super::LBPHFaceRecognizer, super::FacemarkConst, super::Facemark, super::CParamsTraitConst, super::CParamsTrait, super::FacemarkTrainConst, super::FacemarkTrain, super::FacemarkLBF_ParamsTraitConst, super::FacemarkLBF_ParamsTrait, super::FacemarkLBFConst, super::FacemarkLBF, super::FacemarkAAM_ParamsTraitConst, super::FacemarkAAM_ParamsTrait, super::FacemarkAAM_ConfigTraitConst, super::FacemarkAAM_ConfigTrait, super::FacemarkAAM_DataTraitConst, super::FacemarkAAM_DataTrait, super::FacemarkAAM_Model_TextureTraitConst, super::FacemarkAAM_Model_TextureTrait, super::FacemarkAAM_ModelTraitConst, super::FacemarkAAM_ModelTrait, super::FacemarkAAMConst, super::FacemarkAAM, super::FacemarkKazemi_ParamsTraitConst, super::FacemarkKazemi_ParamsTrait, super::FacemarkKazemiConst, super::FacemarkKazemi, super::MACEConst, super::MACE, super::BIFConst, super::BIF };
}

pub type FN_FaceDetector = Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>;
#[inline]
pub fn create_facemark_aam() -> Result<core::Ptr<dyn crate::face::Facemark>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_createFacemarkAAM(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_facemark_kazemi() -> Result<core::Ptr<dyn crate::face::Facemark>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_createFacemarkKazemi(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_facemark_lbf() -> Result<core::Ptr<dyn crate::face::Facemark>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_createFacemarkLBF(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * color: Scalar(255,0,0)
#[inline]
pub fn draw_facemarks(image: &mut dyn core::ToInputOutputArray, points: &dyn core::ToInputArray, color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), points.as_raw__InputArray(), color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_faces_haar(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, face_cascade_name: &str) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(faces);
	extern_container_arg!(face_cascade_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(image.as_raw__InputArray(), faces.as_raw__OutputArray(), face_cascade_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_faces(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, params: &mut crate::face::CParams) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(faces);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(image.as_raw__InputArray(), faces.as_raw__OutputArray(), params.as_raw_mut_CParams(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn load_dataset_list(image_list: &str, annotation_list: &str, images: &mut core::Vector<String>, annotations: &mut core::Vector<String>) -> Result<bool> {
	extern_container_arg!(mut image_list);
	extern_container_arg!(mut annotation_list);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_loadDatasetList_String_String_vector_String_R_vector_String_R(image_list.opencv_as_extern_mut(), annotation_list.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), annotations.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * offset: 0.0f
#[inline]
pub fn load_face_points(filename: &str, points: &mut dyn core::ToOutputArray, offset: f32) -> Result<bool> {
	extern_container_arg!(mut filename);
	output_array_arg!(points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_loadFacePoints_String_const__OutputArrayR_float(filename.opencv_as_extern_mut(), points.as_raw__OutputArray(), offset, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * offset: 0.0f
#[inline]
pub fn load_training_data_1(image_list: &str, ground_truth: &str, images: &mut core::Vector<String>, face_points: &mut dyn core::ToOutputArray, offset: f32) -> Result<bool> {
	extern_container_arg!(mut image_list);
	extern_container_arg!(mut ground_truth);
	output_array_arg!(face_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_loadTrainingData_String_String_vector_String_R_const__OutputArrayR_float(image_list.opencv_as_extern_mut(), ground_truth.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), offset, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * delim: ' '
/// * offset: 0.0f
#[inline]
pub fn load_training_data(filename: &str, images: &mut core::Vector<String>, face_points: &mut dyn core::ToOutputArray, delim: i8, offset: f32) -> Result<bool> {
	extern_container_arg!(mut filename);
	output_array_arg!(face_points);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_loadTrainingData_String_vector_String_R_const__OutputArrayR_char_float(filename.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), delim, offset, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn load_training_data_2(mut filename: core::Vector<String>, trainlandmarks: &mut core::Vector<core::Vector<core::Point2f>>, trainimages: &mut core::Vector<String>) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_face_loadTrainingData_vector_String__vector_vector_Point2f__R_vector_String_R(filename.as_raw_mut_VectorOfString(), trainlandmarks.as_raw_mut_VectorOfVectorOfPoint2f(), trainimages.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait BIFConst: core::AlgorithmTraitConst {
	fn as_raw_BIF(&self) -> *const c_void;

	#[inline]
	fn get_num_bands(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BIF_getNumBands_const(self.as_raw_BIF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_num_rotations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BIF_getNumRotations_const(self.as_raw_BIF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn compute(&self, image: &dyn core::ToInputArray, features: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(self.as_raw_BIF(), image.as_raw__InputArray(), features.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BIF: core::AlgorithmTrait + crate::face::BIFConst {
	fn as_raw_mut_BIF(&mut self) -> *mut c_void;

}

impl dyn BIF + '_ {
	/// ## C++ default parameters
	/// * num_bands: 8
	/// * num_rotations: 12
	#[inline]
	pub fn create(num_bands: i32, num_rotations: i32) -> Result<core::Ptr<dyn crate::face::BIF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BIF_create_int_int(num_bands, num_rotations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::BIF>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait BasicFaceRecognizerConst: crate::face::FaceRecognizerConst {
	fn as_raw_BasicFaceRecognizer(&self) -> *const c_void;

	#[inline]
	fn get_num_components(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getNumComponents_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getThreshold_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_projections(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getProjections_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_labels(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getLabels_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_eigen_values(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getEigenValues_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_eigen_vectors(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getEigenVectors_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_mean(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_getMean_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_write_const_FileStorageR(self.as_raw_BasicFaceRecognizer(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_empty_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BasicFaceRecognizer: crate::face::BasicFaceRecognizerConst + crate::face::FaceRecognizer {
	fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void;

	#[inline]
	fn set_num_components(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_setNumComponents_int(self.as_raw_mut_BasicFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_setThreshold_double(self.as_raw_mut_BasicFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_BasicFaceRecognizer_read_const_FileNodeR(self.as_raw_mut_BasicFaceRecognizer(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait CParamsTraitConst {
	fn as_raw_CParams(&self) -> *const c_void;

	#[inline]
	fn cascade(&self) -> String {
		let ret = unsafe { sys::cv_face_CParams_getPropCascade_const(self.as_raw_CParams()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn scale_factor(&self) -> f64 {
		let ret = unsafe { sys::cv_face_CParams_getPropScaleFactor_const(self.as_raw_CParams()) };
		ret
	}
	
	#[inline]
	fn min_neighbors(&self) -> i32 {
		let ret = unsafe { sys::cv_face_CParams_getPropMinNeighbors_const(self.as_raw_CParams()) };
		ret
	}
	
	#[inline]
	fn min_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_CParams_getPropMinSize_const(self.as_raw_CParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn max_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_CParams_getPropMaxSize_const(self.as_raw_CParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn face_cascade(&self) -> crate::objdetect::CascadeClassifier {
		let ret = unsafe { sys::cv_face_CParams_getPropFace_cascade_const(self.as_raw_CParams()) };
		let ret = unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait CParamsTrait: crate::face::CParamsTraitConst {
	fn as_raw_mut_CParams(&mut self) -> *mut c_void;

	#[inline]
	fn set_cascade(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_face_CParams_setPropCascade_String(self.as_raw_mut_CParams(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_scale_factor(&mut self, val: f64) {
		let ret = unsafe { sys::cv_face_CParams_setPropScaleFactor_double(self.as_raw_mut_CParams(), val) };
		ret
	}
	
	#[inline]
	fn set_min_neighbors(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_CParams_setPropMinNeighbors_int(self.as_raw_mut_CParams(), val) };
		ret
	}
	
	#[inline]
	fn set_min_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_face_CParams_setPropMinSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_max_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_face_CParams_setPropMaxSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_face_cascade(&mut self, mut val: crate::objdetect::CascadeClassifier) {
		let ret = unsafe { sys::cv_face_CParams_setPropFace_cascade_CascadeClassifier(self.as_raw_mut_CParams(), val.as_raw_mut_CascadeClassifier()) };
		ret
	}
	
}

pub struct CParams {
	ptr: *mut c_void
}

opencv_type_boxed! { CParams }

impl Drop for CParams {
	fn drop(&mut self) {
		extern "C" { fn cv_CParams_delete(instance: *mut c_void); }
		unsafe { cv_CParams_delete(self.as_raw_mut_CParams()) };
	}
}

unsafe impl Send for CParams {}

impl crate::face::CParamsTraitConst for CParams {
	#[inline] fn as_raw_CParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::CParamsTrait for CParams {
	#[inline] fn as_raw_mut_CParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CParams {
	/// ## C++ default parameters
	/// * sf: 1.1
	/// * min_n: 3
	/// * min_sz: Size(30,30)
	/// * max_sz: Size()
	#[inline]
	pub fn new(cascade_model: &str, sf: f64, min_n: i32, min_sz: core::Size, max_sz: core::Size) -> Result<crate::face::CParams> {
		extern_container_arg!(mut cascade_model);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model.opencv_as_extern_mut(), sf, min_n, min_sz.opencv_as_extern(), max_sz.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::CParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait EigenFaceRecognizerConst: crate::face::BasicFaceRecognizerConst {
	fn as_raw_EigenFaceRecognizer(&self) -> *const c_void;

}

pub trait EigenFaceRecognizer: crate::face::BasicFaceRecognizer + crate::face::EigenFaceRecognizerConst {
	fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void;

}

impl dyn EigenFaceRecognizer + '_ {
	/// ## C++ default parameters
	/// * num_components: 0
	/// * threshold: DBL_MAX
	#[inline]
	pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr<dyn crate::face::EigenFaceRecognizer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_EigenFaceRecognizer_create_int_double(num_components, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::EigenFaceRecognizer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait FaceRecognizerConst: core::AlgorithmTraitConst {
	fn as_raw_FaceRecognizer(&self) -> *const c_void;

	#[inline]
	fn predict_label(&self, src: &dyn core::ToInputArray) -> Result<i32> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn predict(&self, src: &dyn core::ToInputArray, label: &mut i32, confidence: &mut f64) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), label, confidence, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn predict_collect(&self, src: &dyn core::ToInputArray, mut collector: core::Ptr<dyn crate::face::PredictCollector>) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_Ptr_PredictCollector_(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), collector.as_raw_mut_PtrOfPredictCollector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_write_const_const_StringR(self.as_raw_FaceRecognizer(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn write_1(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_write_const_FileStorageR(self.as_raw_FaceRecognizer(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_empty_const(self.as_raw_FaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_label_info(&self, label: i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_getLabelInfo_const_int(self.as_raw_FaceRecognizer(), label, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_labels_by_string(&self, str: &str) -> Result<core::Vector<i32>> {
		extern_container_arg!(str);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(self.as_raw_FaceRecognizer(), str.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_getThreshold_const(self.as_raw_FaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FaceRecognizer: core::AlgorithmTrait + crate::face::FaceRecognizerConst {
	fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void;

	#[inline]
	fn train(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_read_const_StringR(self.as_raw_mut_FaceRecognizer(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read_1(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_read_const_FileNodeR(self.as_raw_mut_FaceRecognizer(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_label_info(&mut self, label: i32, str_info: &str) -> Result<()> {
		extern_container_arg!(str_info);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(self.as_raw_mut_FaceRecognizer(), label, str_info.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FaceRecognizer_setThreshold_double(self.as_raw_mut_FaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FacemarkConst: core::AlgorithmTraitConst {
	fn as_raw_Facemark(&self) -> *const c_void;

}

pub trait Facemark: core::AlgorithmTrait + crate::face::FacemarkConst {
	fn as_raw_mut_Facemark(&mut self) -> *mut c_void;

	#[inline]
	fn load_model(&mut self, model: &str) -> Result<()> {
		extern_container_arg!(mut model);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_Facemark_loadModel_String(self.as_raw_mut_Facemark(), model.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn fit(&mut self, image: &dyn core::ToInputArray, faces: &dyn core::ToInputArray, landmarks: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(faces);
		output_array_arg!(landmarks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Facemark(), image.as_raw__InputArray(), faces.as_raw__InputArray(), landmarks.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FacemarkAAMConst: crate::face::FacemarkTrainConst {
	fn as_raw_FacemarkAAM(&self) -> *const c_void;

}

pub trait FacemarkAAM: crate::face::FacemarkAAMConst + crate::face::FacemarkTrain {
	fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void;

	#[inline]
	fn fit_config(&mut self, image: &dyn core::ToInputArray, roi: &dyn core::ToInputArray, _landmarks: &mut dyn core::ToOutputArray, runtime_params: &core::Vector<crate::face::FacemarkAAM_Config>) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(roi);
		output_array_arg!(_landmarks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vector_Config_R(self.as_raw_mut_FacemarkAAM(), image.as_raw__InputArray(), roi.as_raw__InputArray(), _landmarks.as_raw__OutputArray(), runtime_params.as_raw_VectorOfFacemarkAAM_Config(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn FacemarkAAM + '_ {
	/// ## C++ default parameters
	/// * parameters: FacemarkAAM::Params()
	#[inline]
	pub fn create(parameters: &crate::face::FacemarkAAM_Params) -> Result<core::Ptr<dyn crate::face::FacemarkAAM>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_create_const_ParamsR(parameters.as_raw_FacemarkAAM_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::FacemarkAAM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait FacemarkAAM_ConfigTraitConst {
	fn as_raw_FacemarkAAM_Config(&self) -> *const c_void;

	#[inline]
	fn r(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_getPropR_const(self.as_raw_FacemarkAAM_Config()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn t(&self) -> core::Point2f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Config_getPropT_const(self.as_raw_FacemarkAAM_Config(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_getPropScale_const(self.as_raw_FacemarkAAM_Config()) };
		ret
	}
	
	#[inline]
	fn model_scale_idx(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_getPropModel_scale_idx_const(self.as_raw_FacemarkAAM_Config()) };
		ret
	}
	
}

pub trait FacemarkAAM_ConfigTrait: crate::face::FacemarkAAM_ConfigTraitConst {
	fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void;

	#[inline]
	fn set_r(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_setPropR_Mat(self.as_raw_mut_FacemarkAAM_Config(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_t(&mut self, val: core::Point2f) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_setPropT_Point2f(self.as_raw_mut_FacemarkAAM_Config(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_setPropScale_float(self.as_raw_mut_FacemarkAAM_Config(), val) };
		ret
	}
	
	#[inline]
	fn set_model_scale_idx(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Config_setPropModel_scale_idx_int(self.as_raw_mut_FacemarkAAM_Config(), val) };
		ret
	}
	
}

pub struct FacemarkAAM_Config {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Config }

impl Drop for FacemarkAAM_Config {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Config_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Config_delete(self.as_raw_mut_FacemarkAAM_Config()) };
	}
}

unsafe impl Send for FacemarkAAM_Config {}

impl crate::face::FacemarkAAM_ConfigTraitConst for FacemarkAAM_Config {
	#[inline] fn as_raw_FacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkAAM_ConfigTrait for FacemarkAAM_Config {
	#[inline] fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Config {
	/// ## C++ default parameters
	/// * rot: Mat::eye(2,2,CV_32F)
	/// * trans: Point2f(0.0f,0.0f)
	/// * scaling: 1.0f
	/// * scale_id: 0
	#[inline]
	pub fn new(mut rot: core::Mat, trans: core::Point2f, scaling: f32, scale_id: i32) -> Result<crate::face::FacemarkAAM_Config> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot.as_raw_mut_Mat(), trans.opencv_as_extern(), scaling, scale_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::FacemarkAAM_Config::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FacemarkAAM_DataTraitConst {
	fn as_raw_FacemarkAAM_Data(&self) -> *const c_void;

	#[inline]
	fn s0(&self) -> core::Vector<core::Point2f> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Data_getPropS0_const(self.as_raw_FacemarkAAM_Data()) };
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait FacemarkAAM_DataTrait: crate::face::FacemarkAAM_DataTraitConst {
	fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void;

	#[inline]
	fn set_s0(&mut self, mut val: core::Vector<core::Point2f>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Data_setPropS0_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Data(), val.as_raw_mut_VectorOfPoint2f()) };
		ret
	}
	
}

pub struct FacemarkAAM_Data {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Data }

impl Drop for FacemarkAAM_Data {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Data_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Data_delete(self.as_raw_mut_FacemarkAAM_Data()) };
	}
}

unsafe impl Send for FacemarkAAM_Data {}

impl crate::face::FacemarkAAM_DataTraitConst for FacemarkAAM_Data {
	#[inline] fn as_raw_FacemarkAAM_Data(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkAAM_DataTrait for FacemarkAAM_Data {
	#[inline] fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Data {
}

pub trait FacemarkAAM_ModelTraitConst {
	fn as_raw_FacemarkAAM_Model(&self) -> *const c_void;

	#[inline]
	fn scales(&self) -> core::Vector<f32> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropScales_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn triangles(&self) -> core::Vector<core::Vec3i> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropTriangles_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Vector::<core::Vec3i>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn textures(&self) -> core::Vector<crate::face::FacemarkAAM_Model_Texture> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropTextures_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Vector::<crate::face::FacemarkAAM_Model_Texture>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn s0(&self) -> core::Vector<core::Point2f> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropS0_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn s(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropS_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn q(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_getPropQ_const(self.as_raw_FacemarkAAM_Model()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait FacemarkAAM_ModelTrait: crate::face::FacemarkAAM_ModelTraitConst {
	fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void;

	#[inline]
	fn set_scales(&mut self, mut val: core::Vector<f32>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropScales_vector_float_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOff32()) };
		ret
	}
	
	#[inline]
	fn set_triangles(&mut self, mut val: core::Vector<core::Vec3i>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropTriangles_vector_Vec3i_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfVec3i()) };
		ret
	}
	
	#[inline]
	fn set_textures(&mut self, mut val: core::Vector<crate::face::FacemarkAAM_Model_Texture>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropTextures_vector_Texture_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfFacemarkAAM_Model_Texture()) };
		ret
	}
	
	#[inline]
	fn set_s0(&mut self, mut val: core::Vector<core::Point2f>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropS0_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfPoint2f()) };
		ret
	}
	
	#[inline]
	fn set_s(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropS_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_q(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_setPropQ_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) };
		ret
	}
	
}

pub struct FacemarkAAM_Model {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Model }

impl Drop for FacemarkAAM_Model {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Model_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Model_delete(self.as_raw_mut_FacemarkAAM_Model()) };
	}
}

unsafe impl Send for FacemarkAAM_Model {}

impl crate::face::FacemarkAAM_ModelTraitConst for FacemarkAAM_Model {
	#[inline] fn as_raw_FacemarkAAM_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkAAM_ModelTrait for FacemarkAAM_Model {
	#[inline] fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Model {
}

pub trait FacemarkAAM_Model_TextureTraitConst {
	fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void;

	#[inline]
	fn max_m(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropMax_m_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		ret
	}
	
	#[inline]
	fn resolution(&self) -> core::Rect {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropResolution_const(self.as_raw_FacemarkAAM_Model_Texture(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn a(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropA_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn a0(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropA0_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn aa(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropAA_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn aa0(&self) -> core::Mat {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropAA0_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn texture_idx(&self) -> core::Vector<core::Vector<core::Point>> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropTextureIdx_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Point>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn base_shape(&self) -> core::Vector<core::Point2f> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropBase_shape_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn ind1(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropInd1_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn ind2(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropInd2_const(self.as_raw_FacemarkAAM_Model_Texture()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait FacemarkAAM_Model_TextureTrait: crate::face::FacemarkAAM_Model_TextureTraitConst {
	fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void;

	#[inline]
	fn set_max_m(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropMax_m_int(self.as_raw_mut_FacemarkAAM_Model_Texture(), val) };
		ret
	}
	
	#[inline]
	fn set_resolution(&mut self, val: core::Rect) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropResolution_Rect(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_a(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_a0(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_aa(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropAA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_aa0(&mut self, mut val: core::Mat) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropAA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
		ret
	}
	
	#[inline]
	fn set_texture_idx(&mut self, mut val: core::Vector<core::Vector<core::Point>>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropTextureIdx_vector_vector_Point__(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfVectorOfPoint()) };
		ret
	}
	
	#[inline]
	fn set_base_shape(&mut self, mut val: core::Vector<core::Point2f>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropBase_shape_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfPoint2f()) };
		ret
	}
	
	#[inline]
	fn set_ind1(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropInd1_vector_int_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_ind2(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropInd2_vector_int_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
}

pub struct FacemarkAAM_Model_Texture {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Model_Texture }

impl Drop for FacemarkAAM_Model_Texture {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Model_Texture_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Model_Texture_delete(self.as_raw_mut_FacemarkAAM_Model_Texture()) };
	}
}

unsafe impl Send for FacemarkAAM_Model_Texture {}

impl crate::face::FacemarkAAM_Model_TextureTraitConst for FacemarkAAM_Model_Texture {
	#[inline] fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkAAM_Model_TextureTrait for FacemarkAAM_Model_Texture {
	#[inline] fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Model_Texture {
}

pub trait FacemarkAAM_ParamsTraitConst {
	fn as_raw_FacemarkAAM_Params(&self) -> *const c_void;

	#[inline]
	fn model_filename(&self) -> String {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropModel_filename_const(self.as_raw_FacemarkAAM_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn m(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropM_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn n(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropN_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn n_iter(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropN_iter_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn verbose(&self) -> bool {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropVerbose_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn save_model(&self) -> bool {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropSave_model_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn max_m(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropMax_m_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn max_n(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropMax_n_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn texture_max_m(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropTexture_max_m_const(self.as_raw_FacemarkAAM_Params()) };
		ret
	}
	
	#[inline]
	fn scales(&self) -> core::Vector<f32> {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_getPropScales_const(self.as_raw_FacemarkAAM_Params()) };
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Params_write_const_FileStorageR(self.as_raw_FacemarkAAM_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FacemarkAAM_ParamsTrait: crate::face::FacemarkAAM_ParamsTraitConst {
	fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_model_filename(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropModel_filename_string(self.as_raw_mut_FacemarkAAM_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_m(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropM_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_n(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropN_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_n_iter(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropN_iter_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_verbose(&mut self, val: bool) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropVerbose_bool(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_save_model(&mut self, val: bool) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropSave_model_bool(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_max_m(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropMax_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_max_n(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropMax_n_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_texture_max_m(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropTexture_max_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_scales(&mut self, mut val: core::Vector<f32>) {
		let ret = unsafe { sys::cv_face_FacemarkAAM_Params_setPropScales_vector_float_(self.as_raw_mut_FacemarkAAM_Params(), val.as_raw_mut_VectorOff32()) };
		ret
	}
	
	#[inline]
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkAAM_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct FacemarkAAM_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Params }

impl Drop for FacemarkAAM_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Params_delete(self.as_raw_mut_FacemarkAAM_Params()) };
	}
}

unsafe impl Send for FacemarkAAM_Params {}

impl crate::face::FacemarkAAM_ParamsTraitConst for FacemarkAAM_Params {
	#[inline] fn as_raw_FacemarkAAM_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkAAM_ParamsTrait for FacemarkAAM_Params {
	#[inline] fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Params {
	#[inline]
	pub fn default() -> Result<crate::face::FacemarkAAM_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkAAM_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::FacemarkAAM_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FacemarkKazemiConst: crate::face::FacemarkConst {
	fn as_raw_FacemarkKazemi(&self) -> *const c_void;

}

pub trait FacemarkKazemi: crate::face::Facemark + crate::face::FacemarkKazemiConst {
	fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * model_filename: "face_landmarks.dat"
	#[inline]
	fn training(&mut self, images: &mut core::Vector<core::Mat>, landmarks: &mut core::Vector<core::Vector<core::Point2f>>, configfile: &str, scale: core::Size, model_filename: &str) -> Result<bool> {
		extern_container_arg!(mut configfile);
		extern_container_arg!(mut model_filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkKazemi_training_vector_Mat_R_vector_vector_Point2f__R_string_Size_string(self.as_raw_mut_FacemarkKazemi(), images.as_raw_mut_VectorOfMat(), landmarks.as_raw_mut_VectorOfVectorOfPoint2f(), configfile.opencv_as_extern_mut(), scale.opencv_as_extern(), model_filename.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_face_detector(&mut self, f: Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>) -> Result<bool> {
		callback_arg!(f_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *mut c_void) -> bool => unnamed_2 in callbacks => f(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
		userdata_arg!(user_data in callbacks => f);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(self.as_raw_mut_FacemarkKazemi(), f_trampoline, user_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkKazemi(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn FacemarkKazemi + '_ {
	/// ## C++ default parameters
	/// * parameters: FacemarkKazemi::Params()
	#[inline]
	pub fn create(parameters: &crate::face::FacemarkKazemi_Params) -> Result<core::Ptr<dyn crate::face::FacemarkKazemi>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkKazemi_create_const_ParamsR(parameters.as_raw_FacemarkKazemi_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::FacemarkKazemi>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait FacemarkKazemi_ParamsTraitConst {
	fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void;

	#[inline]
	fn cascade_depth(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropCascade_depth_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn tree_depth(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropTree_depth_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn num_trees_per_cascade_level(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_trees_per_cascade_level_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn learning_rate(&self) -> f32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropLearning_rate_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn oversampling_amount(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropOversampling_amount_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn num_test_coordinates(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_test_coordinates_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn lambda(&self) -> f32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropLambda_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn num_test_splits(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_test_splits_const(self.as_raw_FacemarkKazemi_Params()) };
		ret
	}
	
	#[inline]
	fn configfile(&self) -> String {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_getPropConfigfile_const(self.as_raw_FacemarkKazemi_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait FacemarkKazemi_ParamsTrait: crate::face::FacemarkKazemi_ParamsTraitConst {
	fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_cascade_depth(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropCascade_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tree_depth(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropTree_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_num_trees_per_cascade_level(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_trees_per_cascade_level_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_learning_rate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropLearning_rate_float(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_oversampling_amount(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropOversampling_amount_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_num_test_coordinates(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_test_coordinates_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_lambda(&mut self, val: f32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropLambda_float(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_num_test_splits(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_test_splits_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_configfile(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_setPropConfigfile_String(self.as_raw_mut_FacemarkKazemi_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
}

pub struct FacemarkKazemi_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkKazemi_Params }

impl Drop for FacemarkKazemi_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkKazemi_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkKazemi_Params_delete(self.as_raw_mut_FacemarkKazemi_Params()) };
	}
}

unsafe impl Send for FacemarkKazemi_Params {}

impl crate::face::FacemarkKazemi_ParamsTraitConst for FacemarkKazemi_Params {
	#[inline] fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkKazemi_ParamsTrait for FacemarkKazemi_Params {
	#[inline] fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkKazemi_Params {
	#[inline]
	pub fn default() -> Result<crate::face::FacemarkKazemi_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkKazemi_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::FacemarkKazemi_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FacemarkLBFConst: crate::face::FacemarkTrainConst {
	fn as_raw_FacemarkLBF(&self) -> *const c_void;

}

pub trait FacemarkLBF: crate::face::FacemarkLBFConst + crate::face::FacemarkTrain {
	fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void;

}

impl dyn FacemarkLBF + '_ {
	/// ## C++ default parameters
	/// * parameters: FacemarkLBF::Params()
	#[inline]
	pub fn create(parameters: &crate::face::FacemarkLBF_Params) -> Result<core::Ptr<dyn crate::face::FacemarkLBF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkLBF_create_const_ParamsR(parameters.as_raw_FacemarkLBF_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::FacemarkLBF>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait FacemarkLBF_ParamsTraitConst {
	fn as_raw_FacemarkLBF_Params(&self) -> *const c_void;

	#[inline]
	fn shape_offset(&self) -> f64 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropShape_offset_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn cascade_face(&self) -> String {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropCascade_face_const(self.as_raw_FacemarkLBF_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn verbose(&self) -> bool {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropVerbose_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn n_landmarks(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropN_landmarks_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn init_shape_n(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropInitShape_n_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn stages_n(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropStages_n_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn tree_n(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropTree_n_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn tree_depth(&self) -> i32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropTree_depth_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn bagging_overlap(&self) -> f64 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropBagging_overlap_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn model_filename(&self) -> String {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropModel_filename_const(self.as_raw_FacemarkLBF_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn save_model(&self) -> bool {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropSave_model_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn seed(&self) -> u32 {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropSeed_const(self.as_raw_FacemarkLBF_Params()) };
		ret
	}
	
	#[inline]
	fn feats_m(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropFeats_m_const(self.as_raw_FacemarkLBF_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn radius_m(&self) -> core::Vector<f64> {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_getPropRadius_m_const(self.as_raw_FacemarkLBF_Params()) };
		let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn detect_roi(&self) -> core::Rect {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropDetectROI_const(self.as_raw_FacemarkLBF_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkLBF_Params_write_const_FileStorageR(self.as_raw_FacemarkLBF_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FacemarkLBF_ParamsTrait: crate::face::FacemarkLBF_ParamsTraitConst {
	fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_shape_offset(&mut self, val: f64) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropShape_offset_double(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_cascade_face(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropCascade_face_String(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_verbose(&mut self, val: bool) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropVerbose_bool(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_n_landmarks(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropN_landmarks_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_init_shape_n(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropInitShape_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_stages_n(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropStages_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tree_n(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropTree_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_tree_depth(&mut self, val: i32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropTree_depth_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_bagging_overlap(&mut self, val: f64) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropBagging_overlap_double(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_model_filename(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropModel_filename_string(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_save_model(&mut self, val: bool) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropSave_model_bool(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_seed(&mut self, val: u32) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropSeed_unsigned_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_feats_m(&mut self, mut val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropFeats_m_vector_int_(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_radius_m(&mut self, mut val: core::Vector<f64>) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropRadius_m_vector_double_(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOff64()) };
		ret
	}
	
	#[inline]
	fn set_detect_roi(&mut self, val: core::Rect) {
		let ret = unsafe { sys::cv_face_FacemarkLBF_Params_setPropDetectROI_Rect(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkLBF_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkLBF_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct FacemarkLBF_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkLBF_Params }

impl Drop for FacemarkLBF_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkLBF_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkLBF_Params_delete(self.as_raw_mut_FacemarkLBF_Params()) };
	}
}

unsafe impl Send for FacemarkLBF_Params {}

impl crate::face::FacemarkLBF_ParamsTraitConst for FacemarkLBF_Params {
	#[inline] fn as_raw_FacemarkLBF_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::FacemarkLBF_ParamsTrait for FacemarkLBF_Params {
	#[inline] fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkLBF_Params {
	#[inline]
	pub fn default() -> Result<crate::face::FacemarkLBF_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkLBF_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::FacemarkLBF_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait FacemarkTrainConst: crate::face::FacemarkConst {
	fn as_raw_FacemarkTrain(&self) -> *const c_void;

}

pub trait FacemarkTrain: crate::face::Facemark + crate::face::FacemarkTrainConst {
	fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void;

	#[inline]
	fn add_training_sample(&mut self, image: &dyn core::ToInputArray, landmarks: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(landmarks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), landmarks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * parameters: 0
	#[inline]
	unsafe fn training(&mut self, parameters: *mut c_void) -> Result<()> {
		return_send!(via ocvrs_return);
		{ sys::cv_face_FacemarkTrain_training_voidX(self.as_raw_mut_FacemarkTrain(), parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * user_data: 0
	#[inline]
	fn set_face_detector(&mut self, detector: crate::face::FN_FaceDetector) -> Result<bool> {
		callback_arg!(detector_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, user_data: *mut c_void) -> bool => user_data in callbacks => detector(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
		userdata_arg!(user_data in callbacks => detector);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(self.as_raw_mut_FacemarkTrain(), detector_trampoline, user_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * items: 0
	#[inline]
	unsafe fn get_data(&mut self, items: *mut c_void) -> Result<bool> {
		return_send!(via ocvrs_return);
		{ sys::cv_face_FacemarkTrain_getData_voidX(self.as_raw_mut_FacemarkTrain(), items, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait FisherFaceRecognizerConst: crate::face::BasicFaceRecognizerConst {
	fn as_raw_FisherFaceRecognizer(&self) -> *const c_void;

}

pub trait FisherFaceRecognizer: crate::face::BasicFaceRecognizer + crate::face::FisherFaceRecognizerConst {
	fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void;

}

impl dyn FisherFaceRecognizer + '_ {
	/// ## C++ default parameters
	/// * num_components: 0
	/// * threshold: DBL_MAX
	#[inline]
	pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr<dyn crate::face::FisherFaceRecognizer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_FisherFaceRecognizer_create_int_double(num_components, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::FisherFaceRecognizer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait LBPHFaceRecognizerConst: crate::face::FaceRecognizerConst {
	fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void;

	#[inline]
	fn get_grid_x(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getGridX_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_grid_y(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getGridY_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getRadius_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_neighbors(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getNeighbors_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getThreshold_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_histograms(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getHistograms_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_labels(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_getLabels_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait LBPHFaceRecognizer: crate::face::FaceRecognizer + crate::face::LBPHFaceRecognizerConst {
	fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void;

	#[inline]
	fn set_grid_x(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_setGridX_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_grid_y(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_setGridY_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_setRadius_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_neighbors(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_setNeighbors_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_setThreshold_double(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn LBPHFaceRecognizer + '_ {
	/// ## C++ default parameters
	/// * radius: 1
	/// * neighbors: 8
	/// * grid_x: 8
	/// * grid_y: 8
	/// * threshold: DBL_MAX
	#[inline]
	pub fn create(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64) -> Result<core::Ptr<dyn crate::face::LBPHFaceRecognizer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius, neighbors, grid_x, grid_y, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::LBPHFaceRecognizer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait MACEConst: core::AlgorithmTraitConst {
	fn as_raw_MACE(&self) -> *const c_void;

	#[inline]
	fn same(&self, query: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(query);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_MACE_same_const_const__InputArrayR(self.as_raw_MACE(), query.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MACE: core::AlgorithmTrait + crate::face::MACEConst {
	fn as_raw_mut_MACE(&mut self) -> *mut c_void;

	#[inline]
	fn salt(&mut self, passphrase: &str) -> Result<()> {
		extern_container_arg!(passphrase);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_MACE_salt_const_StringR(self.as_raw_mut_MACE(), passphrase.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn train(&mut self, images: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_MACE_train_const__InputArrayR(self.as_raw_mut_MACE(), images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn MACE + '_ {
	/// ## C++ default parameters
	/// * objname: String()
	#[inline]
	pub fn load(filename: &str, objname: &str) -> Result<core::Ptr<dyn crate::face::MACE>> {
		extern_container_arg!(filename);
		extern_container_arg!(objname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_MACE_load_const_StringR_const_StringR(filename.opencv_as_extern(), objname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::MACE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * imgsize: 64
	#[inline]
	pub fn create(imgsize: i32) -> Result<core::Ptr<dyn crate::face::MACE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_MACE_create_int(imgsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::face::MACE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait PredictCollectorConst {
	fn as_raw_PredictCollector(&self) -> *const c_void;

}

pub trait PredictCollector: crate::face::PredictCollectorConst {
	fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void;

	#[inline]
	fn init(&mut self, size: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_PredictCollector_init_size_t(self.as_raw_mut_PredictCollector(), size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_PredictCollector_collect_int_double(self.as_raw_mut_PredictCollector(), label, dist, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StandardCollectorTraitConst: crate::face::PredictCollectorConst {
	fn as_raw_StandardCollector(&self) -> *const c_void;

	#[inline]
	fn get_min_label(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_getMinLabel_const(self.as_raw_StandardCollector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_dist(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_getMinDist_const(self.as_raw_StandardCollector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StandardCollectorTrait: crate::face::PredictCollector + crate::face::StandardCollectorTraitConst {
	fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void;

	#[inline]
	fn init(&mut self, size: size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_init_size_t(self.as_raw_mut_StandardCollector(), size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_collect_int_double(self.as_raw_mut_StandardCollector(), label, dist, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct StandardCollector {
	ptr: *mut c_void
}

opencv_type_boxed! { StandardCollector }

impl Drop for StandardCollector {
	fn drop(&mut self) {
		extern "C" { fn cv_StandardCollector_delete(instance: *mut c_void); }
		unsafe { cv_StandardCollector_delete(self.as_raw_mut_StandardCollector()) };
	}
}

unsafe impl Send for StandardCollector {}

impl crate::face::PredictCollectorConst for StandardCollector {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::PredictCollector for StandardCollector {
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::StandardCollectorTraitConst for StandardCollector {
	#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.as_raw() }
}

impl crate::face::StandardCollectorTrait for StandardCollector {
	#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StandardCollector {
	/// ## C++ default parameters
	/// * threshold_: DBL_MAX
	#[inline]
	pub fn new(threshold_: f64) -> Result<crate::face::StandardCollector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_StandardCollector_double(threshold_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::face::StandardCollector::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * threshold: DBL_MAX
	#[inline]
	pub fn create(threshold: f64) -> Result<core::Ptr<crate::face::StandardCollector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_create_double(threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::face::StandardCollector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StandardCollector_PredictResult {
	pub label: i32,
	pub distance: f64,
}

opencv_type_simple! { crate::face::StandardCollector_PredictResult }

impl StandardCollector_PredictResult {
	/// ## C++ default parameters
	/// * label_: -1
	/// * distance_: DBL_MAX
	#[inline]
	pub fn new(label_: i32, distance_: f64) -> Result<crate::face::StandardCollector_PredictResult> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_, distance_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
