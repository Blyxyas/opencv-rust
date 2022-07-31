#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Scene Text Detection and Recognition
//! 
//! The opencv_text module provides different algorithms for text detection and recognition in natural
//! scene images.
//!    # Scene Text Detection
//! 
//! Class-specific Extremal Regions for Scene Text Detection
//! --------------------------------------------------------
//! 
//! The scene text detection algorithm described below has been initially proposed by Lukás Neumann &
//! Jiri Matas [Neumann11](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Neumann11). The main idea behind Class-specific Extremal Regions is similar to the MSER
//! in that suitable Extremal Regions (ERs) are selected from the whole component tree of the image.
//! However, this technique differs from MSER in that selection of suitable ERs is done by a sequential
//! classifier trained for character detection, i.e. dropping the stability requirement of MSERs and
//! selecting class-specific (not necessarily stable) regions.
//! 
//! The component tree of an image is constructed by thresholding by an increasing value step-by-step
//! from 0 to 255 and then linking the obtained connected components from successive levels in a
//! hierarchy by their inclusion relation:
//! 
//! ![image](https://docs.opencv.org/4.6.0/component_tree.png)
//! 
//! The component tree may contain a huge number of regions even for a very simple image as shown in
//! the previous image. This number can easily reach the order of 1 x 10\^6 regions for an average 1
//! Megapixel image. In order to efficiently select suitable regions among all the ERs the algorithm
//! make use of a sequential classifier with two differentiated stages.
//! 
//! In the first stage incrementally computable descriptors (area, perimeter, bounding box, and Euler's
//! number) are computed (in O(1)) for each region r and used as features for a classifier which
//! estimates the class-conditional probability p(r|character). Only the ERs which correspond to local
//! maximum of the probability p(r|character) are selected (if their probability is above a global limit
//! p_min and the difference between local maximum and local minimum is greater than a delta_min
//! value).
//! 
//! In the second stage, the ERs that passed the first stage are classified into character and
//! non-character classes using more informative but also more computationally expensive features. (Hole
//! area ratio, convex hull ratio, and the number of outer boundary inflexion points).
//! 
//! This ER filtering process is done in different single-channel projections of the input image in
//! order to increase the character localization recall.
//! 
//! After the ER filtering is done on each input channel, character candidates must be grouped in
//! high-level text blocks (i.e. words, text lines, paragraphs, ...). The opencv_text module implements
//! two different grouping algorithms: the Exhaustive Search algorithm proposed in [Neumann12](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Neumann12) for
//! grouping horizontally aligned text, and the method proposed by Lluis Gomez and Dimosthenis Karatzas
//! in [Gomez13](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text (see erGrouping).
//! 
//! To see the text detector at work, have a look at the textdetection demo:
//! <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/textdetection.cpp>
//! 
//!    # Scene Text Recognition
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ERStatTraitConst, super::ERStatTrait, super::ERFilter_CallbackConst, super::ERFilter_Callback, super::ERFilterConst, super::ERFilter, super::BaseOCRConst, super::BaseOCR, super::OCRTesseractConst, super::OCRTesseract, super::OCRHMMDecoder_ClassifierCallbackTraitConst, super::OCRHMMDecoder_ClassifierCallbackTrait, super::OCRHMMDecoderTraitConst, super::OCRHMMDecoderTrait, super::OCRBeamSearchDecoder_ClassifierCallbackTraitConst, super::OCRBeamSearchDecoder_ClassifierCallbackTrait, super::OCRBeamSearchDecoderTraitConst, super::OCRBeamSearchDecoderTrait, super::OCRHolisticWordRecognizerConst, super::OCRHolisticWordRecognizer, super::TextDetectorConst, super::TextDetector, super::TextDetectorCNNConst, super::TextDetectorCNN };
}

pub const ERFILTER_NM_IHSGrad: i32 = 1;
pub const ERFILTER_NM_RGBLGrad: i32 = 0;
pub const ERGROUPING_ORIENTATION_ANY: i32 = 1;
pub const ERGROUPING_ORIENTATION_HORIZ: i32 = 0;
pub const OCR_CNN_CLASSIFIER: i32 = 1;
pub const OCR_DECODER_VITERBI: i32 = 0;
pub const OCR_KNN_CLASSIFIER: i32 = 0;
pub const OCR_LEVEL_TEXTLINE: i32 = 1;
pub const OCR_LEVEL_WORD: i32 = 0;
pub const OEM_CUBE_ONLY: i32 = 1;
pub const OEM_DEFAULT: i32 = 3;
pub const OEM_TESSERACT_CUBE_COMBINED: i32 = 2;
pub const OEM_TESSERACT_ONLY: i32 = 0;
pub const PSM_AUTO: i32 = 3;
pub const PSM_AUTO_ONLY: i32 = 2;
pub const PSM_AUTO_OSD: i32 = 1;
pub const PSM_CIRCLE_WORD: i32 = 9;
pub const PSM_OSD_ONLY: i32 = 0;
pub const PSM_SINGLE_BLOCK: i32 = 6;
pub const PSM_SINGLE_BLOCK_VERT_TEXT: i32 = 5;
pub const PSM_SINGLE_CHAR: i32 = 10;
pub const PSM_SINGLE_COLUMN: i32 = 4;
pub const PSM_SINGLE_LINE: i32 = 7;
pub const PSM_SINGLE_WORD: i32 = 8;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum classifier_type {
	OCR_KNN_CLASSIFIER = 0,
	OCR_CNN_CLASSIFIER = 1,
}

opencv_type_enum! { crate::text::classifier_type }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum decoder_mode {
	OCR_DECODER_VITERBI = 0,
}

opencv_type_enum! { crate::text::decoder_mode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum erGrouping_Modes {
	ERGROUPING_ORIENTATION_HORIZ = 0,
	ERGROUPING_ORIENTATION_ANY = 1,
}

opencv_type_enum! { crate::text::erGrouping_Modes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ocr_engine_mode {
	OEM_TESSERACT_ONLY = 0,
	OEM_CUBE_ONLY = 1,
	OEM_TESSERACT_CUBE_COMBINED = 2,
	OEM_DEFAULT = 3,
}

opencv_type_enum! { crate::text::ocr_engine_mode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum page_seg_mode {
	PSM_OSD_ONLY = 0,
	PSM_AUTO_OSD = 1,
	PSM_AUTO_ONLY = 2,
	PSM_AUTO = 3,
	PSM_SINGLE_COLUMN = 4,
	PSM_SINGLE_BLOCK_VERT_TEXT = 5,
	PSM_SINGLE_BLOCK = 6,
	PSM_SINGLE_LINE = 7,
	PSM_SINGLE_WORD = 8,
	PSM_CIRCLE_WORD = 9,
	PSM_SINGLE_CHAR = 10,
}

opencv_type_enum! { crate::text::page_seg_mode }

#[inline]
pub fn mse_rs_to_er_stats(image: &dyn core::ToInputArray, contours: &mut core::Vector<core::Vector<core::Point>>, regions: &mut core::Vector<core::Vector<crate::text::ERStat>>) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_MSERsToERStats_const__InputArrayR_vector_vector_Point__R_vector_vector_ERStat__R(image.as_raw__InputArray(), contours.as_raw_mut_VectorOfVectorOfPoint(), regions.as_raw_mut_VectorOfVectorOfERStat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * _mode: ERFILTER_NM_RGBLGrad
#[inline]
pub fn compute_nm_channels(_src: &dyn core::ToInputArray, _channels: &mut dyn core::ToOutputArray, _mode: i32) -> Result<()> {
	input_array_arg!(_src);
	output_array_arg!(_channels);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR_int(_src.as_raw__InputArray(), _channels.as_raw__OutputArray(), _mode, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * threshold_delta: 1
/// * min_area: (float)0.00025
/// * max_area: (float)0.13
/// * min_probability: (float)0.4
/// * non_max_suppression: true
/// * min_probability_diff: (float)0.1
#[inline]
pub fn create_er_filter_nm1(cb: &core::Ptr<dyn crate::text::ERFilter_Callback>, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<core::Ptr<dyn crate::text::ERFilter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createERFilterNM1_const_Ptr_Callback_R_int_float_float_float_bool_float(cb.as_raw_PtrOfERFilter_Callback(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * threshold_delta: 1
/// * min_area: (float)0.00025
/// * max_area: (float)0.13
/// * min_probability: (float)0.4
/// * non_max_suppression: true
/// * min_probability_diff: (float)0.1
#[inline]
pub fn create_er_filter_nm1_from_file(filename: &str, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<core::Ptr<dyn crate::text::ERFilter>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float(filename.opencv_as_extern(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * min_probability: (float)0.3
#[inline]
pub fn create_er_filter_nm2(cb: &core::Ptr<dyn crate::text::ERFilter_Callback>, min_probability: f32) -> Result<core::Ptr<dyn crate::text::ERFilter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createERFilterNM2_const_Ptr_Callback_R_float(cb.as_raw_PtrOfERFilter_Callback(), min_probability, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * min_probability: (float)0.3
#[inline]
pub fn create_er_filter_nm2_from_file(filename: &str, min_probability: f32) -> Result<core::Ptr<dyn crate::text::ERFilter>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createERFilterNM2_const_StringR_float(filename.opencv_as_extern(), min_probability, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_ocrhmm_transitions_table_1(vocabulary: &str, lexicon: &mut core::Vector<String>) -> Result<core::Mat> {
	extern_container_arg!(vocabulary);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createOCRHMMTransitionsTable_const_StringR_vector_String_R(vocabulary.opencv_as_extern(), lexicon.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn create_ocrhmm_transitions_table(vocabulary: &mut String, lexicon: &mut core::Vector<String>, transition_probabilities_table: &mut dyn core::ToOutputArray) -> Result<()> {
	string_arg_output_send!(via vocabulary_via);
	output_array_arg!(transition_probabilities_table);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_createOCRHMMTransitionsTable_stringR_vector_string_R_const__OutputArrayR(&mut vocabulary_via, lexicon.as_raw_mut_VectorOfString(), transition_probabilities_table.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	string_arg_output_receive!(vocabulary_via => vocabulary);
	Ok(ret)
}

/// ## C++ default parameters
/// * method: ERGROUPING_ORIENTATION_HORIZ
/// * filename: String()
/// * min_probability: (float)0.5
#[inline]
pub fn detect_regions_from_file(image: &dyn core::ToInputArray, er_filter1: &core::Ptr<dyn crate::text::ERFilter>, er_filter2: &core::Ptr<dyn crate::text::ERFilter>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probability: f32) -> Result<()> {
	input_array_arg!(image);
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_detectRegions_const__InputArrayR_const_Ptr_ERFilter_R_const_Ptr_ERFilter_R_vector_Rect_R_int_const_StringR_float(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probability, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn detect_regions(image: &dyn core::ToInputArray, er_filter1: &core::Ptr<dyn crate::text::ERFilter>, er_filter2: &core::Ptr<dyn crate::text::ERFilter>, regions: &mut core::Vector<core::Vector<core::Point>>) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_detectRegions_const__InputArrayR_const_Ptr_ERFilter_R_const_Ptr_ERFilter_R_vector_vector_Point__R(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), regions.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * draw: noArray()
/// * chain_b_bs: noArray()
#[inline]
pub fn detect_text_swt(input: &dyn core::ToInputArray, result: &mut core::Vector<core::Rect>, dark_on_light: bool, draw: &mut dyn core::ToOutputArray, chain_b_bs: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(input);
	output_array_arg!(draw);
	output_array_arg!(chain_b_bs);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_detectTextSWT_const__InputArrayR_vector_Rect_R_bool_const__OutputArrayR_const__OutputArrayR(input.as_raw__InputArray(), result.as_raw_mut_VectorOfRect(), dark_on_light, draw.as_raw__OutputArray(), chain_b_bs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * method: ERGROUPING_ORIENTATION_HORIZ
/// * filename: std::string()
/// * min_probablity: 0.5
#[inline]
pub fn er_grouping(img: &dyn core::ToInputArray, channels: &dyn core::ToInputArray, regions: &mut core::Vector<core::Vector<crate::text::ERStat>>, groups: &mut core::Vector<core::Vector<core::Vec2i>>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
	input_array_arg!(img);
	input_array_arg!(channels);
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vector_vector_ERStat__R_vector_vector_Vec2i__R_vector_Rect_R_int_const_stringR_float(img.as_raw__InputArray(), channels.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfERStat(), groups.as_raw_mut_VectorOfVectorOfVec2i(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probablity, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * method: ERGROUPING_ORIENTATION_HORIZ
/// * filename: String()
/// * min_probablity: (float)0.5
#[inline]
pub fn er_grouping_1(image: &dyn core::ToInputArray, channel: &dyn core::ToInputArray, mut regions: core::Vector<core::Vector<core::Point>>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(channel);
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vector_vector_Point___vector_Rect_R_int_const_StringR_float(image.as_raw__InputArray(), channel.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfPoint(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probablity, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn load_classifier_nm1(filename: &str) -> Result<core::Ptr<dyn crate::text::ERFilter_Callback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadClassifierNM1_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter_Callback>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn load_classifier_nm2(filename: &str) -> Result<core::Ptr<dyn crate::text::ERFilter_Callback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadClassifierNM2_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::text::ERFilter_Callback>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn load_ocr_beam_search_classifier_cnn(filename: &str) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadOCRBeamSearchClassifierCNN_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder_ClassifierCallback>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn load_ocrhmm_classifier_cnn(filename: &str) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadOCRHMMClassifierCNN_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn load_ocrhmm_classifier_nm(filename: &str) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadOCRHMMClassifierNM_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn load_ocrhmm_classifier(filename: &str, classifier: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_text_loadOCRHMMClassifier_const_StringR_int(filename.opencv_as_extern(), classifier, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
	Ok(ret)
}

pub trait BaseOCRConst {
	fn as_raw_BaseOCR(&self) -> *const c_void;

}

pub trait BaseOCR: crate::text::BaseOCRConst {
	fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_BaseOCR_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_BaseOCR_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
}

pub trait ERFilterConst: core::AlgorithmTraitConst {
	fn as_raw_ERFilter(&self) -> *const c_void;

	#[inline]
	fn get_num_rejected(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_getNumRejected_const(self.as_raw_ERFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ERFilter: core::AlgorithmTrait + crate::text::ERFilterConst {
	fn as_raw_mut_ERFilter(&mut self) -> *mut c_void;

	#[inline]
	fn run(&mut self, image: &dyn core::ToInputArray, regions: &mut core::Vector<crate::text::ERStat>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_run_const__InputArrayR_vector_ERStat_R(self.as_raw_mut_ERFilter(), image.as_raw__InputArray(), regions.as_raw_mut_VectorOfERStat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_callback(&mut self, cb: &core::Ptr<dyn crate::text::ERFilter_Callback>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setCallback_const_Ptr_Callback_R(self.as_raw_mut_ERFilter(), cb.as_raw_PtrOfERFilter_Callback(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_threshold_delta(&mut self, threshold_delta: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setThresholdDelta_int(self.as_raw_mut_ERFilter(), threshold_delta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_area(&mut self, min_area: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setMinArea_float(self.as_raw_mut_ERFilter(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_max_area(&mut self, max_area: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setMaxArea_float(self.as_raw_mut_ERFilter(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_probability(&mut self, min_probability: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setMinProbability_float(self.as_raw_mut_ERFilter(), min_probability, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_min_probability_diff(&mut self, min_probability_diff: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setMinProbabilityDiff_float(self.as_raw_mut_ERFilter(), min_probability_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_non_max_suppression(&mut self, non_max_suppression: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_setNonMaxSuppression_bool(self.as_raw_mut_ERFilter(), non_max_suppression, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ERFilter_CallbackConst {
	fn as_raw_ERFilter_Callback(&self) -> *const c_void;

}

pub trait ERFilter_Callback: crate::text::ERFilter_CallbackConst {
	fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void;

	#[inline]
	fn eval(&mut self, stat: &crate::text::ERStat) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERFilter_Callback_eval_const_ERStatR(self.as_raw_mut_ERFilter_Callback(), stat.as_raw_ERStat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ERStatTraitConst {
	fn as_raw_ERStat(&self) -> *const c_void;

	#[inline]
	fn pixel(&self) -> i32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropPixel_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn level(&self) -> i32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropLevel_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn area(&self) -> i32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropArea_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn perimeter(&self) -> i32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropPerimeter_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn euler(&self) -> i32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropEuler_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn rect(&self) -> core::Rect {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERStat_getPropRect_const(self.as_raw_ERStat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn med_crossings(&self) -> f32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropMed_crossings_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn hole_area_ratio(&self) -> f32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropHole_area_ratio_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn convex_hull_ratio(&self) -> f32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropConvex_hull_ratio_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn num_inflexion_points(&self) -> f32 {
		let ret = unsafe { sys::cv_text_ERStat_getPropNum_inflexion_points_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn probability(&self) -> f64 {
		let ret = unsafe { sys::cv_text_ERStat_getPropProbability_const(self.as_raw_ERStat()) };
		ret
	}
	
	#[inline]
	fn local_maxima(&self) -> bool {
		let ret = unsafe { sys::cv_text_ERStat_getPropLocal_maxima_const(self.as_raw_ERStat()) };
		ret
	}
	
}

pub trait ERStatTrait: crate::text::ERStatTraitConst {
	fn as_raw_mut_ERStat(&mut self) -> *mut c_void;

	#[inline]
	fn set_pixel(&mut self, val: i32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropPixel_int(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropLevel_int(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_area(&mut self, val: i32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropArea_int(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_perimeter(&mut self, val: i32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropPerimeter_int(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_euler(&mut self, val: i32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropEuler_int(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_rect(&mut self, val: core::Rect) {
		let ret = unsafe { sys::cv_text_ERStat_setPropRect_Rect(self.as_raw_mut_ERStat(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn raw_moments(&mut self) -> &mut [f64; 2] {
		let ret = unsafe { sys::cv_text_ERStat_getPropRaw_moments(self.as_raw_mut_ERStat()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
	#[inline]
	fn central_moments(&mut self) -> &mut [f64; 3] {
		let ret = unsafe { sys::cv_text_ERStat_getPropCentral_moments(self.as_raw_mut_ERStat()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
	#[inline]
	fn set_med_crossings(&mut self, val: f32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropMed_crossings_float(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_hole_area_ratio(&mut self, val: f32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropHole_area_ratio_float(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_convex_hull_ratio(&mut self, val: f32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropConvex_hull_ratio_float(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_num_inflexion_points(&mut self, val: f32) {
		let ret = unsafe { sys::cv_text_ERStat_setPropNum_inflexion_points_float(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn set_probability(&mut self, val: f64) {
		let ret = unsafe { sys::cv_text_ERStat_setPropProbability_double(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn parent(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropParent(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_parent(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropParent_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
	#[inline]
	fn child(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropChild(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_child(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropChild_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
	#[inline]
	fn next(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropNext(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_next(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropNext_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
	#[inline]
	fn prev(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropPrev(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_prev(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropPrev_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
	#[inline]
	fn set_local_maxima(&mut self, val: bool) {
		let ret = unsafe { sys::cv_text_ERStat_setPropLocal_maxima_bool(self.as_raw_mut_ERStat(), val) };
		ret
	}
	
	#[inline]
	fn max_probability_ancestor(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropMax_probability_ancestor(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_max_probability_ancestor(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropMax_probability_ancestor_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
	#[inline]
	fn min_probability_ancestor(&mut self) -> crate::text::ERStat {
		let ret = unsafe { sys::cv_text_ERStat_getPropMin_probability_ancestor(self.as_raw_mut_ERStat()) };
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn set_min_probability_ancestor(&mut self, val: &mut crate::text::ERStat) {
		let ret = unsafe { sys::cv_text_ERStat_setPropMin_probability_ancestor_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
		ret
	}
	
}

pub struct ERStat {
	ptr: *mut c_void
}

opencv_type_boxed! { ERStat }

impl Drop for ERStat {
	fn drop(&mut self) {
		extern "C" { fn cv_ERStat_delete(instance: *mut c_void); }
		unsafe { cv_ERStat_delete(self.as_raw_mut_ERStat()) };
	}
}

unsafe impl Send for ERStat {}

impl crate::text::ERStatTraitConst for ERStat {
	#[inline] fn as_raw_ERStat(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::ERStatTrait for ERStat {
	#[inline] fn as_raw_mut_ERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ERStat {
	/// ## C++ default parameters
	/// * level: 256
	/// * pixel: 0
	/// * x: 0
	/// * y: 0
	#[inline]
	pub fn new(level: i32, pixel: i32, x: i32, y: i32) -> Result<crate::text::ERStat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_ERStat_ERStat_int_int_int_int(level, pixel, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait OCRBeamSearchDecoderTraitConst: crate::text::BaseOCRConst {
	fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void;

}

pub trait OCRBeamSearchDecoderTrait: crate::text::BaseOCR + crate::text::OCRBeamSearchDecoderTraitConst {
	fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run_mask(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct OCRBeamSearchDecoder {
	ptr: *mut c_void
}

opencv_type_boxed! { OCRBeamSearchDecoder }

impl Drop for OCRBeamSearchDecoder {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRBeamSearchDecoder_delete(instance: *mut c_void); }
		unsafe { cv_OCRBeamSearchDecoder_delete(self.as_raw_mut_OCRBeamSearchDecoder()) };
	}
}

unsafe impl Send for OCRBeamSearchDecoder {}

impl crate::text::BaseOCRConst for OCRBeamSearchDecoder {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::BaseOCR for OCRBeamSearchDecoder {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRBeamSearchDecoderTraitConst for OCRBeamSearchDecoder {
	#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::OCRBeamSearchDecoderTrait for OCRBeamSearchDecoder {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OCRBeamSearchDecoder {
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	/// * beam_size: 500
	#[inline]
	pub fn create(classifier: core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: crate::text::decoder_mode, beam_size: i32) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
		extern_container_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_Ptr_ClassifierCallback__const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(classifier.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	/// * beam_size: 500
	#[inline]
	pub fn create_from_file(filename: &str, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: crate::text::decoder_mode, beam_size: i32) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
		extern_container_arg!(filename);
		extern_container_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait OCRBeamSearchDecoder_ClassifierCallbackTraitConst {
	fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void;

}

pub trait OCRBeamSearchDecoder_ClassifierCallbackTrait: crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst {
	fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void;

	#[inline]
	fn eval(&mut self, image: &dyn core::ToInputArray, recognition_probabilities: &mut core::Vector<core::Vector<f64>>, oversegmentation: &mut core::Vector<i32>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vector_vector_double__R_vector_int_R(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), image.as_raw__InputArray(), recognition_probabilities.as_raw_mut_VectorOfVectorOff64(), oversegmentation.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_window_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_step_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct OCRBeamSearchDecoder_ClassifierCallback {
	ptr: *mut c_void
}

opencv_type_boxed! { OCRBeamSearchDecoder_ClassifierCallback }

impl Drop for OCRBeamSearchDecoder_ClassifierCallback {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRBeamSearchDecoder_ClassifierCallback_delete(instance: *mut c_void); }
		unsafe { cv_OCRBeamSearchDecoder_ClassifierCallback_delete(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback()) };
	}
}

unsafe impl Send for OCRBeamSearchDecoder_ClassifierCallback {}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for OCRBeamSearchDecoder_ClassifierCallback {
	#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for OCRBeamSearchDecoder_ClassifierCallback {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OCRBeamSearchDecoder_ClassifierCallback {
}

pub trait OCRHMMDecoderTraitConst: crate::text::BaseOCRConst {
	fn as_raw_OCRHMMDecoder(&self) -> *const c_void;

}

pub trait OCRHMMDecoderTrait: crate::text::BaseOCR + crate::text::OCRHMMDecoderTraitConst {
	fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run_mask(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct OCRHMMDecoder {
	ptr: *mut c_void
}

opencv_type_boxed! { OCRHMMDecoder }

impl Drop for OCRHMMDecoder {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRHMMDecoder_delete(instance: *mut c_void); }
		unsafe { cv_OCRHMMDecoder_delete(self.as_raw_mut_OCRHMMDecoder()) };
	}
}

unsafe impl Send for OCRHMMDecoder {}

impl crate::text::BaseOCRConst for OCRHMMDecoder {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::BaseOCR for OCRHMMDecoder {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRHMMDecoderTraitConst for OCRHMMDecoder {
	#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::OCRHMMDecoderTrait for OCRHMMDecoder {
	#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OCRHMMDecoder {
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	#[inline]
	pub fn create(classifier: core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
		extern_container_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_create_const_Ptr_ClassifierCallback__const_StringR_const__InputArrayR_const__InputArrayR_int(classifier.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	/// * classifier: OCR_KNN_CLASSIFIER
	#[inline]
	pub fn create_from_file(filename: &str, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32, classifier: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
		extern_container_arg!(filename);
		extern_container_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, classifier, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait OCRHMMDecoder_ClassifierCallbackTraitConst {
	fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void;

}

pub trait OCRHMMDecoder_ClassifierCallbackTrait: crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst {
	fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void;

	#[inline]
	fn eval(&mut self, image: &dyn core::ToInputArray, out_class: &mut core::Vector<i32>, out_confidence: &mut core::Vector<f64>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vector_int_R_vector_double_R(self.as_raw_mut_OCRHMMDecoder_ClassifierCallback(), image.as_raw__InputArray(), out_class.as_raw_mut_VectorOfi32(), out_confidence.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct OCRHMMDecoder_ClassifierCallback {
	ptr: *mut c_void
}

opencv_type_boxed! { OCRHMMDecoder_ClassifierCallback }

impl Drop for OCRHMMDecoder_ClassifierCallback {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRHMMDecoder_ClassifierCallback_delete(instance: *mut c_void); }
		unsafe { cv_OCRHMMDecoder_ClassifierCallback_delete(self.as_raw_mut_OCRHMMDecoder_ClassifierCallback()) };
	}
}

unsafe impl Send for OCRHMMDecoder_ClassifierCallback {}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for OCRHMMDecoder_ClassifierCallback {
	#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for OCRHMMDecoder_ClassifierCallback {
	#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OCRHMMDecoder_ClassifierCallback {
}

pub trait OCRHolisticWordRecognizerConst: crate::text::BaseOCRConst {
	fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void;

}

pub trait OCRHolisticWordRecognizer: crate::text::BaseOCR + crate::text::OCRHolisticWordRecognizerConst {
	fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: OCR_LEVEL_WORD
	#[inline]
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: OCR_LEVEL_WORD
	#[inline]
	fn run_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
}

impl dyn OCRHolisticWordRecognizer + '_ {
	#[inline]
	pub fn create(arch_filename: &str, weights_filename: &str, words_filename: &str) -> Result<core::Ptr<dyn crate::text::OCRHolisticWordRecognizer>> {
		extern_container_arg!(arch_filename);
		extern_container_arg!(weights_filename);
		extern_container_arg!(words_filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(arch_filename.opencv_as_extern(), weights_filename.opencv_as_extern(), words_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::text::OCRHolisticWordRecognizer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait OCRTesseractConst: crate::text::BaseOCRConst {
	fn as_raw_OCRTesseract(&self) -> *const c_void;

}

pub trait OCRTesseract: crate::text::BaseOCR + crate::text::OCRTesseractConst {
	fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	#[inline]
	fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(output_text_via => output_text);
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_int_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	#[inline]
	fn run_mask(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_white_list(&mut self, char_whitelist: &str) -> Result<()> {
		extern_container_arg!(char_whitelist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_setWhiteList_const_StringR(self.as_raw_mut_OCRTesseract(), char_whitelist.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn OCRTesseract + '_ {
	/// ## C++ default parameters
	/// * datapath: NULL
	/// * language: NULL
	/// * char_whitelist: NULL
	/// * oem: OEM_DEFAULT
	/// * psmode: PSM_AUTO
	#[inline]
	pub fn create(datapath: &str, language: &str, char_whitelist: &str, oem: i32, psmode: i32) -> Result<core::Ptr<dyn crate::text::OCRTesseract>> {
		extern_container_arg!(datapath);
		extern_container_arg!(language);
		extern_container_arg!(char_whitelist);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(datapath.opencv_as_extern(), language.opencv_as_extern(), char_whitelist.opencv_as_extern(), oem, psmode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::text::OCRTesseract>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait TextDetectorConst {
	fn as_raw_TextDetector(&self) -> *const c_void;

}

pub trait TextDetector: crate::text::TextDetectorConst {
	fn as_raw_mut_TextDetector(&mut self) -> *mut c_void;

	#[inline]
	fn detect(&mut self, input_image: &dyn core::ToInputArray, bbox: &mut core::Vector<core::Rect>, confidence: &mut core::Vector<f32>) -> Result<()> {
		input_array_arg!(input_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_TextDetector_detect_const__InputArrayR_vector_Rect_R_vector_float_R(self.as_raw_mut_TextDetector(), input_image.as_raw__InputArray(), bbox.as_raw_mut_VectorOfRect(), confidence.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TextDetectorCNNConst: crate::text::TextDetectorConst {
	fn as_raw_TextDetectorCNN(&self) -> *const c_void;

}

pub trait TextDetectorCNN: crate::text::TextDetector + crate::text::TextDetectorCNNConst {
	fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void;

	#[inline]
	fn detect(&mut self, input_image: &dyn core::ToInputArray, bbox: &mut core::Vector<core::Rect>, confidence: &mut core::Vector<f32>) -> Result<()> {
		input_array_arg!(input_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_TextDetectorCNN_detect_const__InputArrayR_vector_Rect_R_vector_float_R(self.as_raw_mut_TextDetectorCNN(), input_image.as_raw__InputArray(), bbox.as_raw_mut_VectorOfRect(), confidence.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn TextDetectorCNN + '_ {
	#[inline]
	pub fn create_with_sizes(model_arch_filename: &str, model_weights_filename: &str, mut detection_sizes: core::Vector<core::Size>) -> Result<core::Ptr<dyn crate::text::TextDetectorCNN>> {
		extern_container_arg!(model_arch_filename);
		extern_container_arg!(model_weights_filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vector_Size_(model_arch_filename.opencv_as_extern(), model_weights_filename.opencv_as_extern(), detection_sizes.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::text::TextDetectorCNN>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create(model_arch_filename: &str, model_weights_filename: &str) -> Result<core::Ptr<dyn crate::text::TextDetectorCNN>> {
		extern_container_arg!(model_arch_filename);
		extern_container_arg!(model_weights_filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_TextDetectorCNN_create_const_StringR_const_StringR(model_arch_filename.opencv_as_extern(), model_weights_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::text::TextDetectorCNN>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}