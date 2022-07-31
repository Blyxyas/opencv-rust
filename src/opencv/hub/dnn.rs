#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Deep Neural Network module
//!   This module contains:
//!       - API for new layers creation, layers are building bricks of neural networks;
//!       - set of built-in most-useful Layers;
//!       - API to construct and modify comprehensive neural networks from layers;
//!       - functionality for loading serialized networks models from different frameworks.
//! 
//!   Functionality of this module is designed only for forward pass computations (i.e. network testing).
//!   A network training is in principle not supported.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DictValueTraitConst, super::DictValueTrait, super::DictTraitConst, super::DictTrait, super::LayerParamsTraitConst, super::LayerParamsTrait, super::BackendNodeTraitConst, super::BackendNodeTrait, super::BackendWrapperConst, super::BackendWrapper, super::LayerTraitConst, super::LayerTrait, super::NetTraitConst, super::NetTrait, super::ModelTraitConst, super::ModelTrait, super::ClassificationModelTraitConst, super::ClassificationModelTrait, super::KeypointsModelTraitConst, super::KeypointsModelTrait, super::SegmentationModelTraitConst, super::SegmentationModelTrait, super::DetectionModelTraitConst, super::DetectionModelTrait, super::TextRecognitionModelTraitConst, super::TextRecognitionModelTrait, super::TextDetectionModelTraitConst, super::TextDetectionModelTrait, super::TextDetectionModel_EASTTraitConst, super::TextDetectionModel_EASTTrait, super::TextDetectionModel_DBTraitConst, super::TextDetectionModel_DBTrait, super::LayerFactoryTraitConst, super::LayerFactoryTrait, super::BlankLayerTraitConst, super::BlankLayerTrait, super::ConstLayerTraitConst, super::ConstLayerTrait, super::LSTMLayerConst, super::LSTMLayer, super::GRULayerTraitConst, super::GRULayerTrait, super::RNNLayerConst, super::RNNLayer, super::BaseConvolutionLayerTraitConst, super::BaseConvolutionLayerTrait, super::ConvolutionLayerTraitConst, super::ConvolutionLayerTrait, super::ConvolutionLayerInt8TraitConst, super::ConvolutionLayerInt8Trait, super::DeconvolutionLayerTraitConst, super::DeconvolutionLayerTrait, super::LRNLayerTraitConst, super::LRNLayerTrait, super::ArgLayerTraitConst, super::ArgLayerTrait, super::PoolingLayerTraitConst, super::PoolingLayerTrait, super::PoolingLayerInt8TraitConst, super::PoolingLayerInt8Trait, super::ReduceLayerTraitConst, super::ReduceLayerTrait, super::ReduceLayerInt8TraitConst, super::ReduceLayerInt8Trait, super::SoftmaxLayerTraitConst, super::SoftmaxLayerTrait, super::SoftmaxLayerInt8TraitConst, super::SoftmaxLayerInt8Trait, super::InnerProductLayerTraitConst, super::InnerProductLayerTrait, super::InnerProductLayerInt8TraitConst, super::InnerProductLayerInt8Trait, super::MVNLayerTraitConst, super::MVNLayerTrait, super::ReshapeLayerTraitConst, super::ReshapeLayerTrait, super::FlattenLayerTraitConst, super::FlattenLayerTrait, super::QuantizeLayerTraitConst, super::QuantizeLayerTrait, super::DequantizeLayerTraitConst, super::DequantizeLayerTrait, super::RequantizeLayerTraitConst, super::RequantizeLayerTrait, super::ConcatLayerTraitConst, super::ConcatLayerTrait, super::SplitLayerTraitConst, super::SplitLayerTrait, super::SliceLayerTraitConst, super::SliceLayerTrait, super::PermuteLayerTraitConst, super::PermuteLayerTrait, super::ShuffleChannelLayerTraitConst, super::ShuffleChannelLayerTrait, super::PaddingLayerTraitConst, super::PaddingLayerTrait, super::ActivationLayerTraitConst, super::ActivationLayerTrait, super::ReLULayerTraitConst, super::ReLULayerTrait, super::ReLU6LayerTraitConst, super::ReLU6LayerTrait, super::ChannelsPReLULayerTraitConst, super::ChannelsPReLULayerTrait, super::ELULayerTraitConst, super::ELULayerTrait, super::TanHLayerTraitConst, super::TanHLayerTrait, super::SwishLayerTraitConst, super::SwishLayerTrait, super::MishLayerTraitConst, super::MishLayerTrait, super::SigmoidLayerTraitConst, super::SigmoidLayerTrait, super::BNLLLayerTraitConst, super::BNLLLayerTrait, super::AbsLayerTraitConst, super::AbsLayerTrait, super::PowerLayerTraitConst, super::PowerLayerTrait, super::ExpLayerTraitConst, super::ExpLayerTrait, super::CeilLayerTraitConst, super::CeilLayerTrait, super::FloorLayerTraitConst, super::FloorLayerTrait, super::LogLayerTraitConst, super::LogLayerTrait, super::RoundLayerTraitConst, super::RoundLayerTrait, super::SqrtLayerTraitConst, super::SqrtLayerTrait, super::NotLayerTraitConst, super::NotLayerTrait, super::AcosLayerTraitConst, super::AcosLayerTrait, super::AcoshLayerTraitConst, super::AcoshLayerTrait, super::AsinLayerTraitConst, super::AsinLayerTrait, super::AsinhLayerTraitConst, super::AsinhLayerTrait, super::AtanLayerTraitConst, super::AtanLayerTrait, super::AtanhLayerTraitConst, super::AtanhLayerTrait, super::CosLayerTraitConst, super::CosLayerTrait, super::CoshLayerTraitConst, super::CoshLayerTrait, super::ErfLayerTraitConst, super::ErfLayerTrait, super::HardSwishLayerTraitConst, super::HardSwishLayerTrait, super::SinLayerTraitConst, super::SinLayerTrait, super::SinhLayerTraitConst, super::SinhLayerTrait, super::SoftplusLayerTraitConst, super::SoftplusLayerTrait, super::SoftsignLayerTraitConst, super::SoftsignLayerTrait, super::TanLayerTraitConst, super::TanLayerTrait, super::CeluLayerTraitConst, super::CeluLayerTrait, super::HardSigmoidLayerTraitConst, super::HardSigmoidLayerTrait, super::SeluLayerTraitConst, super::SeluLayerTrait, super::ThresholdedReluLayerTraitConst, super::ThresholdedReluLayerTrait, super::ActivationLayerInt8TraitConst, super::ActivationLayerInt8Trait, super::SignLayerTraitConst, super::SignLayerTrait, super::ShrinkLayerTraitConst, super::ShrinkLayerTrait, super::ReciprocalLayerTraitConst, super::ReciprocalLayerTrait, super::CropLayerTraitConst, super::CropLayerTrait, super::EltwiseLayerTraitConst, super::EltwiseLayerTrait, super::EltwiseLayerInt8TraitConst, super::EltwiseLayerInt8Trait, super::BatchNormLayerTraitConst, super::BatchNormLayerTrait, super::BatchNormLayerInt8TraitConst, super::BatchNormLayerInt8Trait, super::MaxUnpoolLayerTraitConst, super::MaxUnpoolLayerTrait, super::ScaleLayerTraitConst, super::ScaleLayerTrait, super::ScaleLayerInt8TraitConst, super::ScaleLayerInt8Trait, super::ShiftLayerTraitConst, super::ShiftLayerTrait, super::ShiftLayerInt8TraitConst, super::ShiftLayerInt8Trait, super::CompareLayerTraitConst, super::CompareLayerTrait, super::DataAugmentationLayerTraitConst, super::DataAugmentationLayerTrait, super::CorrelationLayerTraitConst, super::CorrelationLayerTrait, super::AccumLayerTraitConst, super::AccumLayerTrait, super::FlowWarpLayerTraitConst, super::FlowWarpLayerTrait, super::PriorBoxLayerTraitConst, super::PriorBoxLayerTrait, super::ReorgLayerTraitConst, super::ReorgLayerTrait, super::RegionLayerTraitConst, super::RegionLayerTrait, super::DetectionOutputLayerTraitConst, super::DetectionOutputLayerTrait, super::NormalizeBBoxLayerTraitConst, super::NormalizeBBoxLayerTrait, super::ResizeLayerTraitConst, super::ResizeLayerTrait, super::InterpLayerTraitConst, super::InterpLayerTrait, super::ProposalLayerTraitConst, super::ProposalLayerTrait, super::CropAndResizeLayerTraitConst, super::CropAndResizeLayerTrait, super::CumSumLayerTraitConst, super::CumSumLayerTrait, super::_RangeTraitConst, super::_RangeTrait };
}

pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: &str = "NGRAPH";
pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API: &str = "NN_BUILDER";
pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_ARM_COMPUTE: &str = "ARM_COMPUTE";
pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_X86: &str = "X86";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2: &str = "Myriad2";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X: &str = "MyriadX";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED: &str = "";
pub const DNN_BACKEND_CUDA: i32 = 5;
pub const DNN_BACKEND_DEFAULT: i32 = 0;
pub const DNN_BACKEND_HALIDE: i32 = 1;
pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 2;
pub const DNN_BACKEND_OPENCV: i32 = 3;
pub const DNN_BACKEND_TIMVX: i32 = 7;
pub const DNN_BACKEND_VKCOM: i32 = 4;
pub const DNN_BACKEND_WEBNN: i32 = 6;
pub const DNN_TARGET_CPU: i32 = 0;
pub const DNN_TARGET_CUDA: i32 = 6;
pub const DNN_TARGET_CUDA_FP16: i32 = 7;
pub const DNN_TARGET_FPGA: i32 = 5;
pub const DNN_TARGET_HDDL: i32 = 8;
pub const DNN_TARGET_MYRIAD: i32 = 3;
pub const DNN_TARGET_NPU: i32 = 9;
pub const DNN_TARGET_OPENCL: i32 = 1;
pub const DNN_TARGET_OPENCL_FP16: i32 = 2;
pub const DNN_TARGET_VULKAN: i32 = 4;
pub const OPENCV_DNN_API_VERSION: i32 = 20220524;
pub const SoftNMSMethod_SOFTNMS_GAUSSIAN: i32 = 2;
pub const SoftNMSMethod_SOFTNMS_LINEAR: i32 = 1;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Backend {
	DNN_BACKEND_DEFAULT = 0,
	DNN_BACKEND_HALIDE = 1,
	DNN_BACKEND_INFERENCE_ENGINE = 2,
	DNN_BACKEND_OPENCV = 3,
	DNN_BACKEND_VKCOM = 4,
	DNN_BACKEND_CUDA = 5,
	DNN_BACKEND_WEBNN = 6,
	DNN_BACKEND_TIMVX = 7,
}

opencv_type_enum! { crate::dnn::Backend }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SoftNMSMethod {
	SOFTNMS_LINEAR = 1,
	SOFTNMS_GAUSSIAN = 2,
}

opencv_type_enum! { crate::dnn::SoftNMSMethod }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Target {
	DNN_TARGET_CPU = 0,
	DNN_TARGET_OPENCL = 1,
	DNN_TARGET_OPENCL_FP16 = 2,
	DNN_TARGET_MYRIAD = 3,
	DNN_TARGET_VULKAN = 4,
	DNN_TARGET_FPGA = 5,
	DNN_TARGET_CUDA = 6,
	DNN_TARGET_CUDA_FP16 = 7,
	DNN_TARGET_HDDL = 8,
	DNN_TARGET_NPU = 9,
}

opencv_type_enum! { crate::dnn::Target }

pub type LayerFactory_Constructor = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
pub type MatShape = core::Vector<i32>;
pub type Net_LayerId = crate::dnn::DictValue;
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
#[inline]
pub fn nms_boxes_f64(bboxes: &core::Vector<core::Rect2d>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_Rect2d_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
#[inline]
pub fn nms_boxes(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_Rect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
#[inline]
pub fn nms_boxes_rotated(bboxes: &core::Vector<core::RotatedRect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_RotatedRect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
#[inline]
pub fn blob_from_image_to(image: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
#[inline]
pub fn blob_from_image(image: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
#[inline]
pub fn blob_from_images_to(images: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(images);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, size.opencv_as_extern(), &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
#[inline]
pub fn blob_from_images(images: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(images);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), scalefactor, size.opencv_as_extern(), &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn concat(a: &crate::dnn::MatShape, b: &crate::dnn::MatShape) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_concat_const_MatShapeR_const_MatShapeR(a.as_raw_VectorOfi32(), b.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn enable_model_diagnostics(is_diagnostics_mode: bool) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_enableModelDiagnostics_bool(is_diagnostics_mode, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn get_available_targets(be: crate::dnn::Backend) -> Result<core::Vector<crate::dnn::Target>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getAvailableTargets_Backend(be, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::dnn::Target>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_inference_engine_backend_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineBackendType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_inference_engine_cpu_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineCPUType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_inference_engine_vpu_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineVPUType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn get_plane(m: &core::Mat, n: i32, cn: i32) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getPlane_const_MatR_int_int(m.as_raw_Mat(), n, cn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn images_from_blob(blob_: &core::Mat, images_: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(images_);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(blob_.as_raw_Mat(), images_.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * caffe_model: String()
#[inline]
pub fn read_net_from_caffe(prototxt: &str, caffe_model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(prototxt);
	extern_container_arg!(caffe_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringR_const_StringR(prototxt.opencv_as_extern(), caffe_model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
#[inline]
pub fn read_net_from_caffe_str(buffer_proto: &str, len_proto: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_proto);
	extern_container_arg!(buffer_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(buffer_proto.opencv_as_extern(), len_proto, buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
#[inline]
pub fn read_net_from_caffe_buffer(buffer_proto: &core::Vector<u8>, buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_proto.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * darknet_model: String()
#[inline]
pub fn read_net_from_darknet(cfg_file: &str, darknet_model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(cfg_file);
	extern_container_arg!(darknet_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_StringR_const_StringR(cfg_file.opencv_as_extern(), darknet_model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
#[inline]
pub fn read_net_from_darknet_str(buffer_cfg: &str, len_cfg: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_cfg);
	extern_container_arg!(buffer_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(buffer_cfg.opencv_as_extern(), len_cfg, buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
#[inline]
pub fn read_net_from_darknet_buffer(buffer_cfg: &core::Vector<u8>, buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_cfg.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(xml);
	extern_container_arg!(bin);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(xml.opencv_as_extern(), bin.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_model_optimizer_1(buffer_model_config: &core::Vector<u8>, buffer_weights: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_onnx(onnx_file: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(onnx_file);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_StringR(onnx_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_onnx_str(buffer: &str, size_buffer: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_charX_size_t(buffer.opencv_as_extern(), size_buffer, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_net_from_onnx_buffer(buffer: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_vector_unsigned_char_R(buffer.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * config: String()
#[inline]
pub fn read_net_from_tensorflow(model: &str, config: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	extern_container_arg!(config);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_config: NULL
/// * len_config: 0
#[inline]
pub fn read_net_from_tensorflow_str(buffer_model: &str, len_model: size_t, buffer_config: &str, len_config: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_model);
	extern_container_arg!(buffer_config);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(buffer_model.opencv_as_extern(), len_model, buffer_config.opencv_as_extern(), len_config, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
#[inline]
pub fn read_net_from_tensorflow_buffer(buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * is_binary: true
/// * evaluate: true
#[inline]
pub fn read_net_from_torch(model: &str, is_binary: bool, evaluate: bool) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTorch_const_StringR_bool_bool(model.opencv_as_extern(), is_binary, evaluate, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * config: ""
/// * framework: ""
#[inline]
pub fn read_net(model: &str, config: &str, framework: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	extern_container_arg!(config);
	extern_container_arg!(framework);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), framework.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
#[inline]
pub fn read_net_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	extern_container_arg!(framework);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR_const_vector_unsigned_char_R_const_vector_unsigned_char_R(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn read_tensor_from_onnx(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readTensorFromONNX_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * is_binary: true
#[inline]
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<core::Mat> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringR_bool(filename.opencv_as_extern(), is_binary, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn release_hddl_plugin() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_releaseHDDLPlugin(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn reset_myriad_device() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_resetMyriadDevice(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn set_inference_engine_backend_type(new_backend_type: &str) -> Result<String> {
	extern_container_arg!(new_backend_type);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_setInferenceEngineBackendType_const_StringR(new_backend_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn shape_1(mat: &core::Mat) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn shape_2(sz: &core::MatSize) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_MatSizeR(sz.as_raw_MatSize(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn shape_3(mat: &core::UMat) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_UMatR(mat.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn shape(dims: &i32, n: i32) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_intX_const_int(dims, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * a1: -1
/// * a2: -1
/// * a3: -1
#[inline]
pub fn shape_4(a0: i32, a1: i32, a2: i32, a3: i32) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_int_int_int_int(a0, a1, a2, a3, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * layers_types: std::vector<String>()
#[inline]
pub fn shrink_caffe_model(src: &str, dst: &str, layers_types: &core::Vector<String>) -> Result<()> {
	extern_container_arg!(src);
	extern_container_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vector_String_R(src.opencv_as_extern(), dst.opencv_as_extern(), layers_types.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn slice(m: &core::Mat, r0: &crate::dnn::_Range) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn slice_1(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn slice_2(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

#[inline]
pub fn slice_3(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range, r3: &crate::dnn::_Range) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), r3.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * top_k: 0
/// * sigma: 0.5
/// * method: SoftNMSMethod::SOFTNMS_GAUSSIAN
#[inline]
pub fn soft_nms_boxes(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, updated_scores: &mut core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, top_k: size_t, sigma: f32, method: crate::dnn::SoftNMSMethod) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_softNMSBoxes_const_vector_Rect_R_const_vector_float_R_vector_float_R_const_float_const_float_vector_int_R_size_t_const_float_SoftNMSMethod(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), updated_scores.as_raw_mut_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), top_k, sigma, method, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * start: -1
/// * end: -1
#[inline]
pub fn total(shape: &crate::dnn::MatShape, start: i32, end: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_total_const_MatShapeR_int_int(shape.as_raw_VectorOfi32(), start, end, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

#[inline]
pub fn write_text_graph(model: &str, output: &str) -> Result<()> {
	extern_container_arg!(model);
	extern_container_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_writeTextGraph_const_StringR_const_StringR(model.opencv_as_extern(), output.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait AbsLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AbsLayer(&self) -> *const c_void;

}

pub trait AbsLayerTrait: crate::dnn::AbsLayerTraitConst + crate::dnn::ActivationLayerTrait {
	fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void;

}

pub struct AbsLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AbsLayer }

impl Drop for AbsLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AbsLayer_delete(instance: *mut c_void); }
		unsafe { cv_AbsLayer_delete(self.as_raw_mut_AbsLayer()) };
	}
}

unsafe impl Send for AbsLayer {}

impl crate::dnn::ActivationLayerTraitConst for AbsLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AbsLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AbsLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AbsLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AbsLayerTraitConst for AbsLayer {
	#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AbsLayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AbsLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AbsLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AbsLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AbsLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AbsLayer, crate::dnn::ActivationLayer, cv_AbsLayer_to_ActivationLayer }

boxed_cast_base! { AbsLayer, core::Algorithm, cv_AbsLayer_to_Algorithm }

boxed_cast_base! { AbsLayer, crate::dnn::Layer, cv_AbsLayer_to_Layer }

pub trait AccumLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_AccumLayer(&self) -> *const c_void;

}

pub trait AccumLayerTrait: crate::dnn::AccumLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void;

}

pub struct AccumLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AccumLayer }

impl Drop for AccumLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AccumLayer_delete(instance: *mut c_void); }
		unsafe { cv_AccumLayer_delete(self.as_raw_mut_AccumLayer()) };
	}
}

unsafe impl Send for AccumLayer {}

impl core::AlgorithmTraitConst for AccumLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AccumLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AccumLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AccumLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AccumLayerTraitConst for AccumLayer {
	#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AccumLayerTrait for AccumLayer {
	#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AccumLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AccumLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AccumLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AccumLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AccumLayer, core::Algorithm, cv_AccumLayer_to_Algorithm }

boxed_cast_base! { AccumLayer, crate::dnn::Layer, cv_AccumLayer_to_Layer }

pub trait AcosLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AcosLayer(&self) -> *const c_void;

}

pub trait AcosLayerTrait: crate::dnn::AcosLayerTraitConst + crate::dnn::ActivationLayerTrait {
	fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void;

}

pub struct AcosLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AcosLayer }

impl Drop for AcosLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AcosLayer_delete(instance: *mut c_void); }
		unsafe { cv_AcosLayer_delete(self.as_raw_mut_AcosLayer()) };
	}
}

unsafe impl Send for AcosLayer {}

impl crate::dnn::ActivationLayerTraitConst for AcosLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AcosLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AcosLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AcosLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AcosLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AcosLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AcosLayerTraitConst for AcosLayer {
	#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AcosLayerTrait for AcosLayer {
	#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AcosLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AcosLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AcosLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AcosLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AcosLayer, crate::dnn::ActivationLayer, cv_AcosLayer_to_ActivationLayer }

boxed_cast_base! { AcosLayer, core::Algorithm, cv_AcosLayer_to_Algorithm }

boxed_cast_base! { AcosLayer, crate::dnn::Layer, cv_AcosLayer_to_Layer }

pub trait AcoshLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AcoshLayer(&self) -> *const c_void;

}

pub trait AcoshLayerTrait: crate::dnn::AcoshLayerTraitConst + crate::dnn::ActivationLayerTrait {
	fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void;

}

pub struct AcoshLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AcoshLayer }

impl Drop for AcoshLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AcoshLayer_delete(instance: *mut c_void); }
		unsafe { cv_AcoshLayer_delete(self.as_raw_mut_AcoshLayer()) };
	}
}

unsafe impl Send for AcoshLayer {}

impl crate::dnn::ActivationLayerTraitConst for AcoshLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AcoshLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AcoshLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AcoshLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AcoshLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AcoshLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AcoshLayerTraitConst for AcoshLayer {
	#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AcoshLayerTrait for AcoshLayer {
	#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AcoshLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AcoshLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AcoshLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AcoshLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AcoshLayer, crate::dnn::ActivationLayer, cv_AcoshLayer_to_ActivationLayer }

boxed_cast_base! { AcoshLayer, core::Algorithm, cv_AcoshLayer_to_Algorithm }

boxed_cast_base! { AcoshLayer, crate::dnn::Layer, cv_AcoshLayer_to_Layer }

pub trait ActivationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ActivationLayer(&self) -> *const c_void;

	#[inline]
	fn forward_slice(&self, src: &f32, dst: &mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(self.as_raw_ActivationLayer(), src, dst, len, out_plane_size, cn0, cn1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward_slice_1(&self, src: &i32, lut: &i32, dst: &mut i32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_intX_const_intX_intX_int_size_t_int_int(self.as_raw_ActivationLayer(), src, lut, dst, len, out_plane_size, cn0, cn1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ActivationLayerTrait: crate::dnn::ActivationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void;

}

pub struct ActivationLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ActivationLayer }

impl Drop for ActivationLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ActivationLayer_delete(instance: *mut c_void); }
		unsafe { cv_ActivationLayer_delete(self.as_raw_mut_ActivationLayer()) };
	}
}

unsafe impl Send for ActivationLayer {}

impl core::AlgorithmTraitConst for ActivationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ActivationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ActivationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ActivationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for ActivationLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ActivationLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ActivationLayer {
}

boxed_cast_descendant! { ActivationLayer, crate::dnn::AbsLayer, cv_ActivationLayer_to_AbsLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AcosLayer, cv_ActivationLayer_to_AcosLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AcoshLayer, cv_ActivationLayer_to_AcoshLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ActivationLayerInt8, cv_ActivationLayer_to_ActivationLayerInt8 }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AsinLayer, cv_ActivationLayer_to_AsinLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AsinhLayer, cv_ActivationLayer_to_AsinhLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AtanLayer, cv_ActivationLayer_to_AtanLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::AtanhLayer, cv_ActivationLayer_to_AtanhLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::BNLLLayer, cv_ActivationLayer_to_BNLLLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::BatchNormLayer, cv_ActivationLayer_to_BatchNormLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::CeilLayer, cv_ActivationLayer_to_CeilLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::CeluLayer, cv_ActivationLayer_to_CeluLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ChannelsPReLULayer, cv_ActivationLayer_to_ChannelsPReLULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::CosLayer, cv_ActivationLayer_to_CosLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::CoshLayer, cv_ActivationLayer_to_CoshLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ELULayer, cv_ActivationLayer_to_ELULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ErfLayer, cv_ActivationLayer_to_ErfLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ExpLayer, cv_ActivationLayer_to_ExpLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::FloorLayer, cv_ActivationLayer_to_FloorLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::HardSigmoidLayer, cv_ActivationLayer_to_HardSigmoidLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::HardSwishLayer, cv_ActivationLayer_to_HardSwishLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::LogLayer, cv_ActivationLayer_to_LogLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::MishLayer, cv_ActivationLayer_to_MishLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::NotLayer, cv_ActivationLayer_to_NotLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::PowerLayer, cv_ActivationLayer_to_PowerLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLU6Layer, cv_ActivationLayer_to_ReLU6Layer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLULayer, cv_ActivationLayer_to_ReLULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ReciprocalLayer, cv_ActivationLayer_to_ReciprocalLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::RoundLayer, cv_ActivationLayer_to_RoundLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SeluLayer, cv_ActivationLayer_to_SeluLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ShrinkLayer, cv_ActivationLayer_to_ShrinkLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SigmoidLayer, cv_ActivationLayer_to_SigmoidLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SignLayer, cv_ActivationLayer_to_SignLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SinLayer, cv_ActivationLayer_to_SinLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SinhLayer, cv_ActivationLayer_to_SinhLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SoftplusLayer, cv_ActivationLayer_to_SoftplusLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SoftsignLayer, cv_ActivationLayer_to_SoftsignLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SqrtLayer, cv_ActivationLayer_to_SqrtLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SwishLayer, cv_ActivationLayer_to_SwishLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::TanHLayer, cv_ActivationLayer_to_TanHLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::TanLayer, cv_ActivationLayer_to_TanLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ThresholdedReluLayer, cv_ActivationLayer_to_ThresholdedReluLayer }

boxed_cast_base! { ActivationLayer, core::Algorithm, cv_ActivationLayer_to_Algorithm }

boxed_cast_base! { ActivationLayer, crate::dnn::Layer, cv_ActivationLayer_to_Layer }

pub trait ActivationLayerInt8TraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ActivationLayerInt8(&self) -> *const c_void;

}

pub trait ActivationLayerInt8Trait: crate::dnn::ActivationLayerInt8TraitConst + crate::dnn::ActivationLayerTrait {
	fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void;

}

pub struct ActivationLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { ActivationLayerInt8 }

impl Drop for ActivationLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_ActivationLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_ActivationLayerInt8_delete(self.as_raw_mut_ActivationLayerInt8()) };
	}
}

unsafe impl Send for ActivationLayerInt8 {}

impl crate::dnn::ActivationLayerTraitConst for ActivationLayerInt8 {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ActivationLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ActivationLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ActivationLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ActivationLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ActivationLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ActivationLayerInt8TraitConst for ActivationLayerInt8 {
	#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerInt8Trait for ActivationLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ActivationLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ActivationLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ActivationLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ActivationLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ActivationLayerInt8, crate::dnn::ActivationLayer, cv_ActivationLayerInt8_to_ActivationLayer }

boxed_cast_base! { ActivationLayerInt8, core::Algorithm, cv_ActivationLayerInt8_to_Algorithm }

boxed_cast_base! { ActivationLayerInt8, crate::dnn::Layer, cv_ActivationLayerInt8_to_Layer }

pub trait ArgLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ArgLayer(&self) -> *const c_void;

}

pub trait ArgLayerTrait: crate::dnn::ArgLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void;

}

pub struct ArgLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ArgLayer }

impl Drop for ArgLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ArgLayer_delete(instance: *mut c_void); }
		unsafe { cv_ArgLayer_delete(self.as_raw_mut_ArgLayer()) };
	}
}

unsafe impl Send for ArgLayer {}

impl core::AlgorithmTraitConst for ArgLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ArgLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ArgLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ArgLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ArgLayerTraitConst for ArgLayer {
	#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ArgLayerTrait for ArgLayer {
	#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ArgLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ArgLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ArgLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ArgLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ArgLayer, core::Algorithm, cv_ArgLayer_to_Algorithm }

boxed_cast_base! { ArgLayer, crate::dnn::Layer, cv_ArgLayer_to_Layer }

pub trait AsinLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AsinLayer(&self) -> *const c_void;

}

pub trait AsinLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AsinLayerTraitConst {
	fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void;

}

pub struct AsinLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AsinLayer }

impl Drop for AsinLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AsinLayer_delete(instance: *mut c_void); }
		unsafe { cv_AsinLayer_delete(self.as_raw_mut_AsinLayer()) };
	}
}

unsafe impl Send for AsinLayer {}

impl crate::dnn::ActivationLayerTraitConst for AsinLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AsinLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AsinLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AsinLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AsinLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AsinLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AsinLayerTraitConst for AsinLayer {
	#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AsinLayerTrait for AsinLayer {
	#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsinLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AsinLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AsinLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AsinLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AsinLayer, crate::dnn::ActivationLayer, cv_AsinLayer_to_ActivationLayer }

boxed_cast_base! { AsinLayer, core::Algorithm, cv_AsinLayer_to_Algorithm }

boxed_cast_base! { AsinLayer, crate::dnn::Layer, cv_AsinLayer_to_Layer }

pub trait AsinhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AsinhLayer(&self) -> *const c_void;

}

pub trait AsinhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AsinhLayerTraitConst {
	fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void;

}

pub struct AsinhLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AsinhLayer }

impl Drop for AsinhLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AsinhLayer_delete(instance: *mut c_void); }
		unsafe { cv_AsinhLayer_delete(self.as_raw_mut_AsinhLayer()) };
	}
}

unsafe impl Send for AsinhLayer {}

impl crate::dnn::ActivationLayerTraitConst for AsinhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AsinhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AsinhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AsinhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AsinhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AsinhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AsinhLayerTraitConst for AsinhLayer {
	#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AsinhLayerTrait for AsinhLayer {
	#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AsinhLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AsinhLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AsinhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AsinhLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AsinhLayer, crate::dnn::ActivationLayer, cv_AsinhLayer_to_ActivationLayer }

boxed_cast_base! { AsinhLayer, core::Algorithm, cv_AsinhLayer_to_Algorithm }

boxed_cast_base! { AsinhLayer, crate::dnn::Layer, cv_AsinhLayer_to_Layer }

pub trait AtanLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AtanLayer(&self) -> *const c_void;

}

pub trait AtanLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AtanLayerTraitConst {
	fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void;

}

pub struct AtanLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AtanLayer }

impl Drop for AtanLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AtanLayer_delete(instance: *mut c_void); }
		unsafe { cv_AtanLayer_delete(self.as_raw_mut_AtanLayer()) };
	}
}

unsafe impl Send for AtanLayer {}

impl crate::dnn::ActivationLayerTraitConst for AtanLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AtanLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AtanLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AtanLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AtanLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AtanLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AtanLayerTraitConst for AtanLayer {
	#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AtanLayerTrait for AtanLayer {
	#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AtanLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AtanLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AtanLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AtanLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AtanLayer, crate::dnn::ActivationLayer, cv_AtanLayer_to_ActivationLayer }

boxed_cast_base! { AtanLayer, core::Algorithm, cv_AtanLayer_to_Algorithm }

boxed_cast_base! { AtanLayer, crate::dnn::Layer, cv_AtanLayer_to_Layer }

pub trait AtanhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AtanhLayer(&self) -> *const c_void;

}

pub trait AtanhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AtanhLayerTraitConst {
	fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void;

}

pub struct AtanhLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { AtanhLayer }

impl Drop for AtanhLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_AtanhLayer_delete(instance: *mut c_void); }
		unsafe { cv_AtanhLayer_delete(self.as_raw_mut_AtanhLayer()) };
	}
}

unsafe impl Send for AtanhLayer {}

impl crate::dnn::ActivationLayerTraitConst for AtanhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AtanhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for AtanhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AtanhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for AtanhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AtanhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AtanhLayerTraitConst for AtanhLayer {
	#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AtanhLayerTrait for AtanhLayer {
	#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AtanhLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AtanhLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AtanhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AtanhLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AtanhLayer, crate::dnn::ActivationLayer, cv_AtanhLayer_to_ActivationLayer }

boxed_cast_base! { AtanhLayer, core::Algorithm, cv_AtanhLayer_to_Algorithm }

boxed_cast_base! { AtanhLayer, crate::dnn::Layer, cv_AtanhLayer_to_Layer }

pub trait BNLLLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_BNLLLayer(&self) -> *const c_void;

}

pub trait BNLLLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BNLLLayerTraitConst {
	fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void;

}

pub struct BNLLLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { BNLLLayer }

impl Drop for BNLLLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BNLLLayer_delete(instance: *mut c_void); }
		unsafe { cv_BNLLLayer_delete(self.as_raw_mut_BNLLLayer()) };
	}
}

unsafe impl Send for BNLLLayer {}

impl crate::dnn::ActivationLayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for BNLLLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BNLLLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BNLLLayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BNLLLayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BNLLLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BNLLLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BNLLLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BNLLLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BNLLLayer, crate::dnn::ActivationLayer, cv_BNLLLayer_to_ActivationLayer }

boxed_cast_base! { BNLLLayer, core::Algorithm, cv_BNLLLayer_to_Algorithm }

boxed_cast_base! { BNLLLayer, crate::dnn::Layer, cv_BNLLLayer_to_Layer }

pub trait BackendNodeTraitConst {
	fn as_raw_BackendNode(&self) -> *const c_void;

	#[inline]
	fn backend_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendNode_getPropBackendId_const(self.as_raw_BackendNode()) };
		ret
	}
	
}

pub trait BackendNodeTrait: crate::dnn::BackendNodeTraitConst {
	fn as_raw_mut_BackendNode(&mut self) -> *mut c_void;

	#[inline]
	fn set_backend_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendNode_setPropBackendId_int(self.as_raw_mut_BackendNode(), val) };
		ret
	}
	
}

pub struct BackendNode {
	ptr: *mut c_void
}

opencv_type_boxed! { BackendNode }

impl Drop for BackendNode {
	fn drop(&mut self) {
		extern "C" { fn cv_BackendNode_delete(instance: *mut c_void); }
		unsafe { cv_BackendNode_delete(self.as_raw_mut_BackendNode()) };
	}
}

unsafe impl Send for BackendNode {}

impl crate::dnn::BackendNodeTraitConst for BackendNode {
	#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BackendNodeTrait for BackendNode {
	#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BackendNode {
}

pub trait BackendWrapperConst {
	fn as_raw_BackendWrapper(&self) -> *const c_void;

	#[inline]
	fn backend_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_getPropBackendId_const(self.as_raw_BackendWrapper()) };
		ret
	}
	
	#[inline]
	fn target_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_getPropTargetId_const(self.as_raw_BackendWrapper()) };
		ret
	}
	
}

pub trait BackendWrapper: crate::dnn::BackendWrapperConst {
	fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void;

	#[inline]
	fn set_backend_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_setPropBackendId_int(self.as_raw_mut_BackendWrapper(), val) };
		ret
	}
	
	#[inline]
	fn set_target_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_setPropTargetId_int(self.as_raw_mut_BackendWrapper(), val) };
		ret
	}
	
	#[inline]
	fn copy_to_host(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BackendWrapper_copyToHost(self.as_raw_mut_BackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_host_dirty(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BackendWrapper_setHostDirty(self.as_raw_mut_BackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BaseConvolutionLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_BaseConvolutionLayer(&self) -> *const c_void;

	#[inline]
	fn kernel(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropKernel_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn stride(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropStride_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn dilation(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropDilation_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn adjust_pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropAdjustPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn adjust_pads(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropAdjust_pads_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn kernel_size(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropKernel_size_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn strides(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropStrides_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn dilations(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropDilations_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pads_begin(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPads_begin_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pads_end(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPads_end_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pad_mode(&self) -> String {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropPadMode_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn num_output(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_getPropNumOutput_const(self.as_raw_BaseConvolutionLayer()) };
		ret
	}
	
}

pub trait BaseConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_kernel(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropKernel_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_stride(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropStride_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPad_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_dilation(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropDilation_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_adjust_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropAdjustPad_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_adjust_pads(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropAdjust_pads_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_kernel_size(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropKernel_size_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_strides(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropStrides_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_dilations(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropDilations_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_pads_begin(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPads_begin_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_pads_end(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPads_end_vector_size_t_(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_pad_mode(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropPadMode_String(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_num_output(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_setPropNumOutput_int(self.as_raw_mut_BaseConvolutionLayer(), val) };
		ret
	}
	
}

pub struct BaseConvolutionLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { BaseConvolutionLayer }

impl Drop for BaseConvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BaseConvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_BaseConvolutionLayer_delete(self.as_raw_mut_BaseConvolutionLayer()) };
	}
}

unsafe impl Send for BaseConvolutionLayer {}

impl core::AlgorithmTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BaseConvolutionLayerTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BaseConvolutionLayer {
}

boxed_cast_base! { BaseConvolutionLayer, core::Algorithm, cv_BaseConvolutionLayer_to_Algorithm }

boxed_cast_base! { BaseConvolutionLayer, crate::dnn::Layer, cv_BaseConvolutionLayer_to_Layer }

pub trait BatchNormLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_BatchNormLayer(&self) -> *const c_void;

	#[inline]
	fn has_weights(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_getPropHasWeights_const(self.as_raw_BatchNormLayer()) };
		ret
	}
	
	#[inline]
	fn has_bias(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_getPropHasBias_const(self.as_raw_BatchNormLayer()) };
		ret
	}
	
	#[inline]
	fn epsilon(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_getPropEpsilon_const(self.as_raw_BatchNormLayer()) };
		ret
	}
	
}

pub trait BatchNormLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BatchNormLayerTraitConst {
	fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_has_weights(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_setPropHasWeights_bool(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_has_bias(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_setPropHasBias_bool(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_epsilon(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_setPropEpsilon_float(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}
	
}

pub struct BatchNormLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { BatchNormLayer }

impl Drop for BatchNormLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BatchNormLayer_delete(instance: *mut c_void); }
		unsafe { cv_BatchNormLayer_delete(self.as_raw_mut_BatchNormLayer()) };
	}
}

unsafe impl Send for BatchNormLayer {}

impl crate::dnn::ActivationLayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for BatchNormLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BatchNormLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BatchNormLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BatchNormLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BatchNormLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BatchNormLayer, crate::dnn::ActivationLayer, cv_BatchNormLayer_to_ActivationLayer }

boxed_cast_base! { BatchNormLayer, core::Algorithm, cv_BatchNormLayer_to_Algorithm }

boxed_cast_base! { BatchNormLayer, crate::dnn::Layer, cv_BatchNormLayer_to_Layer }

pub trait BatchNormLayerInt8TraitConst: crate::dnn::BatchNormLayerTraitConst {
	fn as_raw_BatchNormLayerInt8(&self) -> *const c_void;

	#[inline]
	fn input_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_getPropInput_sc_const(self.as_raw_BatchNormLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_getPropOutput_sc_const(self.as_raw_BatchNormLayerInt8()) };
		ret
	}
	
	#[inline]
	fn input_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_getPropInput_zp_const(self.as_raw_BatchNormLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_getPropOutput_zp_const(self.as_raw_BatchNormLayerInt8()) };
		ret
	}
	
}

pub trait BatchNormLayerInt8Trait: crate::dnn::BatchNormLayerInt8TraitConst + crate::dnn::BatchNormLayerTrait {
	fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_input_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_setPropInput_sc_float(self.as_raw_mut_BatchNormLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_setPropOutput_sc_float(self.as_raw_mut_BatchNormLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_input_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_setPropInput_zp_int(self.as_raw_mut_BatchNormLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_setPropOutput_zp_int(self.as_raw_mut_BatchNormLayerInt8(), val) };
		ret
	}
	
}

pub struct BatchNormLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { BatchNormLayerInt8 }

impl Drop for BatchNormLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_BatchNormLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_BatchNormLayerInt8_delete(self.as_raw_mut_BatchNormLayerInt8()) };
	}
}

unsafe impl Send for BatchNormLayerInt8 {}

impl crate::dnn::ActivationLayerTraitConst for BatchNormLayerInt8 {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for BatchNormLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for BatchNormLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BatchNormLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerTraitConst for BatchNormLayerInt8 {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for BatchNormLayerInt8 {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for BatchNormLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BatchNormLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BatchNormLayerInt8TraitConst for BatchNormLayerInt8 {
	#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BatchNormLayerInt8Trait for BatchNormLayerInt8 {
	#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BatchNormLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BatchNormLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BatchNormLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BatchNormLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BatchNormLayerInt8, crate::dnn::ActivationLayer, cv_BatchNormLayerInt8_to_ActivationLayer }

boxed_cast_base! { BatchNormLayerInt8, core::Algorithm, cv_BatchNormLayerInt8_to_Algorithm }

boxed_cast_base! { BatchNormLayerInt8, crate::dnn::BatchNormLayer, cv_BatchNormLayerInt8_to_BatchNormLayer }

boxed_cast_base! { BatchNormLayerInt8, crate::dnn::Layer, cv_BatchNormLayerInt8_to_Layer }

pub trait BlankLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_BlankLayer(&self) -> *const c_void;

}

pub trait BlankLayerTrait: crate::dnn::BlankLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void;

}

pub struct BlankLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { BlankLayer }

impl Drop for BlankLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BlankLayer_delete(instance: *mut c_void); }
		unsafe { cv_BlankLayer_delete(self.as_raw_mut_BlankLayer()) };
	}
}

unsafe impl Send for BlankLayer {}

impl core::AlgorithmTraitConst for BlankLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BlankLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for BlankLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BlankLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BlankLayerTraitConst for BlankLayer {
	#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BlankLayerTrait for BlankLayer {
	#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BlankLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BlankLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BlankLayer, core::Algorithm, cv_BlankLayer_to_Algorithm }

boxed_cast_base! { BlankLayer, crate::dnn::Layer, cv_BlankLayer_to_Layer }

pub trait CeilLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_CeilLayer(&self) -> *const c_void;

}

pub trait CeilLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CeilLayerTraitConst {
	fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void;

}

pub struct CeilLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CeilLayer }

impl Drop for CeilLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CeilLayer_delete(instance: *mut c_void); }
		unsafe { cv_CeilLayer_delete(self.as_raw_mut_CeilLayer()) };
	}
}

unsafe impl Send for CeilLayer {}

impl crate::dnn::ActivationLayerTraitConst for CeilLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for CeilLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for CeilLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CeilLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CeilLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CeilLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CeilLayerTraitConst for CeilLayer {
	#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CeilLayerTrait for CeilLayer {
	#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CeilLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CeilLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CeilLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CeilLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CeilLayer, crate::dnn::ActivationLayer, cv_CeilLayer_to_ActivationLayer }

boxed_cast_base! { CeilLayer, core::Algorithm, cv_CeilLayer_to_Algorithm }

boxed_cast_base! { CeilLayer, crate::dnn::Layer, cv_CeilLayer_to_Layer }

pub trait CeluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_CeluLayer(&self) -> *const c_void;

	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_CeluLayer_getPropAlpha_const(self.as_raw_CeluLayer()) };
		ret
	}
	
}

pub trait CeluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CeluLayerTraitConst {
	fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_CeluLayer_setPropAlpha_float(self.as_raw_mut_CeluLayer(), val) };
		ret
	}
	
}

pub struct CeluLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CeluLayer }

impl Drop for CeluLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CeluLayer_delete(instance: *mut c_void); }
		unsafe { cv_CeluLayer_delete(self.as_raw_mut_CeluLayer()) };
	}
}

unsafe impl Send for CeluLayer {}

impl crate::dnn::ActivationLayerTraitConst for CeluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for CeluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for CeluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CeluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CeluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CeluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CeluLayerTraitConst for CeluLayer {
	#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CeluLayerTrait for CeluLayer {
	#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CeluLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CeluLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CeluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CeluLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CeluLayer, crate::dnn::ActivationLayer, cv_CeluLayer_to_ActivationLayer }

boxed_cast_base! { CeluLayer, core::Algorithm, cv_CeluLayer_to_Algorithm }

boxed_cast_base! { CeluLayer, crate::dnn::Layer, cv_CeluLayer_to_Layer }

pub trait ChannelsPReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ChannelsPReLULayer(&self) -> *const c_void;

}

pub trait ChannelsPReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ChannelsPReLULayerTraitConst {
	fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void;

}

pub struct ChannelsPReLULayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ChannelsPReLULayer }

impl Drop for ChannelsPReLULayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ChannelsPReLULayer_delete(instance: *mut c_void); }
		unsafe { cv_ChannelsPReLULayer_delete(self.as_raw_mut_ChannelsPReLULayer()) };
	}
}

unsafe impl Send for ChannelsPReLULayer {}

impl crate::dnn::ActivationLayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ChannelsPReLULayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_ChannelsPReLULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ChannelsPReLULayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ChannelsPReLULayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ChannelsPReLULayer, crate::dnn::ActivationLayer, cv_ChannelsPReLULayer_to_ActivationLayer }

boxed_cast_base! { ChannelsPReLULayer, core::Algorithm, cv_ChannelsPReLULayer_to_Algorithm }

boxed_cast_base! { ChannelsPReLULayer, crate::dnn::Layer, cv_ChannelsPReLULayer_to_Layer }

pub trait ClassificationModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_ClassificationModel(&self) -> *const c_void;

	#[inline]
	fn get_enable_softmax_post_processing(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_getEnableSoftmaxPostProcessing_const(self.as_raw_ClassificationModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ClassificationModelTrait: crate::dnn::ClassificationModelTraitConst + crate::dnn::ModelTrait {
	fn as_raw_mut_ClassificationModel(&mut self) -> *mut c_void;

	#[inline]
	fn set_enable_softmax_post_processing(&mut self, enable: bool) -> Result<crate::dnn::ClassificationModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_setEnableSoftmaxPostProcessing_bool(self.as_raw_mut_ClassificationModel(), enable, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn classify(&mut self, frame: &dyn core::ToInputArray, class_id: &mut i32, conf: &mut f32) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_classify_const__InputArrayR_intR_floatR(self.as_raw_mut_ClassificationModel(), frame.as_raw__InputArray(), class_id, conf, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct ClassificationModel {
	ptr: *mut c_void
}

opencv_type_boxed! { ClassificationModel }

impl Drop for ClassificationModel {
	fn drop(&mut self) {
		extern "C" { fn cv_ClassificationModel_delete(instance: *mut c_void); }
		unsafe { cv_ClassificationModel_delete(self.as_raw_mut_ClassificationModel()) };
	}
}

unsafe impl Send for ClassificationModel {}

impl crate::dnn::ModelTraitConst for ClassificationModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for ClassificationModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ClassificationModelTraitConst for ClassificationModel {
	#[inline] fn as_raw_ClassificationModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ClassificationModelTrait for ClassificationModel {
	#[inline] fn as_raw_mut_ClassificationModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ClassificationModel {
	#[inline]
	pub fn default() -> Result<crate::dnn::ClassificationModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new(model: &str, config: &str) -> Result<crate::dnn::ClassificationModel> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::ClassificationModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ClassificationModel, crate::dnn::Model, cv_ClassificationModel_to_Model }

pub trait CompareLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CompareLayer(&self) -> *const c_void;

}

pub trait CompareLayerTrait: crate::dnn::CompareLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CompareLayer(&mut self) -> *mut c_void;

}

pub struct CompareLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CompareLayer }

impl Drop for CompareLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CompareLayer_delete(instance: *mut c_void); }
		unsafe { cv_CompareLayer_delete(self.as_raw_mut_CompareLayer()) };
	}
}

unsafe impl Send for CompareLayer {}

impl core::AlgorithmTraitConst for CompareLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CompareLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CompareLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CompareLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CompareLayerTraitConst for CompareLayer {
	#[inline] fn as_raw_CompareLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CompareLayerTrait for CompareLayer {
	#[inline] fn as_raw_mut_CompareLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CompareLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CompareLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CompareLayer, core::Algorithm, cv_CompareLayer_to_Algorithm }

boxed_cast_base! { CompareLayer, crate::dnn::Layer, cv_CompareLayer_to_Layer }

pub trait ConcatLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ConcatLayer(&self) -> *const c_void;

	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_getPropAxis_const(self.as_raw_ConcatLayer()) };
		ret
	}
	
	#[inline]
	fn padding(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_getPropPadding_const(self.as_raw_ConcatLayer()) };
		ret
	}
	
	#[inline]
	fn padding_value(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_getPropPaddingValue_const(self.as_raw_ConcatLayer()) };
		ret
	}
	
}

pub trait ConcatLayerTrait: crate::dnn::ConcatLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_setPropAxis_int(self.as_raw_mut_ConcatLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_padding(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_setPropPadding_bool(self.as_raw_mut_ConcatLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_padding_value(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_setPropPaddingValue_int(self.as_raw_mut_ConcatLayer(), val) };
		ret
	}
	
}

pub struct ConcatLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ConcatLayer }

impl Drop for ConcatLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConcatLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConcatLayer_delete(self.as_raw_mut_ConcatLayer()) };
	}
}

unsafe impl Send for ConcatLayer {}

impl core::AlgorithmTraitConst for ConcatLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConcatLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ConcatLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConcatLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ConcatLayerTraitConst for ConcatLayer {
	#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConcatLayerTrait for ConcatLayer {
	#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ConcatLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ConcatLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConcatLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ConcatLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ConcatLayer, core::Algorithm, cv_ConcatLayer_to_Algorithm }

boxed_cast_base! { ConcatLayer, crate::dnn::Layer, cv_ConcatLayer_to_Layer }

pub trait ConstLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ConstLayer(&self) -> *const c_void;

}

pub trait ConstLayerTrait: crate::dnn::ConstLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void;

}

pub struct ConstLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ConstLayer }

impl Drop for ConstLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConstLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConstLayer_delete(self.as_raw_mut_ConstLayer()) };
	}
}

unsafe impl Send for ConstLayer {}

impl core::AlgorithmTraitConst for ConstLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConstLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ConstLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConstLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ConstLayerTraitConst for ConstLayer {
	#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConstLayerTrait for ConstLayer {
	#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ConstLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConstLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ConstLayer, core::Algorithm, cv_ConstLayer_to_Algorithm }

boxed_cast_base! { ConstLayer, crate::dnn::Layer, cv_ConstLayer_to_Layer }

pub trait ConvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
	fn as_raw_ConvolutionLayer(&self) -> *const c_void;

}

pub trait ConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::ConvolutionLayerTraitConst {
	fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void;

}

pub struct ConvolutionLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ConvolutionLayer }

impl Drop for ConvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConvolutionLayer_delete(self.as_raw_mut_ConvolutionLayer()) };
	}
}

unsafe impl Send for ConvolutionLayer {}

impl core::AlgorithmTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BaseConvolutionLayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ConvolutionLayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConvolutionLayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ConvolutionLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConvolutionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ConvolutionLayer, core::Algorithm, cv_ConvolutionLayer_to_Algorithm }

boxed_cast_base! { ConvolutionLayer, crate::dnn::BaseConvolutionLayer, cv_ConvolutionLayer_to_BaseConvolutionLayer }

boxed_cast_base! { ConvolutionLayer, crate::dnn::Layer, cv_ConvolutionLayer_to_Layer }

pub trait ConvolutionLayerInt8TraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
	fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void;

	#[inline]
	fn input_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_getPropInput_zp_const(self.as_raw_ConvolutionLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_getPropOutput_zp_const(self.as_raw_ConvolutionLayerInt8()) };
		ret
	}
	
	#[inline]
	fn input_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_getPropInput_sc_const(self.as_raw_ConvolutionLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_getPropOutput_sc_const(self.as_raw_ConvolutionLayerInt8()) };
		ret
	}
	
}

pub trait ConvolutionLayerInt8Trait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::ConvolutionLayerInt8TraitConst {
	fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_input_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_setPropInput_zp_int(self.as_raw_mut_ConvolutionLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_setPropOutput_zp_int(self.as_raw_mut_ConvolutionLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_input_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_setPropInput_sc_float(self.as_raw_mut_ConvolutionLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_setPropOutput_sc_float(self.as_raw_mut_ConvolutionLayerInt8(), val) };
		ret
	}
	
}

pub struct ConvolutionLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { ConvolutionLayerInt8 }

impl Drop for ConvolutionLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_ConvolutionLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_ConvolutionLayerInt8_delete(self.as_raw_mut_ConvolutionLayerInt8()) };
	}
}

unsafe impl Send for ConvolutionLayerInt8 {}

impl core::AlgorithmTraitConst for ConvolutionLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConvolutionLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BaseConvolutionLayerTraitConst for ConvolutionLayerInt8 {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayerInt8 {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ConvolutionLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConvolutionLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ConvolutionLayerInt8TraitConst for ConvolutionLayerInt8 {
	#[inline] fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConvolutionLayerInt8Trait for ConvolutionLayerInt8 {
	#[inline] fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ConvolutionLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConvolutionLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ConvolutionLayerInt8, core::Algorithm, cv_ConvolutionLayerInt8_to_Algorithm }

boxed_cast_base! { ConvolutionLayerInt8, crate::dnn::BaseConvolutionLayer, cv_ConvolutionLayerInt8_to_BaseConvolutionLayer }

boxed_cast_base! { ConvolutionLayerInt8, crate::dnn::Layer, cv_ConvolutionLayerInt8_to_Layer }

pub trait CorrelationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CorrelationLayer(&self) -> *const c_void;

}

pub trait CorrelationLayerTrait: crate::dnn::CorrelationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void;

}

pub struct CorrelationLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CorrelationLayer }

impl Drop for CorrelationLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CorrelationLayer_delete(instance: *mut c_void); }
		unsafe { cv_CorrelationLayer_delete(self.as_raw_mut_CorrelationLayer()) };
	}
}

unsafe impl Send for CorrelationLayer {}

impl core::AlgorithmTraitConst for CorrelationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CorrelationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CorrelationLayerTraitConst for CorrelationLayer {
	#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CorrelationLayerTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CorrelationLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CorrelationLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CorrelationLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CorrelationLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CorrelationLayer, core::Algorithm, cv_CorrelationLayer_to_Algorithm }

boxed_cast_base! { CorrelationLayer, crate::dnn::Layer, cv_CorrelationLayer_to_Layer }

pub trait CosLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_CosLayer(&self) -> *const c_void;

}

pub trait CosLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CosLayerTraitConst {
	fn as_raw_mut_CosLayer(&mut self) -> *mut c_void;

}

pub struct CosLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CosLayer }

impl Drop for CosLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CosLayer_delete(instance: *mut c_void); }
		unsafe { cv_CosLayer_delete(self.as_raw_mut_CosLayer()) };
	}
}

unsafe impl Send for CosLayer {}

impl crate::dnn::ActivationLayerTraitConst for CosLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for CosLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for CosLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CosLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CosLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CosLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CosLayerTraitConst for CosLayer {
	#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CosLayerTrait for CosLayer {
	#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CosLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CosLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CosLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CosLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CosLayer, crate::dnn::ActivationLayer, cv_CosLayer_to_ActivationLayer }

boxed_cast_base! { CosLayer, core::Algorithm, cv_CosLayer_to_Algorithm }

boxed_cast_base! { CosLayer, crate::dnn::Layer, cv_CosLayer_to_Layer }

pub trait CoshLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_CoshLayer(&self) -> *const c_void;

}

pub trait CoshLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CoshLayerTraitConst {
	fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void;

}

pub struct CoshLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CoshLayer }

impl Drop for CoshLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CoshLayer_delete(instance: *mut c_void); }
		unsafe { cv_CoshLayer_delete(self.as_raw_mut_CoshLayer()) };
	}
}

unsafe impl Send for CoshLayer {}

impl crate::dnn::ActivationLayerTraitConst for CoshLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for CoshLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for CoshLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CoshLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CoshLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CoshLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CoshLayerTraitConst for CoshLayer {
	#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CoshLayerTrait for CoshLayer {
	#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CoshLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CoshLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CoshLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CoshLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CoshLayer, crate::dnn::ActivationLayer, cv_CoshLayer_to_ActivationLayer }

boxed_cast_base! { CoshLayer, core::Algorithm, cv_CoshLayer_to_Algorithm }

boxed_cast_base! { CoshLayer, crate::dnn::Layer, cv_CoshLayer_to_Layer }

pub trait CropAndResizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CropAndResizeLayer(&self) -> *const c_void;

}

pub trait CropAndResizeLayerTrait: crate::dnn::CropAndResizeLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void;

}

pub struct CropAndResizeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CropAndResizeLayer }

impl Drop for CropAndResizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CropAndResizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_CropAndResizeLayer_delete(self.as_raw_mut_CropAndResizeLayer()) };
	}
}

unsafe impl Send for CropAndResizeLayer {}

impl core::AlgorithmTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CropAndResizeLayerTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CropAndResizeLayerTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CropAndResizeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CropAndResizeLayer, core::Algorithm, cv_CropAndResizeLayer_to_Algorithm }

boxed_cast_base! { CropAndResizeLayer, crate::dnn::Layer, cv_CropAndResizeLayer_to_Layer }

pub trait CropLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CropLayer(&self) -> *const c_void;

}

pub trait CropLayerTrait: crate::dnn::CropLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CropLayer(&mut self) -> *mut c_void;

}

pub struct CropLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CropLayer }

impl Drop for CropLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CropLayer_delete(instance: *mut c_void); }
		unsafe { cv_CropLayer_delete(self.as_raw_mut_CropLayer()) };
	}
}

unsafe impl Send for CropLayer {}

impl core::AlgorithmTraitConst for CropLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CropLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CropLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CropLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CropLayerTraitConst for CropLayer {
	#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CropLayerTrait for CropLayer {
	#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CropLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CropLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CropLayer, core::Algorithm, cv_CropLayer_to_Algorithm }

boxed_cast_base! { CropLayer, crate::dnn::Layer, cv_CropLayer_to_Layer }

pub trait CumSumLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CumSumLayer(&self) -> *const c_void;

	#[inline]
	fn exclusive(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_CumSumLayer_getPropExclusive_const(self.as_raw_CumSumLayer()) };
		ret
	}
	
	#[inline]
	fn reverse(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_CumSumLayer_getPropReverse_const(self.as_raw_CumSumLayer()) };
		ret
	}
	
}

pub trait CumSumLayerTrait: crate::dnn::CumSumLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_exclusive(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_CumSumLayer_setPropExclusive_int(self.as_raw_mut_CumSumLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_reverse(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_CumSumLayer_setPropReverse_int(self.as_raw_mut_CumSumLayer(), val) };
		ret
	}
	
}

pub struct CumSumLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { CumSumLayer }

impl Drop for CumSumLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CumSumLayer_delete(instance: *mut c_void); }
		unsafe { cv_CumSumLayer_delete(self.as_raw_mut_CumSumLayer()) };
	}
}

unsafe impl Send for CumSumLayer {}

impl core::AlgorithmTraitConst for CumSumLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CumSumLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for CumSumLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CumSumLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CumSumLayerTraitConst for CumSumLayer {
	#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CumSumLayerTrait for CumSumLayer {
	#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CumSumLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CumSumLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CumSumLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CumSumLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CumSumLayer, core::Algorithm, cv_CumSumLayer_to_Algorithm }

boxed_cast_base! { CumSumLayer, crate::dnn::Layer, cv_CumSumLayer_to_Layer }

pub trait DataAugmentationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_DataAugmentationLayer(&self) -> *const c_void;

}

pub trait DataAugmentationLayerTrait: crate::dnn::DataAugmentationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void;

}

pub struct DataAugmentationLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { DataAugmentationLayer }

impl Drop for DataAugmentationLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DataAugmentationLayer_delete(instance: *mut c_void); }
		unsafe { cv_DataAugmentationLayer_delete(self.as_raw_mut_DataAugmentationLayer()) };
	}
}

unsafe impl Send for DataAugmentationLayer {}

impl core::AlgorithmTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DataAugmentationLayerTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DataAugmentationLayerTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DataAugmentationLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DataAugmentationLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::DataAugmentationLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DataAugmentationLayer, core::Algorithm, cv_DataAugmentationLayer_to_Algorithm }

boxed_cast_base! { DataAugmentationLayer, crate::dnn::Layer, cv_DataAugmentationLayer_to_Layer }

pub trait DeconvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
	fn as_raw_DeconvolutionLayer(&self) -> *const c_void;

}

pub trait DeconvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::DeconvolutionLayerTraitConst {
	fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void;

}

pub struct DeconvolutionLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { DeconvolutionLayer }

impl Drop for DeconvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DeconvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_DeconvolutionLayer_delete(self.as_raw_mut_DeconvolutionLayer()) };
	}
}

unsafe impl Send for DeconvolutionLayer {}

impl core::AlgorithmTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BaseConvolutionLayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DeconvolutionLayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_DeconvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DeconvolutionLayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DeconvolutionLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DeconvolutionLayer, core::Algorithm, cv_DeconvolutionLayer_to_Algorithm }

boxed_cast_base! { DeconvolutionLayer, crate::dnn::BaseConvolutionLayer, cv_DeconvolutionLayer_to_BaseConvolutionLayer }

boxed_cast_base! { DeconvolutionLayer, crate::dnn::Layer, cv_DeconvolutionLayer_to_Layer }

pub trait DequantizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_DequantizeLayer(&self) -> *const c_void;

	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_DequantizeLayer_getPropScale_const(self.as_raw_DequantizeLayer()) };
		ret
	}
	
	#[inline]
	fn zeropoint(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_DequantizeLayer_getPropZeropoint_const(self.as_raw_DequantizeLayer()) };
		ret
	}
	
}

pub trait DequantizeLayerTrait: crate::dnn::DequantizeLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_DequantizeLayer_setPropScale_float(self.as_raw_mut_DequantizeLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_zeropoint(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_DequantizeLayer_setPropZeropoint_int(self.as_raw_mut_DequantizeLayer(), val) };
		ret
	}
	
}

pub struct DequantizeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { DequantizeLayer }

impl Drop for DequantizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DequantizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_DequantizeLayer_delete(self.as_raw_mut_DequantizeLayer()) };
	}
}

unsafe impl Send for DequantizeLayer {}

impl core::AlgorithmTraitConst for DequantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DequantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for DequantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DequantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DequantizeLayerTraitConst for DequantizeLayer {
	#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DequantizeLayerTrait for DequantizeLayer {
	#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DequantizeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DequantizeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DequantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::DequantizeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DequantizeLayer, core::Algorithm, cv_DequantizeLayer_to_Algorithm }

boxed_cast_base! { DequantizeLayer, crate::dnn::Layer, cv_DequantizeLayer_to_Layer }

pub trait DetectionModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_DetectionModel(&self) -> *const c_void;

}

pub trait DetectionModelTrait: crate::dnn::DetectionModelTraitConst + crate::dnn::ModelTrait {
	fn as_raw_mut_DetectionModel(&mut self) -> *mut c_void;

	#[inline]
	fn set_nms_across_classes(&mut self, value: bool) -> Result<crate::dnn::DetectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_setNmsAcrossClasses_bool(self.as_raw_mut_DetectionModel(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_nms_across_classes(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_getNmsAcrossClasses(self.as_raw_mut_DetectionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * conf_threshold: 0.5f
	/// * nms_threshold: 0.0f
	#[inline]
	fn detect(&mut self, frame: &dyn core::ToInputArray, class_ids: &mut core::Vector<i32>, confidences: &mut core::Vector<f32>, boxes: &mut core::Vector<core::Rect>, conf_threshold: f32, nms_threshold: f32) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_detect_const__InputArrayR_vector_int_R_vector_float_R_vector_Rect_R_float_float(self.as_raw_mut_DetectionModel(), frame.as_raw__InputArray(), class_ids.as_raw_mut_VectorOfi32(), confidences.as_raw_mut_VectorOff32(), boxes.as_raw_mut_VectorOfRect(), conf_threshold, nms_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct DetectionModel {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionModel }

impl Drop for DetectionModel {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionModel_delete(instance: *mut c_void); }
		unsafe { cv_DetectionModel_delete(self.as_raw_mut_DetectionModel()) };
	}
}

unsafe impl Send for DetectionModel {}

impl crate::dnn::ModelTraitConst for DetectionModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for DetectionModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DetectionModelTraitConst for DetectionModel {
	#[inline] fn as_raw_DetectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DetectionModelTrait for DetectionModel {
	#[inline] fn as_raw_mut_DetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionModel {
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new(model: &str, config: &str) -> Result<crate::dnn::DetectionModel> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_DetectionModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::DetectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_DetectionModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::dnn::DetectionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionModel_DetectionModel(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DetectionModel, crate::dnn::Model, cv_DetectionModel_to_Model }

pub trait DetectionOutputLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_DetectionOutputLayer(&self) -> *const c_void;

}

pub trait DetectionOutputLayerTrait: crate::dnn::DetectionOutputLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void;

}

pub struct DetectionOutputLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionOutputLayer }

impl Drop for DetectionOutputLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionOutputLayer_delete(instance: *mut c_void); }
		unsafe { cv_DetectionOutputLayer_delete(self.as_raw_mut_DetectionOutputLayer()) };
	}
}

unsafe impl Send for DetectionOutputLayer {}

impl core::AlgorithmTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DetectionOutputLayerTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DetectionOutputLayerTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionOutputLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DetectionOutputLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::DetectionOutputLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { DetectionOutputLayer, core::Algorithm, cv_DetectionOutputLayer_to_Algorithm }

boxed_cast_base! { DetectionOutputLayer, crate::dnn::Layer, cv_DetectionOutputLayer_to_Layer }

pub trait DictTraitConst {
	fn as_raw_Dict(&self) -> *const c_void;

	#[inline]
	fn has(&self, key: &str) -> Result<bool> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_has_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	unsafe fn ptr(&self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Dict_ptr_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_get_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait DictTrait: crate::dnn::DictTraitConst {
	fn as_raw_mut_Dict(&mut self) -> *mut c_void;

	#[inline]
	unsafe fn ptr_mut(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Dict_ptr_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_str(&mut self, key: &str, value: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_cv_String_const_StringR_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set(&mut self, key: &str, value: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_cv_dnn_DictValue_const_StringR_const_DictValueR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_f64(&mut self, key: &str, value: &f64) -> Result<f64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_double_const_StringR_const_doubleR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_i64(&mut self, key: &str, value: &i64) -> Result<i64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn erase(&mut self, key: &str) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_erase_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Dict {
	ptr: *mut c_void
}

opencv_type_boxed! { Dict }

impl Drop for Dict {
	fn drop(&mut self) {
		extern "C" { fn cv_Dict_delete(instance: *mut c_void); }
		unsafe { cv_Dict_delete(self.as_raw_mut_Dict()) };
	}
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTraitConst for Dict {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictTrait for Dict {
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Dict {
}

pub trait DictValueTraitConst {
	fn as_raw_DictValue(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_str(&self, idx: i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_f64(&self, idx: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_double_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_i32(&self, idx: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_i64(&self, idx: i32) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int64_t_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_int(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_string(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn is_real(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_int_value(&self, idx: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getIntValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_real_value(&self, idx: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getRealValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	#[inline]
	fn get_string_value(&self, idx: i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getStringValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait DictValueTrait: crate::dnn::DictValueTraitConst {
	fn as_raw_mut_DictValue(&mut self) -> *mut c_void;

}

pub struct DictValue {
	ptr: *mut c_void
}

opencv_type_boxed! { DictValue }

impl Drop for DictValue {
	fn drop(&mut self) {
		extern "C" { fn cv_DictValue_delete(instance: *mut c_void); }
		unsafe { cv_DictValue_delete(self.as_raw_mut_DictValue()) };
	}
}

unsafe impl Send for DictValue {}

impl crate::dnn::DictValueTraitConst for DictValue {
	#[inline] fn as_raw_DictValue(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictValueTrait for DictValue {
	#[inline] fn as_raw_mut_DictValue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DictValue {
	#[inline]
	pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueR(r.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn from_bool(i: bool) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_bool(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: 0
	#[inline]
	pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_int64_t(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_int(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_double(p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(s);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_charX(s.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ELULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ELULayer(&self) -> *const c_void;

	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ELULayer_getPropAlpha_const(self.as_raw_ELULayer()) };
		ret
	}
	
}

pub trait ELULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ELULayerTraitConst {
	fn as_raw_mut_ELULayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ELULayer_setPropAlpha_float(self.as_raw_mut_ELULayer(), val) };
		ret
	}
	
}

pub struct ELULayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ELULayer }

impl Drop for ELULayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ELULayer_delete(instance: *mut c_void); }
		unsafe { cv_ELULayer_delete(self.as_raw_mut_ELULayer()) };
	}
}

unsafe impl Send for ELULayer {}

impl crate::dnn::ActivationLayerTraitConst for ELULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ELULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ELULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ELULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ELULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ELULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ELULayerTraitConst for ELULayer {
	#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ELULayerTrait for ELULayer {
	#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ELULayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ELULayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ELULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ELULayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ELULayer, crate::dnn::ActivationLayer, cv_ELULayer_to_ActivationLayer }

boxed_cast_base! { ELULayer, core::Algorithm, cv_ELULayer_to_Algorithm }

boxed_cast_base! { ELULayer, crate::dnn::Layer, cv_ELULayer_to_Layer }

pub trait EltwiseLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_EltwiseLayer(&self) -> *const c_void;

}

pub trait EltwiseLayerTrait: crate::dnn::EltwiseLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void;

}

pub struct EltwiseLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { EltwiseLayer }

impl Drop for EltwiseLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_EltwiseLayer_delete(instance: *mut c_void); }
		unsafe { cv_EltwiseLayer_delete(self.as_raw_mut_EltwiseLayer()) };
	}
}

unsafe impl Send for EltwiseLayer {}

impl core::AlgorithmTraitConst for EltwiseLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for EltwiseLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::EltwiseLayerTraitConst for EltwiseLayer {
	#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::EltwiseLayerTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl EltwiseLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::EltwiseLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_EltwiseLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::EltwiseLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { EltwiseLayer, core::Algorithm, cv_EltwiseLayer_to_Algorithm }

boxed_cast_base! { EltwiseLayer, crate::dnn::Layer, cv_EltwiseLayer_to_Layer }

pub trait EltwiseLayerInt8TraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_EltwiseLayerInt8(&self) -> *const c_void;

}

pub trait EltwiseLayerInt8Trait: crate::dnn::EltwiseLayerInt8TraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void;

}

pub struct EltwiseLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { EltwiseLayerInt8 }

impl Drop for EltwiseLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_EltwiseLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_EltwiseLayerInt8_delete(self.as_raw_mut_EltwiseLayerInt8()) };
	}
}

unsafe impl Send for EltwiseLayerInt8 {}

impl core::AlgorithmTraitConst for EltwiseLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for EltwiseLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for EltwiseLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for EltwiseLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::EltwiseLayerInt8TraitConst for EltwiseLayerInt8 {
	#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::EltwiseLayerInt8Trait for EltwiseLayerInt8 {
	#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl EltwiseLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::EltwiseLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_EltwiseLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::EltwiseLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { EltwiseLayerInt8, core::Algorithm, cv_EltwiseLayerInt8_to_Algorithm }

boxed_cast_base! { EltwiseLayerInt8, crate::dnn::Layer, cv_EltwiseLayerInt8_to_Layer }

pub trait ErfLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ErfLayer(&self) -> *const c_void;

}

pub trait ErfLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ErfLayerTraitConst {
	fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void;

}

pub struct ErfLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ErfLayer }

impl Drop for ErfLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ErfLayer_delete(instance: *mut c_void); }
		unsafe { cv_ErfLayer_delete(self.as_raw_mut_ErfLayer()) };
	}
}

unsafe impl Send for ErfLayer {}

impl crate::dnn::ActivationLayerTraitConst for ErfLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ErfLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ErfLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ErfLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ErfLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ErfLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ErfLayerTraitConst for ErfLayer {
	#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ErfLayerTrait for ErfLayer {
	#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ErfLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ErfLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ErfLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ErfLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ErfLayer, crate::dnn::ActivationLayer, cv_ErfLayer_to_ActivationLayer }

boxed_cast_base! { ErfLayer, core::Algorithm, cv_ErfLayer_to_Algorithm }

boxed_cast_base! { ErfLayer, crate::dnn::Layer, cv_ErfLayer_to_Layer }

pub trait ExpLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ExpLayer(&self) -> *const c_void;

	#[inline]
	fn base(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_getPropBase_const(self.as_raw_ExpLayer()) };
		ret
	}
	
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_getPropScale_const(self.as_raw_ExpLayer()) };
		ret
	}
	
	#[inline]
	fn shift(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_getPropShift_const(self.as_raw_ExpLayer()) };
		ret
	}
	
}

pub trait ExpLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ExpLayerTraitConst {
	fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_base(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_setPropBase_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_setPropScale_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_shift(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_setPropShift_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}
	
}

pub struct ExpLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ExpLayer }

impl Drop for ExpLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ExpLayer_delete(instance: *mut c_void); }
		unsafe { cv_ExpLayer_delete(self.as_raw_mut_ExpLayer()) };
	}
}

unsafe impl Send for ExpLayer {}

impl crate::dnn::ActivationLayerTraitConst for ExpLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ExpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ExpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ExpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ExpLayerTraitConst for ExpLayer {
	#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ExpLayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ExpLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ExpLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ExpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ExpLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ExpLayer, crate::dnn::ActivationLayer, cv_ExpLayer_to_ActivationLayer }

boxed_cast_base! { ExpLayer, core::Algorithm, cv_ExpLayer_to_Algorithm }

boxed_cast_base! { ExpLayer, crate::dnn::Layer, cv_ExpLayer_to_Layer }

pub trait FlattenLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_FlattenLayer(&self) -> *const c_void;

}

pub trait FlattenLayerTrait: crate::dnn::FlattenLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void;

}

pub struct FlattenLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { FlattenLayer }

impl Drop for FlattenLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_FlattenLayer_delete(instance: *mut c_void); }
		unsafe { cv_FlattenLayer_delete(self.as_raw_mut_FlattenLayer()) };
	}
}

unsafe impl Send for FlattenLayer {}

impl core::AlgorithmTraitConst for FlattenLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlattenLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for FlattenLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for FlattenLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FlattenLayerTraitConst for FlattenLayer {
	#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::FlattenLayerTrait for FlattenLayer {
	#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FlattenLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FlattenLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_FlattenLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::FlattenLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FlattenLayer, core::Algorithm, cv_FlattenLayer_to_Algorithm }

boxed_cast_base! { FlattenLayer, crate::dnn::Layer, cv_FlattenLayer_to_Layer }

pub trait FloorLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_FloorLayer(&self) -> *const c_void;

}

pub trait FloorLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::FloorLayerTraitConst {
	fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void;

}

pub struct FloorLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { FloorLayer }

impl Drop for FloorLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_FloorLayer_delete(instance: *mut c_void); }
		unsafe { cv_FloorLayer_delete(self.as_raw_mut_FloorLayer()) };
	}
}

unsafe impl Send for FloorLayer {}

impl crate::dnn::ActivationLayerTraitConst for FloorLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for FloorLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for FloorLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FloorLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for FloorLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for FloorLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FloorLayerTraitConst for FloorLayer {
	#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::FloorLayerTrait for FloorLayer {
	#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FloorLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FloorLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_FloorLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::FloorLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FloorLayer, crate::dnn::ActivationLayer, cv_FloorLayer_to_ActivationLayer }

boxed_cast_base! { FloorLayer, core::Algorithm, cv_FloorLayer_to_Algorithm }

boxed_cast_base! { FloorLayer, crate::dnn::Layer, cv_FloorLayer_to_Layer }

pub trait FlowWarpLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_FlowWarpLayer(&self) -> *const c_void;

}

pub trait FlowWarpLayerTrait: crate::dnn::FlowWarpLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void;

}

pub struct FlowWarpLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { FlowWarpLayer }

impl Drop for FlowWarpLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_FlowWarpLayer_delete(instance: *mut c_void); }
		unsafe { cv_FlowWarpLayer_delete(self.as_raw_mut_FlowWarpLayer()) };
	}
}

unsafe impl Send for FlowWarpLayer {}

impl core::AlgorithmTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FlowWarpLayerTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::FlowWarpLayerTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FlowWarpLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FlowWarpLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_FlowWarpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::FlowWarpLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FlowWarpLayer, core::Algorithm, cv_FlowWarpLayer_to_Algorithm }

boxed_cast_base! { FlowWarpLayer, crate::dnn::Layer, cv_FlowWarpLayer_to_Layer }

pub trait GRULayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_GRULayer(&self) -> *const c_void;

}

pub trait GRULayerTrait: crate::dnn::GRULayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_GRULayer(&mut self) -> *mut c_void;

}

pub struct GRULayer {
	ptr: *mut c_void
}

opencv_type_boxed! { GRULayer }

impl Drop for GRULayer {
	fn drop(&mut self) {
		extern "C" { fn cv_GRULayer_delete(instance: *mut c_void); }
		unsafe { cv_GRULayer_delete(self.as_raw_mut_GRULayer()) };
	}
}

unsafe impl Send for GRULayer {}

impl core::AlgorithmTraitConst for GRULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GRULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for GRULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for GRULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::GRULayerTraitConst for GRULayer {
	#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::GRULayerTrait for GRULayer {
	#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GRULayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::GRULayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_GRULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::GRULayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { GRULayer, core::Algorithm, cv_GRULayer_to_Algorithm }

boxed_cast_base! { GRULayer, crate::dnn::Layer, cv_GRULayer_to_Layer }

pub trait HardSigmoidLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_HardSigmoidLayer(&self) -> *const c_void;

	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_getPropAlpha_const(self.as_raw_HardSigmoidLayer()) };
		ret
	}
	
	#[inline]
	fn beta(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_getPropBeta_const(self.as_raw_HardSigmoidLayer()) };
		ret
	}
	
}

pub trait HardSigmoidLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::HardSigmoidLayerTraitConst {
	fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_setPropAlpha_float(self.as_raw_mut_HardSigmoidLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_beta(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_setPropBeta_float(self.as_raw_mut_HardSigmoidLayer(), val) };
		ret
	}
	
}

pub struct HardSigmoidLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { HardSigmoidLayer }

impl Drop for HardSigmoidLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_HardSigmoidLayer_delete(instance: *mut c_void); }
		unsafe { cv_HardSigmoidLayer_delete(self.as_raw_mut_HardSigmoidLayer()) };
	}
}

unsafe impl Send for HardSigmoidLayer {}

impl crate::dnn::ActivationLayerTraitConst for HardSigmoidLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for HardSigmoidLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for HardSigmoidLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for HardSigmoidLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for HardSigmoidLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for HardSigmoidLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::HardSigmoidLayerTraitConst for HardSigmoidLayer {
	#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::HardSigmoidLayerTrait for HardSigmoidLayer {
	#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HardSigmoidLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::HardSigmoidLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_HardSigmoidLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::HardSigmoidLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { HardSigmoidLayer, crate::dnn::ActivationLayer, cv_HardSigmoidLayer_to_ActivationLayer }

boxed_cast_base! { HardSigmoidLayer, core::Algorithm, cv_HardSigmoidLayer_to_Algorithm }

boxed_cast_base! { HardSigmoidLayer, crate::dnn::Layer, cv_HardSigmoidLayer_to_Layer }

pub trait HardSwishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_HardSwishLayer(&self) -> *const c_void;

}

pub trait HardSwishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::HardSwishLayerTraitConst {
	fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void;

}

pub struct HardSwishLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { HardSwishLayer }

impl Drop for HardSwishLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_HardSwishLayer_delete(instance: *mut c_void); }
		unsafe { cv_HardSwishLayer_delete(self.as_raw_mut_HardSwishLayer()) };
	}
}

unsafe impl Send for HardSwishLayer {}

impl crate::dnn::ActivationLayerTraitConst for HardSwishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for HardSwishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for HardSwishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for HardSwishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for HardSwishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for HardSwishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::HardSwishLayerTraitConst for HardSwishLayer {
	#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::HardSwishLayerTrait for HardSwishLayer {
	#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HardSwishLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::HardSwishLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_HardSwishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::HardSwishLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { HardSwishLayer, crate::dnn::ActivationLayer, cv_HardSwishLayer_to_ActivationLayer }

boxed_cast_base! { HardSwishLayer, core::Algorithm, cv_HardSwishLayer_to_Algorithm }

boxed_cast_base! { HardSwishLayer, crate::dnn::Layer, cv_HardSwishLayer_to_Layer }

pub trait InnerProductLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_InnerProductLayer(&self) -> *const c_void;

	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayer_getPropAxis_const(self.as_raw_InnerProductLayer()) };
		ret
	}
	
}

pub trait InnerProductLayerTrait: crate::dnn::InnerProductLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayer_setPropAxis_int(self.as_raw_mut_InnerProductLayer(), val) };
		ret
	}
	
}

pub struct InnerProductLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { InnerProductLayer }

impl Drop for InnerProductLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_InnerProductLayer_delete(instance: *mut c_void); }
		unsafe { cv_InnerProductLayer_delete(self.as_raw_mut_InnerProductLayer()) };
	}
}

unsafe impl Send for InnerProductLayer {}

impl core::AlgorithmTraitConst for InnerProductLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for InnerProductLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerTraitConst for InnerProductLayer {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl InnerProductLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::InnerProductLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_InnerProductLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::InnerProductLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { InnerProductLayer, core::Algorithm, cv_InnerProductLayer_to_Algorithm }

boxed_cast_base! { InnerProductLayer, crate::dnn::Layer, cv_InnerProductLayer_to_Layer }

pub trait InnerProductLayerInt8TraitConst: crate::dnn::InnerProductLayerTraitConst {
	fn as_raw_InnerProductLayerInt8(&self) -> *const c_void;

	#[inline]
	fn input_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_getPropInput_zp_const(self.as_raw_InnerProductLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_getPropOutput_zp_const(self.as_raw_InnerProductLayerInt8()) };
		ret
	}
	
	#[inline]
	fn input_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_getPropInput_sc_const(self.as_raw_InnerProductLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_getPropOutput_sc_const(self.as_raw_InnerProductLayerInt8()) };
		ret
	}
	
}

pub trait InnerProductLayerInt8Trait: crate::dnn::InnerProductLayerInt8TraitConst + crate::dnn::InnerProductLayerTrait {
	fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_input_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_setPropInput_zp_int(self.as_raw_mut_InnerProductLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_setPropOutput_zp_int(self.as_raw_mut_InnerProductLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_input_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_setPropInput_sc_float(self.as_raw_mut_InnerProductLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_setPropOutput_sc_float(self.as_raw_mut_InnerProductLayerInt8(), val) };
		ret
	}
	
}

pub struct InnerProductLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { InnerProductLayerInt8 }

impl Drop for InnerProductLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_InnerProductLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_InnerProductLayerInt8_delete(self.as_raw_mut_InnerProductLayerInt8()) };
	}
}

unsafe impl Send for InnerProductLayerInt8 {}

impl core::AlgorithmTraitConst for InnerProductLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for InnerProductLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerTraitConst for InnerProductLayerInt8 {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for InnerProductLayerInt8 {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for InnerProductLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for InnerProductLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerInt8TraitConst for InnerProductLayerInt8 {
	#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InnerProductLayerInt8Trait for InnerProductLayerInt8 {
	#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl InnerProductLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::InnerProductLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_InnerProductLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::InnerProductLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { InnerProductLayerInt8, core::Algorithm, cv_InnerProductLayerInt8_to_Algorithm }

boxed_cast_base! { InnerProductLayerInt8, crate::dnn::InnerProductLayer, cv_InnerProductLayerInt8_to_InnerProductLayer }

boxed_cast_base! { InnerProductLayerInt8, crate::dnn::Layer, cv_InnerProductLayerInt8_to_Layer }

pub trait InterpLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_InterpLayer(&self) -> *const c_void;

}

pub trait InterpLayerTrait: crate::dnn::InterpLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void;

}

pub struct InterpLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { InterpLayer }

impl Drop for InterpLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_InterpLayer_delete(instance: *mut c_void); }
		unsafe { cv_InterpLayer_delete(self.as_raw_mut_InterpLayer()) };
	}
}

unsafe impl Send for InterpLayer {}

impl core::AlgorithmTraitConst for InterpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for InterpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for InterpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for InterpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InterpLayerTraitConst for InterpLayer {
	#[inline] fn as_raw_InterpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InterpLayerTrait for InterpLayer {
	#[inline] fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl InterpLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_InterpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { InterpLayer, core::Algorithm, cv_InterpLayer_to_Algorithm }

boxed_cast_base! { InterpLayer, crate::dnn::Layer, cv_InterpLayer_to_Layer }

pub trait KeypointsModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_KeypointsModel(&self) -> *const c_void;

}

pub trait KeypointsModelTrait: crate::dnn::KeypointsModelTraitConst + crate::dnn::ModelTrait {
	fn as_raw_mut_KeypointsModel(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * thresh: 0.5
	#[inline]
	fn estimate(&mut self, frame: &dyn core::ToInputArray, thresh: f32) -> Result<core::Vector<core::Point2f>> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_KeypointsModel_estimate_const__InputArrayR_float(self.as_raw_mut_KeypointsModel(), frame.as_raw__InputArray(), thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct KeypointsModel {
	ptr: *mut c_void
}

opencv_type_boxed! { KeypointsModel }

impl Drop for KeypointsModel {
	fn drop(&mut self) {
		extern "C" { fn cv_KeypointsModel_delete(instance: *mut c_void); }
		unsafe { cv_KeypointsModel_delete(self.as_raw_mut_KeypointsModel()) };
	}
}

unsafe impl Send for KeypointsModel {}

impl crate::dnn::ModelTraitConst for KeypointsModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for KeypointsModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::KeypointsModelTraitConst for KeypointsModel {
	#[inline] fn as_raw_KeypointsModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::KeypointsModelTrait for KeypointsModel {
	#[inline] fn as_raw_mut_KeypointsModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeypointsModel {
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new(model: &str, config: &str) -> Result<crate::dnn::KeypointsModel> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_KeypointsModel_KeypointsModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::KeypointsModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::KeypointsModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_KeypointsModel_KeypointsModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::KeypointsModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { KeypointsModel, crate::dnn::Model, cv_KeypointsModel_to_Model }

pub trait LRNLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_LRNLayer(&self) -> *const c_void;

	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropType_const(self.as_raw_LRNLayer()) };
		ret
	}
	
	#[inline]
	fn size(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropSize_const(self.as_raw_LRNLayer()) };
		ret
	}
	
	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropAlpha_const(self.as_raw_LRNLayer()) };
		ret
	}
	
	#[inline]
	fn beta(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropBeta_const(self.as_raw_LRNLayer()) };
		ret
	}
	
	#[inline]
	fn bias(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropBias_const(self.as_raw_LRNLayer()) };
		ret
	}
	
	#[inline]
	fn norm_by_size(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_LRNLayer_getPropNormBySize_const(self.as_raw_LRNLayer()) };
		ret
	}
	
}

pub trait LRNLayerTrait: crate::dnn::LRNLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropType_int(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropSize_int(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropAlpha_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_beta(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropBeta_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_bias(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropBias_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_norm_by_size(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_setPropNormBySize_bool(self.as_raw_mut_LRNLayer(), val) };
		ret
	}
	
}

pub struct LRNLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { LRNLayer }

impl Drop for LRNLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_LRNLayer_delete(instance: *mut c_void); }
		unsafe { cv_LRNLayer_delete(self.as_raw_mut_LRNLayer()) };
	}
}

unsafe impl Send for LRNLayer {}

impl core::AlgorithmTraitConst for LRNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LRNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for LRNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for LRNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LRNLayerTraitConst for LRNLayer {
	#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LRNLayerTrait for LRNLayer {
	#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LRNLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LRNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LRNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::LRNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LRNLayer, core::Algorithm, cv_LRNLayer_to_Algorithm }

boxed_cast_base! { LRNLayer, crate::dnn::Layer, cv_LRNLayer_to_Layer }

pub trait LSTMLayerConst: crate::dnn::LayerTraitConst {
	fn as_raw_LSTMLayer(&self) -> *const c_void;

}

pub trait LSTMLayer: crate::dnn::LSTMLayerConst + crate::dnn::LayerTrait {
	fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_weights(&mut self, wh: &core::Mat, wx: &core::Mat, b: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(self.as_raw_mut_LSTMLayer(), wh.as_raw_Mat(), wx.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * out_tail_shape: MatShape()
	#[inline]
	fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::MatShape) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(self.as_raw_mut_LSTMLayer(), out_tail_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_: true
	#[inline]
	fn set_use_timstamps_dim(&mut self, use_: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_mut_LSTMLayer(), use_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * produce: false
	#[inline]
	fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_mut_LSTMLayer(), produce, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(mut input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_mut_LSTMLayer(), input_name.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(self.as_raw_mut_LSTMLayer(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn LSTMLayer + '_ {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<dyn crate::dnn::LSTMLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::dnn::LSTMLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait LayerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Layer(&self) -> *const c_void;

	#[inline]
	fn blobs(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_dnn_Layer_getPropBlobs_const(self.as_raw_Layer()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_dnn_Layer_getPropName_const(self.as_raw_Layer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn typ(&self) -> String {
		let ret = unsafe { sys::cv_dnn_Layer_getPropType_const(self.as_raw_Layer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn preferable_target(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_Layer_getPropPreferableTarget_const(self.as_raw_Layer()) };
		ret
	}
	
	#[inline]
	fn apply_halide_scheduler(&self, node: &mut core::Ptr<crate::dnn::BackendNode>, inputs: &core::Vector<core::Mat>, outputs: &core::Vector<core::Mat>, target_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_R_const_vector_MatX_R_const_vector_Mat_R_int(self.as_raw_Layer(), node.as_raw_mut_PtrOfBackendNode(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_scale_shift(&self, scale: &mut core::Mat, shift: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getScaleShift_const_MatR_MatR(self.as_raw_Layer(), scale.as_raw_mut_Mat(), shift.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_scale_zeropoint(&self, scale: &mut f32, zeropoint: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getScaleZeropoint_const_floatR_intR(self.as_raw_Layer(), scale, zeropoint, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_shapes(&self, inputs: &core::Vector<crate::dnn::MatShape>, required_outputs: i32, outputs: &mut core::Vector<crate::dnn::MatShape>, internals: &mut core::Vector<crate::dnn::MatShape>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), required_outputs, outputs.as_raw_mut_VectorOfMatShape(), internals.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flops(&self, inputs: &core::Vector<crate::dnn::MatShape>, outputs: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_R_const_vector_MatShape_R(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), outputs.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait LayerTrait: core::AlgorithmTrait + crate::dnn::LayerTraitConst {
	fn as_raw_mut_Layer(&mut self) -> *mut c_void;

	#[inline]
	fn set_blobs(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_dnn_Layer_setPropBlobs_vector_Mat_(self.as_raw_mut_Layer(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_name(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_Layer_setPropName_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_type(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_Layer_setPropType_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_preferable_target(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_Layer_setPropPreferableTarget_int(self.as_raw_mut_Layer(), val) };
		ret
	}
	
	#[inline]
	fn finalize(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward_mat(&mut self, input: &mut core::Vector<core::Mat>, output: &mut core::Vector<core::Mat>, internals: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_vector_MatX_R_vector_Mat_R_vector_Mat_R(self.as_raw_mut_Layer(), input.as_raw_mut_VectorOfMat(), output.as_raw_mut_VectorOfMat(), internals.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn try_quantize(&mut self, scales: &core::Vector<core::Vector<f32>>, zeropoints: &core::Vector<core::Vector<i32>>, params: &mut crate::dnn::LayerParams) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_tryQuantize_const_vector_vector_float__R_const_vector_vector_int__R_LayerParamsR(self.as_raw_mut_Layer(), scales.as_raw_VectorOfVectorOff32(), zeropoints.as_raw_VectorOfVectorOfi32(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward_fallback(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn finalize_mat_to(&mut self, inputs: &core::Vector<core::Mat>, outputs: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const_vector_Mat_R_vector_Mat_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn finalize_mat(&mut self, inputs: &core::Vector<core::Mat>) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const_vector_Mat_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn run(&mut self, inputs: &core::Vector<core::Mat>, outputs: &mut core::Vector<core::Mat>, internals: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_run_const_vector_Mat_R_vector_Mat_R_vector_Mat_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_mut_VectorOfMat(), internals.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(mut input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_mut_Layer(), input_name.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_outputNameToIndex_const_StringR(self.as_raw_mut_Layer(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn support_backend(&mut self, backend_id: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_supportBackend_int(self.as_raw_mut_Layer(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn init_halide(&mut self, inputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn init_ngraph(&mut self, inputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendNode__R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn init_vk_com(&mut self, inputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initVkCom_const_vector_Ptr_BackendWrapper__R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn init_webnn(&mut self, inputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initWebnn_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendNode__R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	unsafe fn init_cuda(&mut self, context: *mut c_void, inputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>, outputs: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Layer_initCUDA_voidX_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendWrapper__R(self.as_raw_mut_Layer(), context, inputs.as_raw_VectorOfPtrOfBackendWrapper(), outputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	unsafe fn init_tim_vx(&mut self, tim_vx_info: *mut c_void, inputs_wrapper: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>, outputs_wrapper: &core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>, is_last: bool) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Layer_initTimVX_voidX_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendWrapper__R_bool(self.as_raw_mut_Layer(), tim_vx_info, inputs_wrapper.as_raw_VectorOfPtrOfBackendWrapper(), outputs_wrapper.as_raw_VectorOfPtrOfBackendWrapper(), is_last, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn try_attach(&mut self, node: &core::Ptr<crate::dnn::BackendNode>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_R(self.as_raw_mut_Layer(), node.as_raw_PtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_activation(&mut self, layer: &core::Ptr<crate::dnn::ActivationLayer>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_R(self.as_raw_mut_Layer(), layer.as_raw_PtrOfActivationLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn try_fuse(&mut self, top: &mut core::Ptr<crate::dnn::Layer>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_tryFuse_Ptr_Layer_R(self.as_raw_mut_Layer(), top.as_raw_mut_PtrOfLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn unset_attached(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_unsetAttached(self.as_raw_mut_Layer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn update_memory_shapes(&mut self, inputs: &core::Vector<crate::dnn::MatShape>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_updateMemoryShapes_const_vector_MatShape_R(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_setParamsFrom_const_LayerParamsR(self.as_raw_mut_Layer(), params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Layer {
	ptr: *mut c_void
}

opencv_type_boxed! { Layer }

impl Drop for Layer {
	fn drop(&mut self) {
		extern "C" { fn cv_Layer_delete(instance: *mut c_void); }
		unsafe { cv_Layer_delete(self.as_raw_mut_Layer()) };
	}
}

unsafe impl Send for Layer {}

impl core::AlgorithmTraitConst for Layer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Layer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for Layer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for Layer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Layer {
	#[inline]
	pub fn default() -> Result<crate::dnn::Layer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_Layer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Layer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(params: &crate::dnn::LayerParams) -> Result<crate::dnn::Layer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_Layer_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Layer::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_descendant! { Layer, crate::dnn::AccumLayer, cv_Layer_to_AccumLayer }

boxed_cast_descendant! { Layer, crate::dnn::ActivationLayer, cv_Layer_to_ActivationLayer }

boxed_cast_descendant! { Layer, crate::dnn::ArgLayer, cv_Layer_to_ArgLayer }

boxed_cast_descendant! { Layer, crate::dnn::BaseConvolutionLayer, cv_Layer_to_BaseConvolutionLayer }

boxed_cast_descendant! { Layer, crate::dnn::BlankLayer, cv_Layer_to_BlankLayer }

boxed_cast_descendant! { Layer, crate::dnn::CompareLayer, cv_Layer_to_CompareLayer }

boxed_cast_descendant! { Layer, crate::dnn::ConcatLayer, cv_Layer_to_ConcatLayer }

boxed_cast_descendant! { Layer, crate::dnn::ConstLayer, cv_Layer_to_ConstLayer }

boxed_cast_descendant! { Layer, crate::dnn::CorrelationLayer, cv_Layer_to_CorrelationLayer }

boxed_cast_descendant! { Layer, crate::dnn::CropAndResizeLayer, cv_Layer_to_CropAndResizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::CropLayer, cv_Layer_to_CropLayer }

boxed_cast_descendant! { Layer, crate::dnn::CumSumLayer, cv_Layer_to_CumSumLayer }

boxed_cast_descendant! { Layer, crate::dnn::DataAugmentationLayer, cv_Layer_to_DataAugmentationLayer }

boxed_cast_descendant! { Layer, crate::dnn::DequantizeLayer, cv_Layer_to_DequantizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::DetectionOutputLayer, cv_Layer_to_DetectionOutputLayer }

boxed_cast_descendant! { Layer, crate::dnn::EltwiseLayer, cv_Layer_to_EltwiseLayer }

boxed_cast_descendant! { Layer, crate::dnn::EltwiseLayerInt8, cv_Layer_to_EltwiseLayerInt8 }

boxed_cast_descendant! { Layer, crate::dnn::FlattenLayer, cv_Layer_to_FlattenLayer }

boxed_cast_descendant! { Layer, crate::dnn::FlowWarpLayer, cv_Layer_to_FlowWarpLayer }

boxed_cast_descendant! { Layer, crate::dnn::GRULayer, cv_Layer_to_GRULayer }

boxed_cast_descendant! { Layer, crate::dnn::InnerProductLayer, cv_Layer_to_InnerProductLayer }

boxed_cast_descendant! { Layer, crate::dnn::InterpLayer, cv_Layer_to_InterpLayer }

boxed_cast_descendant! { Layer, crate::dnn::LRNLayer, cv_Layer_to_LRNLayer }

boxed_cast_descendant! { Layer, crate::dnn::MVNLayer, cv_Layer_to_MVNLayer }

boxed_cast_descendant! { Layer, crate::dnn::MaxUnpoolLayer, cv_Layer_to_MaxUnpoolLayer }

boxed_cast_descendant! { Layer, crate::dnn::NormalizeBBoxLayer, cv_Layer_to_NormalizeBBoxLayer }

boxed_cast_descendant! { Layer, crate::dnn::PaddingLayer, cv_Layer_to_PaddingLayer }

boxed_cast_descendant! { Layer, crate::dnn::PermuteLayer, cv_Layer_to_PermuteLayer }

boxed_cast_descendant! { Layer, crate::dnn::PoolingLayer, cv_Layer_to_PoolingLayer }

boxed_cast_descendant! { Layer, crate::dnn::PriorBoxLayer, cv_Layer_to_PriorBoxLayer }

boxed_cast_descendant! { Layer, crate::dnn::ProposalLayer, cv_Layer_to_ProposalLayer }

boxed_cast_descendant! { Layer, crate::dnn::QuantizeLayer, cv_Layer_to_QuantizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReduceLayer, cv_Layer_to_ReduceLayer }

boxed_cast_descendant! { Layer, crate::dnn::RegionLayer, cv_Layer_to_RegionLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReorgLayer, cv_Layer_to_ReorgLayer }

boxed_cast_descendant! { Layer, crate::dnn::RequantizeLayer, cv_Layer_to_RequantizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReshapeLayer, cv_Layer_to_ReshapeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ResizeLayer, cv_Layer_to_ResizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ScaleLayer, cv_Layer_to_ScaleLayer }

boxed_cast_descendant! { Layer, crate::dnn::ShiftLayer, cv_Layer_to_ShiftLayer }

boxed_cast_descendant! { Layer, crate::dnn::ShiftLayerInt8, cv_Layer_to_ShiftLayerInt8 }

boxed_cast_descendant! { Layer, crate::dnn::ShuffleChannelLayer, cv_Layer_to_ShuffleChannelLayer }

boxed_cast_descendant! { Layer, crate::dnn::SliceLayer, cv_Layer_to_SliceLayer }

boxed_cast_descendant! { Layer, crate::dnn::SoftmaxLayer, cv_Layer_to_SoftmaxLayer }

boxed_cast_descendant! { Layer, crate::dnn::SplitLayer, cv_Layer_to_SplitLayer }

boxed_cast_base! { Layer, core::Algorithm, cv_Layer_to_Algorithm }

pub trait LayerFactoryTraitConst {
	fn as_raw_LayerFactory(&self) -> *const c_void;

}

pub trait LayerFactoryTrait: crate::dnn::LayerFactoryTraitConst {
	fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void;

}

pub struct LayerFactory {
	ptr: *mut c_void
}

opencv_type_boxed! { LayerFactory }

impl Drop for LayerFactory {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerFactory_delete(instance: *mut c_void); }
		unsafe { cv_LayerFactory_delete(self.as_raw_mut_LayerFactory()) };
	}
}

unsafe impl Send for LayerFactory {}

impl crate::dnn::LayerFactoryTraitConst for LayerFactory {
	#[inline] fn as_raw_LayerFactory(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerFactoryTrait for LayerFactory {
	#[inline] fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LayerFactory {
	#[inline]
	pub fn register_layer(typ: &str, constructor: crate::dnn::LayerFactory_Constructor) -> Result<()> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(typ.opencv_as_extern(), constructor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn unregister_layer(typ: &str) -> Result<()> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn is_layer_registered(typ: &str) -> Result<bool> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_isLayerRegistered_const_stringR(typ.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn create_layer_instance(typ: &str, params: &mut crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait LayerParamsTraitConst: crate::dnn::DictTraitConst {
	fn as_raw_LayerParams(&self) -> *const c_void;

	#[inline]
	fn blobs(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_dnn_LayerParams_getPropBlobs_const(self.as_raw_LayerParams()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_dnn_LayerParams_getPropName_const(self.as_raw_LayerParams()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn typ(&self) -> String {
		let ret = unsafe { sys::cv_dnn_LayerParams_getPropType_const(self.as_raw_LayerParams()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait LayerParamsTrait: crate::dnn::DictTrait + crate::dnn::LayerParamsTraitConst {
	fn as_raw_mut_LayerParams(&mut self) -> *mut c_void;

	#[inline]
	fn set_blobs(&mut self, mut val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_dnn_LayerParams_setPropBlobs_vector_Mat_(self.as_raw_mut_LayerParams(), val.as_raw_mut_VectorOfMat()) };
		ret
	}
	
	#[inline]
	fn set_name(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_LayerParams_setPropName_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_type(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_LayerParams_setPropType_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) };
		ret
	}
	
}

pub struct LayerParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LayerParams }

impl Drop for LayerParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerParams_delete(instance: *mut c_void); }
		unsafe { cv_LayerParams_delete(self.as_raw_mut_LayerParams()) };
	}
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTraitConst for LayerParams {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictTrait for LayerParams {
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerParamsTraitConst for LayerParams {
	#[inline] fn as_raw_LayerParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerParamsTrait for LayerParams {
	#[inline] fn as_raw_mut_LayerParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LayerParams {
}

boxed_cast_base! { LayerParams, crate::dnn::Dict, cv_LayerParams_to_Dict }

pub trait LogLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_LogLayer(&self) -> *const c_void;

}

pub trait LogLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::LogLayerTraitConst {
	fn as_raw_mut_LogLayer(&mut self) -> *mut c_void;

}

pub struct LogLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { LogLayer }

impl Drop for LogLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_LogLayer_delete(instance: *mut c_void); }
		unsafe { cv_LogLayer_delete(self.as_raw_mut_LogLayer()) };
	}
}

unsafe impl Send for LogLayer {}

impl crate::dnn::ActivationLayerTraitConst for LogLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for LogLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for LogLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LogLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for LogLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for LogLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LogLayerTraitConst for LogLayer {
	#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LogLayerTrait for LogLayer {
	#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LogLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LogLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LogLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::LogLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LogLayer, crate::dnn::ActivationLayer, cv_LogLayer_to_ActivationLayer }

boxed_cast_base! { LogLayer, core::Algorithm, cv_LogLayer_to_Algorithm }

boxed_cast_base! { LogLayer, crate::dnn::Layer, cv_LogLayer_to_Layer }

pub trait MVNLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_MVNLayer(&self) -> *const c_void;

	#[inline]
	fn eps(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_MVNLayer_getPropEps_const(self.as_raw_MVNLayer()) };
		ret
	}
	
	#[inline]
	fn norm_variance(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_MVNLayer_getPropNormVariance_const(self.as_raw_MVNLayer()) };
		ret
	}
	
	#[inline]
	fn across_channels(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_MVNLayer_getPropAcrossChannels_const(self.as_raw_MVNLayer()) };
		ret
	}
	
}

pub trait MVNLayerTrait: crate::dnn::LayerTrait + crate::dnn::MVNLayerTraitConst {
	fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_eps(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_setPropEps_float(self.as_raw_mut_MVNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_norm_variance(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_setPropNormVariance_bool(self.as_raw_mut_MVNLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_across_channels(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_setPropAcrossChannels_bool(self.as_raw_mut_MVNLayer(), val) };
		ret
	}
	
}

pub struct MVNLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { MVNLayer }

impl Drop for MVNLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_MVNLayer_delete(instance: *mut c_void); }
		unsafe { cv_MVNLayer_delete(self.as_raw_mut_MVNLayer()) };
	}
}

unsafe impl Send for MVNLayer {}

impl core::AlgorithmTraitConst for MVNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MVNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for MVNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MVNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MVNLayerTraitConst for MVNLayer {
	#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MVNLayerTrait for MVNLayer {
	#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MVNLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MVNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MVNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MVNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { MVNLayer, core::Algorithm, cv_MVNLayer_to_Algorithm }

boxed_cast_base! { MVNLayer, crate::dnn::Layer, cv_MVNLayer_to_Layer }

pub trait MaxUnpoolLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_MaxUnpoolLayer(&self) -> *const c_void;

	#[inline]
	fn pool_kernel(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_getPropPoolKernel_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn pool_pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_getPropPoolPad_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn pool_stride(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_getPropPoolStride_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait MaxUnpoolLayerTrait: crate::dnn::LayerTrait + crate::dnn::MaxUnpoolLayerTraitConst {
	fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_pool_kernel(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_setPropPoolKernel_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_pool_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_setPropPoolPad_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_pool_stride(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_setPropPoolStride_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
		ret
	}
	
}

pub struct MaxUnpoolLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { MaxUnpoolLayer }

impl Drop for MaxUnpoolLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_MaxUnpoolLayer_delete(instance: *mut c_void); }
		unsafe { cv_MaxUnpoolLayer_delete(self.as_raw_mut_MaxUnpoolLayer()) };
	}
}

unsafe impl Send for MaxUnpoolLayer {}

impl core::AlgorithmTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MaxUnpoolLayerTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MaxUnpoolLayerTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MaxUnpoolLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MaxUnpoolLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MaxUnpoolLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { MaxUnpoolLayer, core::Algorithm, cv_MaxUnpoolLayer_to_Algorithm }

boxed_cast_base! { MaxUnpoolLayer, crate::dnn::Layer, cv_MaxUnpoolLayer_to_Layer }

pub trait MishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_MishLayer(&self) -> *const c_void;

}

pub trait MishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::MishLayerTraitConst {
	fn as_raw_mut_MishLayer(&mut self) -> *mut c_void;

}

pub struct MishLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { MishLayer }

impl Drop for MishLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_MishLayer_delete(instance: *mut c_void); }
		unsafe { cv_MishLayer_delete(self.as_raw_mut_MishLayer()) };
	}
}

unsafe impl Send for MishLayer {}

impl crate::dnn::ActivationLayerTraitConst for MishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for MishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for MishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for MishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MishLayerTraitConst for MishLayer {
	#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MishLayerTrait for MishLayer {
	#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MishLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MishLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MishLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { MishLayer, crate::dnn::ActivationLayer, cv_MishLayer_to_ActivationLayer }

boxed_cast_base! { MishLayer, core::Algorithm, cv_MishLayer_to_Algorithm }

boxed_cast_base! { MishLayer, crate::dnn::Layer, cv_MishLayer_to_Layer }

pub trait ModelTraitConst {
	fn as_raw_Model(&self) -> *const c_void;

	#[inline]
	fn predict(&self, frame: &dyn core::ToInputArray, outs: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame);
		output_array_arg!(outs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_predict_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Model(), frame.as_raw__InputArray(), outs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_network_(&self) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_getNetwork__const(self.as_raw_Model(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ModelTrait: crate::dnn::ModelTraitConst {
	fn as_raw_mut_Model(&mut self) -> *mut c_void;

	#[inline]
	fn set_input_size(&mut self, size: core::Size) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputSize_const_SizeR(self.as_raw_mut_Model(), &size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_input_size_1(&mut self, width: i32, height: i32) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputSize_int_int(self.as_raw_mut_Model(), width, height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_input_mean(&mut self, mean: core::Scalar) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputMean_const_ScalarR(self.as_raw_mut_Model(), &mean, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_input_scale(&mut self, scale: f64) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputScale_double(self.as_raw_mut_Model(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_input_crop(&mut self, crop: bool) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputCrop_bool(self.as_raw_mut_Model(), crop, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_input_swap_rb(&mut self, swap_rb: bool) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputSwapRB_bool(self.as_raw_mut_Model(), swap_rb, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * size: Size()
	/// * mean: Scalar()
	/// * swap_rb: false
	/// * crop: false
	#[inline]
	fn set_input_params(&mut self, scale: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setInputParams_double_const_SizeR_const_ScalarR_bool_bool(self.as_raw_mut_Model(), scale, &size, &mean, swap_rb, crop, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_preferable_backend(&mut self, backend_id: crate::dnn::Backend) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setPreferableBackend_Backend(self.as_raw_mut_Model(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_preferable_target(&mut self, target_id: crate::dnn::Target) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_setPreferableTarget_Target(self.as_raw_mut_Model(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_network__1(&mut self) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_getNetwork_(self.as_raw_mut_Model(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct Model {
	ptr: *mut c_void
}

opencv_type_boxed! { Model }

impl Drop for Model {
	fn drop(&mut self) {
		extern "C" { fn cv_Model_delete(instance: *mut c_void); }
		unsafe { cv_Model_delete(self.as_raw_mut_Model()) };
	}
}

unsafe impl Send for Model {}

impl crate::dnn::ModelTraitConst for Model {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for Model {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Model {
	#[inline]
	pub fn default() -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_Model(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn copy(unnamed: &crate::dnn::Model) -> crate::dnn::Model {
		let ret = unsafe { sys::cv_dnn_Model_Model_const_ModelR(unnamed.as_raw_Model()) };
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	pub fn copy_mut(unnamed: &mut crate::dnn::Model) -> crate::dnn::Model {
		let ret = unsafe { sys::cv_dnn_Model_Model_ModelR(unnamed.as_raw_mut_Model()) };
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		ret
	}
	
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new(model: &str, config: &str) -> Result<crate::dnn::Model> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_Model_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::Model> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Model_Model_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait NetTraitConst {
	fn as_raw_Net(&self) -> *const c_void;

	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_id(&self, layer: &str) -> Result<i32> {
		extern_container_arg!(layer);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerId_const_const_StringR(self.as_raw_Net(), layer.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_names(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_layer(&self, layer_id: i32) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_int(self.as_raw_Net(), layer_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_1(&self, layer_name: &str) -> Result<core::Ptr<crate::dnn::Layer>> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_const_StringR(self.as_raw_Net(), layer_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_2(&self, layer_id: &crate::dnn::Net_LayerId) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_const_LayerIdR(self.as_raw_Net(), layer_id.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_inputs(&self, layer_id: i32) -> Result<core::Vector<core::Ptr<crate::dnn::Layer>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerInputs_const_int(self.as_raw_Net(), layer_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Ptr<crate::dnn::Layer>>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_input_details(&self, scales: &mut core::Vector<f32>, zeropoints: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getInputDetails_const_vector_float_R_vector_int_R(self.as_raw_Net(), scales.as_raw_mut_VectorOff32(), zeropoints.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_output_details(&self, scales: &mut core::Vector<f32>, zeropoints: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getOutputDetails_const_vector_float_R_vector_int_R(self.as_raw_Net(), scales.as_raw_mut_VectorOff32(), zeropoints.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * num_param: 0
	#[inline]
	fn get_param(&self, layer: i32, num_param: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_int_int(self.as_raw_Net(), layer, num_param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * num_param: 0
	#[inline]
	fn get_param_1(&self, layer_name: &str, num_param: i32) -> Result<core::Mat> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_const_StringR_int(self.as_raw_Net(), layer_name.opencv_as_extern(), num_param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_unconnected_out_layers(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayers_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_unconnected_out_layers_names(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayersNames_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_layers_shapes(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layers_ids: &mut core::Vector<i32>, in_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>, out_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_R_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layers_ids.as_raw_mut_VectorOfi32(), in_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layers_shapes_1(&self, net_input_shape: &crate::dnn::MatShape, layers_ids: &mut core::Vector<i32>, in_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>, out_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layers_ids.as_raw_mut_VectorOfi32(), in_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_shapes(&self, net_input_shape: &crate::dnn::MatShape, layer_id: i32, in_layer_shapes: &mut core::Vector<crate::dnn::MatShape>, out_layer_shapes: &mut core::Vector<crate::dnn::MatShape>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vector_MatShape_R_vector_MatShape_R(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_id, in_layer_shapes.as_raw_mut_VectorOfMatShape(), out_layer_shapes.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_shapes_1(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layer_id: i32, in_layer_shapes: &mut core::Vector<crate::dnn::MatShape>, out_layer_shapes: &mut core::Vector<crate::dnn::MatShape>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_id, in_layer_shapes.as_raw_mut_VectorOfMatShape(), out_layer_shapes.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flops(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_vector_MatShape_R(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flops_1(&self, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_MatShapeR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flops_2(&self, layer_id: i32, net_input_shapes: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_int_const_vector_MatShape_R(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_flops_3(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layer_types(&self, layers_types: &mut core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerTypes_const_vector_String_R(self.as_raw_Net(), layers_types.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_layers_count(&self, layer_type: &str) -> Result<i32> {
		extern_container_arg!(layer_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersCount_const_const_StringR(self.as_raw_Net(), layer_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_size_tR_size_tR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption_1(&self, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption_for_layer(&self, layer_id: i32, net_input_shapes: &core::Vector<crate::dnn::MatShape>, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_int_const_vector_MatShape_R_size_tR_size_tR(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption_2(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption_for_layers(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layer_ids: &mut core::Vector<i32>, weights: &mut core::Vector<size_t>, blobs: &mut core::Vector<size_t>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_vector_int_R_vector_size_t_R_vector_size_t_R(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_ids.as_raw_mut_VectorOfi32(), weights.as_raw_mut_VectorOfsize_t(), blobs.as_raw_mut_VectorOfsize_t(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_memory_consumption_3(&self, net_input_shape: &crate::dnn::MatShape, layer_ids: &mut core::Vector<i32>, weights: &mut core::Vector<size_t>, blobs: &mut core::Vector<size_t>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vector_int_R_vector_size_t_R_vector_size_t_R(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_ids.as_raw_mut_VectorOfi32(), weights.as_raw_mut_VectorOfsize_t(), blobs.as_raw_mut_VectorOfsize_t(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait NetTrait: crate::dnn::NetTraitConst {
	fn as_raw_mut_Net(&mut self) -> *mut c_void;

	#[inline]
	fn dump(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_dump(self.as_raw_mut_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn dump_to_file(&mut self, path: &str) -> Result<()> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_dumpToFile_const_StringR(self.as_raw_mut_Net(), path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add_layer_type(&mut self, name: &str, typ: &str, dtype: &i32, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringR_const_StringR_const_intR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), dtype, params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add_layer(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add_layer_to_prev_type(&mut self, name: &str, typ: &str, dtype: &i32, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_const_intR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), dtype, params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
		extern_container_arg!(mut out_pin);
		extern_container_arg!(mut inp_pin);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_mut_Net(), out_pin.opencv_as_extern_mut(), inp_pin.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_mut_Net(), out_layer_id, out_num, inp_layer_id, inp_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn register_output(&mut self, output_name: &str, layer_id: i32, output_port: i32) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_registerOutput_const_stringR_int_int(self.as_raw_mut_Net(), output_name.opencv_as_extern(), layer_id, output_port, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_inputs_names(&mut self, input_blob_names: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInputsNames_const_vector_String_R(self.as_raw_mut_Net(), input_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_input_shape(&mut self, input_name: &str, shape: &crate::dnn::MatShape) -> Result<()> {
		extern_container_arg!(input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(self.as_raw_mut_Net(), input_name.opencv_as_extern(), shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output_name: String()
	#[inline]
	fn forward_single(&mut self, output_name: &str) -> Result<core::Mat> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const_StringR(self.as_raw_mut_Net(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output_name: String()
	#[inline]
	fn forward_async(&mut self, output_name: &str) -> Result<core::AsyncArray> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forwardAsync_const_StringR(self.as_raw_mut_Net(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output_name: String()
	#[inline]
	fn forward_layer(&mut self, output_blobs: &mut dyn core::ToOutputArray, output_name: &str) -> Result<()> {
		output_array_arg!(output_blobs);
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayR_const_StringR(self.as_raw_mut_Net(), output_blobs.as_raw__OutputArray(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward(&mut self, output_blobs: &mut dyn core::ToOutputArray, out_blob_names: &core::Vector<String>) -> Result<()> {
		output_array_arg!(output_blobs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayR_const_vector_String_R(self.as_raw_mut_Net(), output_blobs.as_raw__OutputArray(), out_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn forward_and_retrieve(&mut self, output_blobs: &mut core::Vector<core::Vector<core::Mat>>, out_blob_names: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_vector_vector_Mat__R_const_vector_String_R(self.as_raw_mut_Net(), output_blobs.as_raw_mut_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn quantize(&mut self, calib_data: &dyn core::ToInputArray, inputs_dtype: i32, outputs_dtype: i32) -> Result<crate::dnn::Net> {
		input_array_arg!(calib_data);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_quantize_const__InputArrayR_int_int(self.as_raw_mut_Net(), calib_data.as_raw__InputArray(), inputs_dtype, outputs_dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_halide_scheduler(&mut self, scheduler: &str) -> Result<()> {
		extern_container_arg!(scheduler);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setHalideScheduler_const_StringR(self.as_raw_mut_Net(), scheduler.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setPreferableBackend_int(self.as_raw_mut_Net(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setPreferableTarget_int(self.as_raw_mut_Net(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * name: ""
	/// * scalefactor: 1.0
	/// * mean: Scalar()
	#[inline]
	fn set_input(&mut self, blob: &dyn core::ToInputArray, name: &str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
		input_array_arg!(blob);
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(self.as_raw_mut_Net(), blob.as_raw__InputArray(), name.opencv_as_extern(), scalefactor, &mean, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_param(&mut self, layer: i32, num_param: i32, blob: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setParam_int_int_const_MatR(self.as_raw_mut_Net(), layer, num_param, blob.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_param_1(&mut self, layer_name: &str, num_param: i32, blob: &core::Mat) -> Result<()> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setParam_const_StringR_int_const_MatR(self.as_raw_mut_Net(), layer_name.opencv_as_extern(), num_param, blob.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn enable_fusion(&mut self, fusion: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_enableFusion_bool(self.as_raw_mut_Net(), fusion, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_perf_profile(&mut self, timings: &mut core::Vector<f64>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getPerfProfile_vector_double_R(self.as_raw_mut_Net(), timings.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Net {
	ptr: *mut c_void
}

opencv_type_boxed! { Net }

impl Drop for Net {
	fn drop(&mut self) {
		extern "C" { fn cv_Net_delete(instance: *mut c_void); }
		unsafe { cv_Net_delete(self.as_raw_mut_Net()) };
	}
}

unsafe impl Send for Net {}

impl crate::dnn::NetTraitConst for Net {
	#[inline] fn as_raw_Net(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::NetTrait for Net {
	#[inline] fn as_raw_mut_Net(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Net {
	#[inline]
	pub fn default() -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_Net(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn read_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
		extern_container_arg!(xml);
		extern_container_arg!(bin);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(xml.opencv_as_extern(), bin.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn read_from_model_optimizer_1(buffer_model_config: &core::Vector<u8>, buffer_weights: &core::Vector<u8>) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn read_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait NormalizeBBoxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void;

	#[inline]
	fn pnorm(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_getPropPnorm_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}
	
	#[inline]
	fn epsilon(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_getPropEpsilon_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}
	
	#[inline]
	fn across_spatial(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_getPropAcrossSpatial_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}
	
}

pub trait NormalizeBBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::NormalizeBBoxLayerTraitConst {
	fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_pnorm(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_setPropPnorm_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_epsilon(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_setPropEpsilon_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_across_spatial(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_setPropAcrossSpatial_bool(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}
	
}

pub struct NormalizeBBoxLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { NormalizeBBoxLayer }

impl Drop for NormalizeBBoxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_NormalizeBBoxLayer_delete(instance: *mut c_void); }
		unsafe { cv_NormalizeBBoxLayer_delete(self.as_raw_mut_NormalizeBBoxLayer()) };
	}
}

unsafe impl Send for NormalizeBBoxLayer {}

impl core::AlgorithmTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::NormalizeBBoxLayerTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::NormalizeBBoxLayerTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NormalizeBBoxLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::NormalizeBBoxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::NormalizeBBoxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { NormalizeBBoxLayer, core::Algorithm, cv_NormalizeBBoxLayer_to_Algorithm }

boxed_cast_base! { NormalizeBBoxLayer, crate::dnn::Layer, cv_NormalizeBBoxLayer_to_Layer }

pub trait NotLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_NotLayer(&self) -> *const c_void;

}

pub trait NotLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::NotLayerTraitConst {
	fn as_raw_mut_NotLayer(&mut self) -> *mut c_void;

}

pub struct NotLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { NotLayer }

impl Drop for NotLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_NotLayer_delete(instance: *mut c_void); }
		unsafe { cv_NotLayer_delete(self.as_raw_mut_NotLayer()) };
	}
}

unsafe impl Send for NotLayer {}

impl crate::dnn::ActivationLayerTraitConst for NotLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for NotLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for NotLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for NotLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for NotLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for NotLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::NotLayerTraitConst for NotLayer {
	#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::NotLayerTrait for NotLayer {
	#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl NotLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::NotLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NotLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::NotLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { NotLayer, crate::dnn::ActivationLayer, cv_NotLayer_to_ActivationLayer }

boxed_cast_base! { NotLayer, core::Algorithm, cv_NotLayer_to_Algorithm }

boxed_cast_base! { NotLayer, crate::dnn::Layer, cv_NotLayer_to_Layer }

pub trait PaddingLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PaddingLayer(&self) -> *const c_void;

}

pub trait PaddingLayerTrait: crate::dnn::LayerTrait + crate::dnn::PaddingLayerTraitConst {
	fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void;

}

pub struct PaddingLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { PaddingLayer }

impl Drop for PaddingLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PaddingLayer_delete(instance: *mut c_void); }
		unsafe { cv_PaddingLayer_delete(self.as_raw_mut_PaddingLayer()) };
	}
}

unsafe impl Send for PaddingLayer {}

impl core::AlgorithmTraitConst for PaddingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PaddingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PaddingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PaddingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PaddingLayerTraitConst for PaddingLayer {
	#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PaddingLayerTrait for PaddingLayer {
	#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PaddingLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PaddingLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PaddingLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PaddingLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PaddingLayer, core::Algorithm, cv_PaddingLayer_to_Algorithm }

boxed_cast_base! { PaddingLayer, crate::dnn::Layer, cv_PaddingLayer_to_Layer }

pub trait PermuteLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PermuteLayer(&self) -> *const c_void;

}

pub trait PermuteLayerTrait: crate::dnn::LayerTrait + crate::dnn::PermuteLayerTraitConst {
	fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void;

}

pub struct PermuteLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { PermuteLayer }

impl Drop for PermuteLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PermuteLayer_delete(instance: *mut c_void); }
		unsafe { cv_PermuteLayer_delete(self.as_raw_mut_PermuteLayer()) };
	}
}

unsafe impl Send for PermuteLayer {}

impl core::AlgorithmTraitConst for PermuteLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PermuteLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PermuteLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PermuteLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PermuteLayerTraitConst for PermuteLayer {
	#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PermuteLayerTrait for PermuteLayer {
	#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PermuteLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PermuteLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PermuteLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PermuteLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PermuteLayer, core::Algorithm, cv_PermuteLayer_to_Algorithm }

boxed_cast_base! { PermuteLayer, crate::dnn::Layer, cv_PermuteLayer_to_Layer }

pub trait PoolingLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PoolingLayer(&self) -> *const c_void;

	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropType_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn kernel_size(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropKernel_size_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn strides(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropStrides_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pads_begin(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropPads_begin_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn pads_end(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropPads_end_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn global_pooling(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropGlobalPooling_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn is_global_pooling(&self) -> core::Vector<bool> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropIsGlobalPooling_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<bool>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn compute_max_idx(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropComputeMaxIdx_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn pad_mode(&self) -> String {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropPadMode_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn ceil_mode(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropCeilMode_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn ave_pool_padded_area(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropAvePoolPaddedArea_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn pooled_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PoolingLayer_getPropPooledSize_const(self.as_raw_PoolingLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn spatial_scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropSpatialScale_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
	#[inline]
	fn ps_roi_out_channels(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_getPropPsRoiOutChannels_const(self.as_raw_PoolingLayer()) };
		ret
	}
	
}

pub trait PoolingLayerTrait: crate::dnn::LayerTrait + crate::dnn::PoolingLayerTraitConst {
	fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropType_int(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_kernel_size(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropKernel_size_vector_size_t_(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_strides(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropStrides_vector_size_t_(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_pads_begin(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropPads_begin_vector_size_t_(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_pads_end(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropPads_end_vector_size_t_(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
	#[inline]
	fn set_global_pooling(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropGlobalPooling_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_is_global_pooling(&mut self, mut val: core::Vector<bool>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropIsGlobalPooling_vector_bool_(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfbool()) };
		ret
	}
	
	#[inline]
	fn set_compute_max_idx(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropComputeMaxIdx_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_pad_mode(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropPadMode_String(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern_mut()) };
		ret
	}
	
	#[inline]
	fn set_ceil_mode(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropCeilMode_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_ave_pool_padded_area(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropAvePoolPaddedArea_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_pooled_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropPooledSize_Size(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_spatial_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropSpatialScale_float(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_ps_roi_out_channels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_setPropPsRoiOutChannels_int(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}
	
}

pub struct PoolingLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { PoolingLayer }

impl Drop for PoolingLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PoolingLayer_delete(instance: *mut c_void); }
		unsafe { cv_PoolingLayer_delete(self.as_raw_mut_PoolingLayer()) };
	}
}

unsafe impl Send for PoolingLayer {}

impl core::AlgorithmTraitConst for PoolingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PoolingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PoolingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PoolingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerTraitConst for PoolingLayer {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PoolingLayerTrait for PoolingLayer {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PoolingLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PoolingLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PoolingLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PoolingLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PoolingLayer, core::Algorithm, cv_PoolingLayer_to_Algorithm }

boxed_cast_base! { PoolingLayer, crate::dnn::Layer, cv_PoolingLayer_to_Layer }

pub trait PoolingLayerInt8TraitConst: crate::dnn::PoolingLayerTraitConst {
	fn as_raw_PoolingLayerInt8(&self) -> *const c_void;

	#[inline]
	fn input_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_getPropInput_zp_const(self.as_raw_PoolingLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_getPropOutput_zp_const(self.as_raw_PoolingLayerInt8()) };
		ret
	}
	
	#[inline]
	fn input_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_getPropInput_sc_const(self.as_raw_PoolingLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_getPropOutput_sc_const(self.as_raw_PoolingLayerInt8()) };
		ret
	}
	
}

pub trait PoolingLayerInt8Trait: crate::dnn::PoolingLayerInt8TraitConst + crate::dnn::PoolingLayerTrait {
	fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_input_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_setPropInput_zp_int(self.as_raw_mut_PoolingLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_setPropOutput_zp_int(self.as_raw_mut_PoolingLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_input_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_setPropInput_sc_float(self.as_raw_mut_PoolingLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_setPropOutput_sc_float(self.as_raw_mut_PoolingLayerInt8(), val) };
		ret
	}
	
}

pub struct PoolingLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { PoolingLayerInt8 }

impl Drop for PoolingLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_PoolingLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_PoolingLayerInt8_delete(self.as_raw_mut_PoolingLayerInt8()) };
	}
}

unsafe impl Send for PoolingLayerInt8 {}

impl core::AlgorithmTraitConst for PoolingLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PoolingLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PoolingLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PoolingLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerTraitConst for PoolingLayerInt8 {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PoolingLayerTrait for PoolingLayerInt8 {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PoolingLayerInt8TraitConst for PoolingLayerInt8 {
	#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PoolingLayerInt8Trait for PoolingLayerInt8 {
	#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PoolingLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PoolingLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PoolingLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PoolingLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PoolingLayerInt8, core::Algorithm, cv_PoolingLayerInt8_to_Algorithm }

boxed_cast_base! { PoolingLayerInt8, crate::dnn::Layer, cv_PoolingLayerInt8_to_Layer }

boxed_cast_base! { PoolingLayerInt8, crate::dnn::PoolingLayer, cv_PoolingLayerInt8_to_PoolingLayer }

pub trait PowerLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_PowerLayer(&self) -> *const c_void;

	#[inline]
	fn power(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_getPropPower_const(self.as_raw_PowerLayer()) };
		ret
	}
	
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_getPropScale_const(self.as_raw_PowerLayer()) };
		ret
	}
	
	#[inline]
	fn shift(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_getPropShift_const(self.as_raw_PowerLayer()) };
		ret
	}
	
}

pub trait PowerLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::PowerLayerTraitConst {
	fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_power(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_setPropPower_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_setPropScale_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_shift(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_setPropShift_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}
	
}

pub struct PowerLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { PowerLayer }

impl Drop for PowerLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PowerLayer_delete(instance: *mut c_void); }
		unsafe { cv_PowerLayer_delete(self.as_raw_mut_PowerLayer()) };
	}
}

unsafe impl Send for PowerLayer {}

impl crate::dnn::ActivationLayerTraitConst for PowerLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for PowerLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PowerLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PowerLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PowerLayerTraitConst for PowerLayer {
	#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PowerLayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PowerLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PowerLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PowerLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PowerLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PowerLayer, crate::dnn::ActivationLayer, cv_PowerLayer_to_ActivationLayer }

boxed_cast_base! { PowerLayer, core::Algorithm, cv_PowerLayer_to_Algorithm }

boxed_cast_base! { PowerLayer, crate::dnn::Layer, cv_PowerLayer_to_Layer }

pub trait PriorBoxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PriorBoxLayer(&self) -> *const c_void;

}

pub trait PriorBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::PriorBoxLayerTraitConst {
	fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void;

}

pub struct PriorBoxLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { PriorBoxLayer }

impl Drop for PriorBoxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PriorBoxLayer_delete(instance: *mut c_void); }
		unsafe { cv_PriorBoxLayer_delete(self.as_raw_mut_PriorBoxLayer()) };
	}
}

unsafe impl Send for PriorBoxLayer {}

impl core::AlgorithmTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PriorBoxLayerTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PriorBoxLayerTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PriorBoxLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PriorBoxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PriorBoxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PriorBoxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { PriorBoxLayer, core::Algorithm, cv_PriorBoxLayer_to_Algorithm }

boxed_cast_base! { PriorBoxLayer, crate::dnn::Layer, cv_PriorBoxLayer_to_Layer }

pub trait ProposalLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ProposalLayer(&self) -> *const c_void;

}

pub trait ProposalLayerTrait: crate::dnn::LayerTrait + crate::dnn::ProposalLayerTraitConst {
	fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void;

}

pub struct ProposalLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ProposalLayer }

impl Drop for ProposalLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ProposalLayer_delete(instance: *mut c_void); }
		unsafe { cv_ProposalLayer_delete(self.as_raw_mut_ProposalLayer()) };
	}
}

unsafe impl Send for ProposalLayer {}

impl core::AlgorithmTraitConst for ProposalLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ProposalLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ProposalLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ProposalLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ProposalLayerTraitConst for ProposalLayer {
	#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ProposalLayerTrait for ProposalLayer {
	#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ProposalLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ProposalLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ProposalLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ProposalLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ProposalLayer, core::Algorithm, cv_ProposalLayer_to_Algorithm }

boxed_cast_base! { ProposalLayer, crate::dnn::Layer, cv_ProposalLayer_to_Layer }

pub trait QuantizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_QuantizeLayer(&self) -> *const c_void;

	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_QuantizeLayer_getPropScale_const(self.as_raw_QuantizeLayer()) };
		ret
	}
	
	#[inline]
	fn zeropoint(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_QuantizeLayer_getPropZeropoint_const(self.as_raw_QuantizeLayer()) };
		ret
	}
	
}

pub trait QuantizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::QuantizeLayerTraitConst {
	fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_QuantizeLayer_setPropScale_float(self.as_raw_mut_QuantizeLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_zeropoint(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_QuantizeLayer_setPropZeropoint_int(self.as_raw_mut_QuantizeLayer(), val) };
		ret
	}
	
}

pub struct QuantizeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { QuantizeLayer }

impl Drop for QuantizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_QuantizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_QuantizeLayer_delete(self.as_raw_mut_QuantizeLayer()) };
	}
}

unsafe impl Send for QuantizeLayer {}

impl core::AlgorithmTraitConst for QuantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QuantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for QuantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for QuantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::QuantizeLayerTraitConst for QuantizeLayer {
	#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::QuantizeLayerTrait for QuantizeLayer {
	#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QuantizeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::QuantizeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_QuantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::QuantizeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { QuantizeLayer, core::Algorithm, cv_QuantizeLayer_to_Algorithm }

boxed_cast_base! { QuantizeLayer, crate::dnn::Layer, cv_QuantizeLayer_to_Layer }

pub trait RNNLayerConst: crate::dnn::LayerTraitConst {
	fn as_raw_RNNLayer(&self) -> *const c_void;

}

pub trait RNNLayer: crate::dnn::LayerTrait + crate::dnn::RNNLayerConst {
	fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_weights(&mut self, wxh: &core::Mat, bh: &core::Mat, whh: &core::Mat, who: &core::Mat, bo: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(self.as_raw_mut_RNNLayer(), wxh.as_raw_Mat(), bh.as_raw_Mat(), whh.as_raw_Mat(), who.as_raw_Mat(), bo.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * produce: false
	#[inline]
	fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_mut_RNNLayer(), produce, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn RNNLayer + '_ {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<dyn crate::dnn::RNNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::dnn::RNNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait ReLU6LayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ReLU6Layer(&self) -> *const c_void;

	#[inline]
	fn min_value(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_getPropMinValue_const(self.as_raw_ReLU6Layer()) };
		ret
	}
	
	#[inline]
	fn max_value(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_getPropMaxValue_const(self.as_raw_ReLU6Layer()) };
		ret
	}
	
}

pub trait ReLU6LayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLU6LayerTraitConst {
	fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void;

	#[inline]
	fn set_min_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_setPropMinValue_float(self.as_raw_mut_ReLU6Layer(), val) };
		ret
	}
	
	#[inline]
	fn set_max_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_setPropMaxValue_float(self.as_raw_mut_ReLU6Layer(), val) };
		ret
	}
	
}

pub struct ReLU6Layer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReLU6Layer }

impl Drop for ReLU6Layer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReLU6Layer_delete(instance: *mut c_void); }
		unsafe { cv_ReLU6Layer_delete(self.as_raw_mut_ReLU6Layer()) };
	}
}

unsafe impl Send for ReLU6Layer {}

impl crate::dnn::ActivationLayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ReLU6Layer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReLU6LayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReLU6LayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReLU6Layer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReLU6Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReLU6Layer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReLU6Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReLU6Layer, crate::dnn::ActivationLayer, cv_ReLU6Layer_to_ActivationLayer }

boxed_cast_base! { ReLU6Layer, core::Algorithm, cv_ReLU6Layer_to_Algorithm }

boxed_cast_base! { ReLU6Layer, crate::dnn::Layer, cv_ReLU6Layer_to_Layer }

pub trait ReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ReLULayer(&self) -> *const c_void;

	#[inline]
	fn negative_slope(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLULayer_getPropNegativeSlope_const(self.as_raw_ReLULayer()) };
		ret
	}
	
}

pub trait ReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLULayerTraitConst {
	fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_negative_slope(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLULayer_setPropNegativeSlope_float(self.as_raw_mut_ReLULayer(), val) };
		ret
	}
	
}

pub struct ReLULayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReLULayer }

impl Drop for ReLULayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReLULayer_delete(instance: *mut c_void); }
		unsafe { cv_ReLULayer_delete(self.as_raw_mut_ReLULayer()) };
	}
}

unsafe impl Send for ReLULayer {}

impl crate::dnn::ActivationLayerTraitConst for ReLULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ReLULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReLULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReLULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReLULayerTraitConst for ReLULayer {
	#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReLULayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReLULayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReLULayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReLULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReLULayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReLULayer, crate::dnn::ActivationLayer, cv_ReLULayer_to_ActivationLayer }

boxed_cast_base! { ReLULayer, core::Algorithm, cv_ReLULayer_to_Algorithm }

boxed_cast_base! { ReLULayer, crate::dnn::Layer, cv_ReLULayer_to_Layer }

pub trait ReciprocalLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ReciprocalLayer(&self) -> *const c_void;

}

pub trait ReciprocalLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReciprocalLayerTraitConst {
	fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void;

}

pub struct ReciprocalLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReciprocalLayer }

impl Drop for ReciprocalLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReciprocalLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReciprocalLayer_delete(self.as_raw_mut_ReciprocalLayer()) };
	}
}

unsafe impl Send for ReciprocalLayer {}

impl crate::dnn::ActivationLayerTraitConst for ReciprocalLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ReciprocalLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ReciprocalLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReciprocalLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReciprocalLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReciprocalLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReciprocalLayerTraitConst for ReciprocalLayer {
	#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReciprocalLayerTrait for ReciprocalLayer {
	#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReciprocalLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReciprocalLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReciprocalLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReciprocalLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReciprocalLayer, crate::dnn::ActivationLayer, cv_ReciprocalLayer_to_ActivationLayer }

boxed_cast_base! { ReciprocalLayer, core::Algorithm, cv_ReciprocalLayer_to_Algorithm }

boxed_cast_base! { ReciprocalLayer, crate::dnn::Layer, cv_ReciprocalLayer_to_Layer }

pub trait ReduceLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ReduceLayer(&self) -> *const c_void;

	#[inline]
	fn reduce_type(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ReduceLayer_getPropReduceType_const(self.as_raw_ReduceLayer()) };
		ret
	}
	
	#[inline]
	fn reduce_dims(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_ReduceLayer_getPropReduceDims_const(self.as_raw_ReduceLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait ReduceLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReduceLayerTraitConst {
	fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_reduce_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ReduceLayer_setPropReduceType_int(self.as_raw_mut_ReduceLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_reduce_dims(&mut self, mut val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_ReduceLayer_setPropReduceDims_vector_size_t_(self.as_raw_mut_ReduceLayer(), val.as_raw_mut_VectorOfsize_t()) };
		ret
	}
	
}

pub struct ReduceLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReduceLayer }

impl Drop for ReduceLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReduceLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReduceLayer_delete(self.as_raw_mut_ReduceLayer()) };
	}
}

unsafe impl Send for ReduceLayer {}

impl core::AlgorithmTraitConst for ReduceLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReduceLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReduceLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReduceLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReduceLayerTraitConst for ReduceLayer {
	#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReduceLayerTrait for ReduceLayer {
	#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReduceLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReduceLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReduceLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReduceLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReduceLayer, core::Algorithm, cv_ReduceLayer_to_Algorithm }

boxed_cast_base! { ReduceLayer, crate::dnn::Layer, cv_ReduceLayer_to_Layer }

pub trait ReduceLayerInt8TraitConst: crate::dnn::ReduceLayerTraitConst {
	fn as_raw_ReduceLayerInt8(&self) -> *const c_void;

}

pub trait ReduceLayerInt8Trait: crate::dnn::ReduceLayerInt8TraitConst + crate::dnn::ReduceLayerTrait {
	fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void;

}

pub struct ReduceLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { ReduceLayerInt8 }

impl Drop for ReduceLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_ReduceLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_ReduceLayerInt8_delete(self.as_raw_mut_ReduceLayerInt8()) };
	}
}

unsafe impl Send for ReduceLayerInt8 {}

impl core::AlgorithmTraitConst for ReduceLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReduceLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReduceLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReduceLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReduceLayerTraitConst for ReduceLayerInt8 {
	#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReduceLayerTrait for ReduceLayerInt8 {
	#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReduceLayerInt8TraitConst for ReduceLayerInt8 {
	#[inline] fn as_raw_ReduceLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReduceLayerInt8Trait for ReduceLayerInt8 {
	#[inline] fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReduceLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReduceLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReduceLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReduceLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReduceLayerInt8, core::Algorithm, cv_ReduceLayerInt8_to_Algorithm }

boxed_cast_base! { ReduceLayerInt8, crate::dnn::Layer, cv_ReduceLayerInt8_to_Layer }

boxed_cast_base! { ReduceLayerInt8, crate::dnn::ReduceLayer, cv_ReduceLayerInt8_to_ReduceLayer }

pub trait RegionLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_RegionLayer(&self) -> *const c_void;

	#[inline]
	fn nms_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_RegionLayer_getPropNmsThreshold_const(self.as_raw_RegionLayer()) };
		ret
	}
	
}

pub trait RegionLayerTrait: crate::dnn::LayerTrait + crate::dnn::RegionLayerTraitConst {
	fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_nms_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_RegionLayer_setPropNmsThreshold_float(self.as_raw_mut_RegionLayer(), val) };
		ret
	}
	
}

pub struct RegionLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { RegionLayer }

impl Drop for RegionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_RegionLayer_delete(instance: *mut c_void); }
		unsafe { cv_RegionLayer_delete(self.as_raw_mut_RegionLayer()) };
	}
}

unsafe impl Send for RegionLayer {}

impl core::AlgorithmTraitConst for RegionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RegionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for RegionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for RegionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RegionLayerTraitConst for RegionLayer {
	#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::RegionLayerTrait for RegionLayer {
	#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RegionLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RegionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RegionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::RegionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RegionLayer, core::Algorithm, cv_RegionLayer_to_Algorithm }

boxed_cast_base! { RegionLayer, crate::dnn::Layer, cv_RegionLayer_to_Layer }

pub trait ReorgLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ReorgLayer(&self) -> *const c_void;

}

pub trait ReorgLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReorgLayerTraitConst {
	fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void;

}

pub struct ReorgLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReorgLayer }

impl Drop for ReorgLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReorgLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReorgLayer_delete(self.as_raw_mut_ReorgLayer()) };
	}
}

unsafe impl Send for ReorgLayer {}

impl core::AlgorithmTraitConst for ReorgLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReorgLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReorgLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReorgLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReorgLayerTraitConst for ReorgLayer {
	#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReorgLayerTrait for ReorgLayer {
	#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReorgLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReorgLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReorgLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReorgLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReorgLayer, core::Algorithm, cv_ReorgLayer_to_Algorithm }

boxed_cast_base! { ReorgLayer, crate::dnn::Layer, cv_ReorgLayer_to_Layer }

pub trait RequantizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_RequantizeLayer(&self) -> *const c_void;

	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_RequantizeLayer_getPropScale_const(self.as_raw_RequantizeLayer()) };
		ret
	}
	
	#[inline]
	fn shift(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_RequantizeLayer_getPropShift_const(self.as_raw_RequantizeLayer()) };
		ret
	}
	
}

pub trait RequantizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::RequantizeLayerTraitConst {
	fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_RequantizeLayer_setPropScale_float(self.as_raw_mut_RequantizeLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_shift(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_RequantizeLayer_setPropShift_float(self.as_raw_mut_RequantizeLayer(), val) };
		ret
	}
	
}

pub struct RequantizeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { RequantizeLayer }

impl Drop for RequantizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_RequantizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_RequantizeLayer_delete(self.as_raw_mut_RequantizeLayer()) };
	}
}

unsafe impl Send for RequantizeLayer {}

impl core::AlgorithmTraitConst for RequantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RequantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for RequantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for RequantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RequantizeLayerTraitConst for RequantizeLayer {
	#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::RequantizeLayerTrait for RequantizeLayer {
	#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RequantizeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RequantizeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RequantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::RequantizeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RequantizeLayer, core::Algorithm, cv_RequantizeLayer_to_Algorithm }

boxed_cast_base! { RequantizeLayer, crate::dnn::Layer, cv_RequantizeLayer_to_Layer }

pub trait ReshapeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ReshapeLayer(&self) -> *const c_void;

	#[inline]
	fn new_shape_desc(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_getPropNewShapeDesc_const(self.as_raw_ReshapeLayer()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn new_shape_range(&self) -> core::Range {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_getPropNewShapeRange_const(self.as_raw_ReshapeLayer()) };
		let ret = unsafe { core::Range::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait ReshapeLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReshapeLayerTraitConst {
	fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_new_shape_desc(&mut self, mut val: crate::dnn::MatShape) {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_setPropNewShapeDesc_MatShape(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_VectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_new_shape_range(&mut self, mut val: core::Range) {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_setPropNewShapeRange_Range(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_Range()) };
		ret
	}
	
}

pub struct ReshapeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ReshapeLayer }

impl Drop for ReshapeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReshapeLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReshapeLayer_delete(self.as_raw_mut_ReshapeLayer()) };
	}
}

unsafe impl Send for ReshapeLayer {}

impl core::AlgorithmTraitConst for ReshapeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ReshapeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReshapeLayerTraitConst for ReshapeLayer {
	#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReshapeLayerTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ReshapeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReshapeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReshapeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReshapeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ReshapeLayer, core::Algorithm, cv_ReshapeLayer_to_Algorithm }

boxed_cast_base! { ReshapeLayer, crate::dnn::Layer, cv_ReshapeLayer_to_Layer }

pub trait ResizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ResizeLayer(&self) -> *const c_void;

}

pub trait ResizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::ResizeLayerTraitConst {
	fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void;

}

pub struct ResizeLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ResizeLayer }

impl Drop for ResizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ResizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_ResizeLayer_delete(self.as_raw_mut_ResizeLayer()) };
	}
}

unsafe impl Send for ResizeLayer {}

impl core::AlgorithmTraitConst for ResizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ResizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ResizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ResizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ResizeLayerTraitConst for ResizeLayer {
	#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ResizeLayerTrait for ResizeLayer {
	#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ResizeLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ResizeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ResizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ResizeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ResizeLayer, core::Algorithm, cv_ResizeLayer_to_Algorithm }

boxed_cast_base! { ResizeLayer, crate::dnn::Layer, cv_ResizeLayer_to_Layer }

pub trait RoundLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_RoundLayer(&self) -> *const c_void;

}

pub trait RoundLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::RoundLayerTraitConst {
	fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void;

}

pub struct RoundLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { RoundLayer }

impl Drop for RoundLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_RoundLayer_delete(instance: *mut c_void); }
		unsafe { cv_RoundLayer_delete(self.as_raw_mut_RoundLayer()) };
	}
}

unsafe impl Send for RoundLayer {}

impl crate::dnn::ActivationLayerTraitConst for RoundLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for RoundLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for RoundLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RoundLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for RoundLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for RoundLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RoundLayerTraitConst for RoundLayer {
	#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::RoundLayerTrait for RoundLayer {
	#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RoundLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RoundLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RoundLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::RoundLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { RoundLayer, crate::dnn::ActivationLayer, cv_RoundLayer_to_ActivationLayer }

boxed_cast_base! { RoundLayer, core::Algorithm, cv_RoundLayer_to_Algorithm }

boxed_cast_base! { RoundLayer, crate::dnn::Layer, cv_RoundLayer_to_Layer }

pub trait ScaleLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ScaleLayer(&self) -> *const c_void;

	#[inline]
	fn has_bias(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_getPropHasBias_const(self.as_raw_ScaleLayer()) };
		ret
	}
	
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_getPropAxis_const(self.as_raw_ScaleLayer()) };
		ret
	}
	
	#[inline]
	fn mode(&self) -> String {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_getPropMode_const(self.as_raw_ScaleLayer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait ScaleLayerTrait: crate::dnn::LayerTrait + crate::dnn::ScaleLayerTraitConst {
	fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_has_bias(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_setPropHasBias_bool(self.as_raw_mut_ScaleLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_setPropAxis_int(self.as_raw_mut_ScaleLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_mode(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_dnn_ScaleLayer_setPropMode_String(self.as_raw_mut_ScaleLayer(), val.opencv_as_extern_mut()) };
		ret
	}
	
}

pub struct ScaleLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ScaleLayer }

impl Drop for ScaleLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ScaleLayer_delete(instance: *mut c_void); }
		unsafe { cv_ScaleLayer_delete(self.as_raw_mut_ScaleLayer()) };
	}
}

unsafe impl Send for ScaleLayer {}

impl core::AlgorithmTraitConst for ScaleLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ScaleLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ScaleLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ScaleLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerTraitConst for ScaleLayer {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ScaleLayerTrait for ScaleLayer {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ScaleLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScaleLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ScaleLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ScaleLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ScaleLayer, core::Algorithm, cv_ScaleLayer_to_Algorithm }

boxed_cast_base! { ScaleLayer, crate::dnn::Layer, cv_ScaleLayer_to_Layer }

pub trait ScaleLayerInt8TraitConst: crate::dnn::ScaleLayerTraitConst {
	fn as_raw_ScaleLayerInt8(&self) -> *const c_void;

	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_getPropOutput_sc_const(self.as_raw_ScaleLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_getPropOutput_zp_const(self.as_raw_ScaleLayerInt8()) };
		ret
	}
	
}

pub trait ScaleLayerInt8Trait: crate::dnn::ScaleLayerInt8TraitConst + crate::dnn::ScaleLayerTrait {
	fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_setPropOutput_sc_float(self.as_raw_mut_ScaleLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_setPropOutput_zp_int(self.as_raw_mut_ScaleLayerInt8(), val) };
		ret
	}
	
}

pub struct ScaleLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { ScaleLayerInt8 }

impl Drop for ScaleLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_ScaleLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_ScaleLayerInt8_delete(self.as_raw_mut_ScaleLayerInt8()) };
	}
}

unsafe impl Send for ScaleLayerInt8 {}

impl core::AlgorithmTraitConst for ScaleLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ScaleLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ScaleLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ScaleLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerTraitConst for ScaleLayerInt8 {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ScaleLayerTrait for ScaleLayerInt8 {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerInt8TraitConst for ScaleLayerInt8 {
	#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ScaleLayerInt8Trait for ScaleLayerInt8 {
	#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ScaleLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScaleLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ScaleLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ScaleLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ScaleLayerInt8, core::Algorithm, cv_ScaleLayerInt8_to_Algorithm }

boxed_cast_base! { ScaleLayerInt8, crate::dnn::Layer, cv_ScaleLayerInt8_to_Layer }

boxed_cast_base! { ScaleLayerInt8, crate::dnn::ScaleLayer, cv_ScaleLayerInt8_to_ScaleLayer }

pub trait SegmentationModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_SegmentationModel(&self) -> *const c_void;

}

pub trait SegmentationModelTrait: crate::dnn::ModelTrait + crate::dnn::SegmentationModelTraitConst {
	fn as_raw_mut_SegmentationModel(&mut self) -> *mut c_void;

	#[inline]
	fn segment(&mut self, frame: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(frame);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SegmentationModel_segment_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SegmentationModel(), frame.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct SegmentationModel {
	ptr: *mut c_void
}

opencv_type_boxed! { SegmentationModel }

impl Drop for SegmentationModel {
	fn drop(&mut self) {
		extern "C" { fn cv_SegmentationModel_delete(instance: *mut c_void); }
		unsafe { cv_SegmentationModel_delete(self.as_raw_mut_SegmentationModel()) };
	}
}

unsafe impl Send for SegmentationModel {}

impl crate::dnn::ModelTraitConst for SegmentationModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for SegmentationModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SegmentationModelTraitConst for SegmentationModel {
	#[inline] fn as_raw_SegmentationModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SegmentationModelTrait for SegmentationModel {
	#[inline] fn as_raw_mut_SegmentationModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SegmentationModel {
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new(model: &str, config: &str) -> Result<crate::dnn::SegmentationModel> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SegmentationModel_SegmentationModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::SegmentationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::SegmentationModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SegmentationModel_SegmentationModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::SegmentationModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SegmentationModel, crate::dnn::Model, cv_SegmentationModel_to_Model }

pub trait SeluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SeluLayer(&self) -> *const c_void;

	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_SeluLayer_getPropAlpha_const(self.as_raw_SeluLayer()) };
		ret
	}
	
	#[inline]
	fn gamma(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_SeluLayer_getPropGamma_const(self.as_raw_SeluLayer()) };
		ret
	}
	
}

pub trait SeluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SeluLayerTraitConst {
	fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_SeluLayer_setPropAlpha_float(self.as_raw_mut_SeluLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_gamma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_SeluLayer_setPropGamma_float(self.as_raw_mut_SeluLayer(), val) };
		ret
	}
	
}

pub struct SeluLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SeluLayer }

impl Drop for SeluLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SeluLayer_delete(instance: *mut c_void); }
		unsafe { cv_SeluLayer_delete(self.as_raw_mut_SeluLayer()) };
	}
}

unsafe impl Send for SeluLayer {}

impl crate::dnn::ActivationLayerTraitConst for SeluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SeluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SeluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SeluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SeluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SeluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SeluLayerTraitConst for SeluLayer {
	#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SeluLayerTrait for SeluLayer {
	#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SeluLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SeluLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SeluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SeluLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SeluLayer, crate::dnn::ActivationLayer, cv_SeluLayer_to_ActivationLayer }

boxed_cast_base! { SeluLayer, core::Algorithm, cv_SeluLayer_to_Algorithm }

boxed_cast_base! { SeluLayer, crate::dnn::Layer, cv_SeluLayer_to_Layer }

pub trait ShiftLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ShiftLayer(&self) -> *const c_void;

}

pub trait ShiftLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShiftLayerTraitConst {
	fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void;

}

pub struct ShiftLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ShiftLayer }

impl Drop for ShiftLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ShiftLayer_delete(instance: *mut c_void); }
		unsafe { cv_ShiftLayer_delete(self.as_raw_mut_ShiftLayer()) };
	}
}

unsafe impl Send for ShiftLayer {}

impl core::AlgorithmTraitConst for ShiftLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShiftLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ShiftLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShiftLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ShiftLayerTraitConst for ShiftLayer {
	#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShiftLayerTrait for ShiftLayer {
	#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ShiftLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShiftLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ShiftLayer, core::Algorithm, cv_ShiftLayer_to_Algorithm }

boxed_cast_base! { ShiftLayer, crate::dnn::Layer, cv_ShiftLayer_to_Layer }

pub trait ShiftLayerInt8TraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ShiftLayerInt8(&self) -> *const c_void;

}

pub trait ShiftLayerInt8Trait: crate::dnn::LayerTrait + crate::dnn::ShiftLayerInt8TraitConst {
	fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void;

}

pub struct ShiftLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { ShiftLayerInt8 }

impl Drop for ShiftLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_ShiftLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_ShiftLayerInt8_delete(self.as_raw_mut_ShiftLayerInt8()) };
	}
}

unsafe impl Send for ShiftLayerInt8 {}

impl core::AlgorithmTraitConst for ShiftLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShiftLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ShiftLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShiftLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ShiftLayerInt8TraitConst for ShiftLayerInt8 {
	#[inline] fn as_raw_ShiftLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShiftLayerInt8Trait for ShiftLayerInt8 {
	#[inline] fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ShiftLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShiftLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ShiftLayerInt8, core::Algorithm, cv_ShiftLayerInt8_to_Algorithm }

boxed_cast_base! { ShiftLayerInt8, crate::dnn::Layer, cv_ShiftLayerInt8_to_Layer }

pub trait ShrinkLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ShrinkLayer(&self) -> *const c_void;

	#[inline]
	fn bias(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ShrinkLayer_getPropBias_const(self.as_raw_ShrinkLayer()) };
		ret
	}
	
	#[inline]
	fn lambd(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ShrinkLayer_getPropLambd_const(self.as_raw_ShrinkLayer()) };
		ret
	}
	
}

pub trait ShrinkLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ShrinkLayerTraitConst {
	fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_bias(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ShrinkLayer_setPropBias_float(self.as_raw_mut_ShrinkLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_lambd(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ShrinkLayer_setPropLambd_float(self.as_raw_mut_ShrinkLayer(), val) };
		ret
	}
	
}

pub struct ShrinkLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ShrinkLayer }

impl Drop for ShrinkLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ShrinkLayer_delete(instance: *mut c_void); }
		unsafe { cv_ShrinkLayer_delete(self.as_raw_mut_ShrinkLayer()) };
	}
}

unsafe impl Send for ShrinkLayer {}

impl crate::dnn::ActivationLayerTraitConst for ShrinkLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ShrinkLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ShrinkLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShrinkLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ShrinkLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShrinkLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ShrinkLayerTraitConst for ShrinkLayer {
	#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShrinkLayerTrait for ShrinkLayer {
	#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ShrinkLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ShrinkLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShrinkLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ShrinkLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ShrinkLayer, crate::dnn::ActivationLayer, cv_ShrinkLayer_to_ActivationLayer }

boxed_cast_base! { ShrinkLayer, core::Algorithm, cv_ShrinkLayer_to_Algorithm }

boxed_cast_base! { ShrinkLayer, crate::dnn::Layer, cv_ShrinkLayer_to_Layer }

pub trait ShuffleChannelLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ShuffleChannelLayer(&self) -> *const c_void;

	#[inline]
	fn group(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_getPropGroup_const(self.as_raw_ShuffleChannelLayer()) };
		ret
	}
	
}

pub trait ShuffleChannelLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShuffleChannelLayerTraitConst {
	fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_group(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_setPropGroup_int(self.as_raw_mut_ShuffleChannelLayer(), val) };
		ret
	}
	
}

pub struct ShuffleChannelLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ShuffleChannelLayer }

impl Drop for ShuffleChannelLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ShuffleChannelLayer_delete(instance: *mut c_void); }
		unsafe { cv_ShuffleChannelLayer_delete(self.as_raw_mut_ShuffleChannelLayer()) };
	}
}

unsafe impl Send for ShuffleChannelLayer {}

impl core::AlgorithmTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ShuffleChannelLayerTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShuffleChannelLayerTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ShuffleChannelLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ShuffleChannelLayer, core::Algorithm, cv_ShuffleChannelLayer_to_Algorithm }

boxed_cast_base! { ShuffleChannelLayer, crate::dnn::Layer, cv_ShuffleChannelLayer_to_Layer }

pub trait SigmoidLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SigmoidLayer(&self) -> *const c_void;

}

pub trait SigmoidLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SigmoidLayerTraitConst {
	fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void;

}

pub struct SigmoidLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SigmoidLayer }

impl Drop for SigmoidLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SigmoidLayer_delete(instance: *mut c_void); }
		unsafe { cv_SigmoidLayer_delete(self.as_raw_mut_SigmoidLayer()) };
	}
}

unsafe impl Send for SigmoidLayer {}

impl crate::dnn::ActivationLayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SigmoidLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SigmoidLayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SigmoidLayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SigmoidLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SigmoidLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SigmoidLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SigmoidLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SigmoidLayer, crate::dnn::ActivationLayer, cv_SigmoidLayer_to_ActivationLayer }

boxed_cast_base! { SigmoidLayer, core::Algorithm, cv_SigmoidLayer_to_Algorithm }

boxed_cast_base! { SigmoidLayer, crate::dnn::Layer, cv_SigmoidLayer_to_Layer }

pub trait SignLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SignLayer(&self) -> *const c_void;

}

pub trait SignLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SignLayerTraitConst {
	fn as_raw_mut_SignLayer(&mut self) -> *mut c_void;

}

pub struct SignLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SignLayer }

impl Drop for SignLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SignLayer_delete(instance: *mut c_void); }
		unsafe { cv_SignLayer_delete(self.as_raw_mut_SignLayer()) };
	}
}

unsafe impl Send for SignLayer {}

impl crate::dnn::ActivationLayerTraitConst for SignLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SignLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SignLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SignLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SignLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SignLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SignLayerTraitConst for SignLayer {
	#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SignLayerTrait for SignLayer {
	#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SignLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SignLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SignLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SignLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SignLayer, crate::dnn::ActivationLayer, cv_SignLayer_to_ActivationLayer }

boxed_cast_base! { SignLayer, core::Algorithm, cv_SignLayer_to_Algorithm }

boxed_cast_base! { SignLayer, crate::dnn::Layer, cv_SignLayer_to_Layer }

pub trait SinLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SinLayer(&self) -> *const c_void;

}

pub trait SinLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SinLayerTraitConst {
	fn as_raw_mut_SinLayer(&mut self) -> *mut c_void;

}

pub struct SinLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SinLayer }

impl Drop for SinLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SinLayer_delete(instance: *mut c_void); }
		unsafe { cv_SinLayer_delete(self.as_raw_mut_SinLayer()) };
	}
}

unsafe impl Send for SinLayer {}

impl crate::dnn::ActivationLayerTraitConst for SinLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SinLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SinLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SinLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SinLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SinLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SinLayerTraitConst for SinLayer {
	#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SinLayerTrait for SinLayer {
	#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SinLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SinLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SinLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SinLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SinLayer, crate::dnn::ActivationLayer, cv_SinLayer_to_ActivationLayer }

boxed_cast_base! { SinLayer, core::Algorithm, cv_SinLayer_to_Algorithm }

boxed_cast_base! { SinLayer, crate::dnn::Layer, cv_SinLayer_to_Layer }

pub trait SinhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SinhLayer(&self) -> *const c_void;

}

pub trait SinhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SinhLayerTraitConst {
	fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void;

}

pub struct SinhLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SinhLayer }

impl Drop for SinhLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SinhLayer_delete(instance: *mut c_void); }
		unsafe { cv_SinhLayer_delete(self.as_raw_mut_SinhLayer()) };
	}
}

unsafe impl Send for SinhLayer {}

impl crate::dnn::ActivationLayerTraitConst for SinhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SinhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SinhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SinhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SinhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SinhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SinhLayerTraitConst for SinhLayer {
	#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SinhLayerTrait for SinhLayer {
	#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SinhLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SinhLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SinhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SinhLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SinhLayer, crate::dnn::ActivationLayer, cv_SinhLayer_to_ActivationLayer }

boxed_cast_base! { SinhLayer, core::Algorithm, cv_SinhLayer_to_Algorithm }

boxed_cast_base! { SinhLayer, crate::dnn::Layer, cv_SinhLayer_to_Layer }

pub trait SliceLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SliceLayer(&self) -> *const c_void;

	#[inline]
	fn slice_ranges(&self) -> core::Vector<core::Vector<core::Range>> {
		let ret = unsafe { sys::cv_dnn_SliceLayer_getPropSliceRanges_const(self.as_raw_SliceLayer()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Range>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn slice_steps(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_dnn_SliceLayer_getPropSliceSteps_const(self.as_raw_SliceLayer()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}
	
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SliceLayer_getPropAxis_const(self.as_raw_SliceLayer()) };
		ret
	}
	
	#[inline]
	fn num_split(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SliceLayer_getPropNum_split_const(self.as_raw_SliceLayer()) };
		ret
	}
	
}

pub trait SliceLayerTrait: crate::dnn::LayerTrait + crate::dnn::SliceLayerTraitConst {
	fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_slice_ranges(&mut self, mut val: core::Vector<core::Vector<core::Range>>) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_setPropSliceRanges_vector_vector_Range__(self.as_raw_mut_SliceLayer(), val.as_raw_mut_VectorOfVectorOfRange()) };
		ret
	}
	
	#[inline]
	fn set_slice_steps(&mut self, mut val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_setPropSliceSteps_vector_vector_int__(self.as_raw_mut_SliceLayer(), val.as_raw_mut_VectorOfVectorOfi32()) };
		ret
	}
	
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_setPropAxis_int(self.as_raw_mut_SliceLayer(), val) };
		ret
	}
	
	#[inline]
	fn set_num_split(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_setPropNum_split_int(self.as_raw_mut_SliceLayer(), val) };
		ret
	}
	
}

pub struct SliceLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SliceLayer }

impl Drop for SliceLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SliceLayer_delete(instance: *mut c_void); }
		unsafe { cv_SliceLayer_delete(self.as_raw_mut_SliceLayer()) };
	}
}

unsafe impl Send for SliceLayer {}

impl core::AlgorithmTraitConst for SliceLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SliceLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SliceLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SliceLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SliceLayerTraitConst for SliceLayer {
	#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SliceLayerTrait for SliceLayer {
	#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SliceLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SliceLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SliceLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SliceLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SliceLayer, core::Algorithm, cv_SliceLayer_to_Algorithm }

boxed_cast_base! { SliceLayer, crate::dnn::Layer, cv_SliceLayer_to_Layer }

pub trait SoftmaxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SoftmaxLayer(&self) -> *const c_void;

	#[inline]
	fn log_soft_max(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayer_getPropLogSoftMax_const(self.as_raw_SoftmaxLayer()) };
		ret
	}
	
}

pub trait SoftmaxLayerTrait: crate::dnn::LayerTrait + crate::dnn::SoftmaxLayerTraitConst {
	fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_log_soft_max(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayer_setPropLogSoftMax_bool(self.as_raw_mut_SoftmaxLayer(), val) };
		ret
	}
	
}

pub struct SoftmaxLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SoftmaxLayer }

impl Drop for SoftmaxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SoftmaxLayer_delete(instance: *mut c_void); }
		unsafe { cv_SoftmaxLayer_delete(self.as_raw_mut_SoftmaxLayer()) };
	}
}

unsafe impl Send for SoftmaxLayer {}

impl core::AlgorithmTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SoftmaxLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftmaxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SoftmaxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SoftmaxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SoftmaxLayer, core::Algorithm, cv_SoftmaxLayer_to_Algorithm }

boxed_cast_base! { SoftmaxLayer, crate::dnn::Layer, cv_SoftmaxLayer_to_Layer }

pub trait SoftmaxLayerInt8TraitConst: crate::dnn::SoftmaxLayerTraitConst {
	fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void;

	#[inline]
	fn output_sc(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_getPropOutput_sc_const(self.as_raw_SoftmaxLayerInt8()) };
		ret
	}
	
	#[inline]
	fn output_zp(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_getPropOutput_zp_const(self.as_raw_SoftmaxLayerInt8()) };
		ret
	}
	
}

pub trait SoftmaxLayerInt8Trait: crate::dnn::SoftmaxLayerInt8TraitConst + crate::dnn::SoftmaxLayerTrait {
	fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void;

	#[inline]
	fn set_output_sc(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_setPropOutput_sc_float(self.as_raw_mut_SoftmaxLayerInt8(), val) };
		ret
	}
	
	#[inline]
	fn set_output_zp(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_setPropOutput_zp_int(self.as_raw_mut_SoftmaxLayerInt8(), val) };
		ret
	}
	
}

pub struct SoftmaxLayerInt8 {
	ptr: *mut c_void
}

opencv_type_boxed! { SoftmaxLayerInt8 }

impl Drop for SoftmaxLayerInt8 {
	fn drop(&mut self) {
		extern "C" { fn cv_SoftmaxLayerInt8_delete(instance: *mut c_void); }
		unsafe { cv_SoftmaxLayerInt8_delete(self.as_raw_mut_SoftmaxLayerInt8()) };
	}
}

unsafe impl Send for SoftmaxLayerInt8 {}

impl core::AlgorithmTraitConst for SoftmaxLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SoftmaxLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerTraitConst for SoftmaxLayerInt8 {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftmaxLayerInt8TraitConst for SoftmaxLayerInt8 {
	#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftmaxLayerInt8Trait for SoftmaxLayerInt8 {
	#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SoftmaxLayerInt8 {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftmaxLayerInt8>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SoftmaxLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SoftmaxLayerInt8>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SoftmaxLayerInt8, core::Algorithm, cv_SoftmaxLayerInt8_to_Algorithm }

boxed_cast_base! { SoftmaxLayerInt8, crate::dnn::Layer, cv_SoftmaxLayerInt8_to_Layer }

boxed_cast_base! { SoftmaxLayerInt8, crate::dnn::SoftmaxLayer, cv_SoftmaxLayerInt8_to_SoftmaxLayer }

pub trait SoftplusLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SoftplusLayer(&self) -> *const c_void;

}

pub trait SoftplusLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SoftplusLayerTraitConst {
	fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void;

}

pub struct SoftplusLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SoftplusLayer }

impl Drop for SoftplusLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SoftplusLayer_delete(instance: *mut c_void); }
		unsafe { cv_SoftplusLayer_delete(self.as_raw_mut_SoftplusLayer()) };
	}
}

unsafe impl Send for SoftplusLayer {}

impl crate::dnn::ActivationLayerTraitConst for SoftplusLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SoftplusLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SoftplusLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SoftplusLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SoftplusLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SoftplusLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftplusLayerTraitConst for SoftplusLayer {
	#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftplusLayerTrait for SoftplusLayer {
	#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SoftplusLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftplusLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SoftplusLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SoftplusLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SoftplusLayer, crate::dnn::ActivationLayer, cv_SoftplusLayer_to_ActivationLayer }

boxed_cast_base! { SoftplusLayer, core::Algorithm, cv_SoftplusLayer_to_Algorithm }

boxed_cast_base! { SoftplusLayer, crate::dnn::Layer, cv_SoftplusLayer_to_Layer }

pub trait SoftsignLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SoftsignLayer(&self) -> *const c_void;

}

pub trait SoftsignLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SoftsignLayerTraitConst {
	fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void;

}

pub struct SoftsignLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SoftsignLayer }

impl Drop for SoftsignLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SoftsignLayer_delete(instance: *mut c_void); }
		unsafe { cv_SoftsignLayer_delete(self.as_raw_mut_SoftsignLayer()) };
	}
}

unsafe impl Send for SoftsignLayer {}

impl crate::dnn::ActivationLayerTraitConst for SoftsignLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SoftsignLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SoftsignLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SoftsignLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SoftsignLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SoftsignLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SoftsignLayerTraitConst for SoftsignLayer {
	#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftsignLayerTrait for SoftsignLayer {
	#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SoftsignLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftsignLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SoftsignLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SoftsignLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SoftsignLayer, crate::dnn::ActivationLayer, cv_SoftsignLayer_to_ActivationLayer }

boxed_cast_base! { SoftsignLayer, core::Algorithm, cv_SoftsignLayer_to_Algorithm }

boxed_cast_base! { SoftsignLayer, crate::dnn::Layer, cv_SoftsignLayer_to_Layer }

pub trait SplitLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SplitLayer(&self) -> *const c_void;

	#[inline]
	fn outputs_count(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SplitLayer_getPropOutputsCount_const(self.as_raw_SplitLayer()) };
		ret
	}
	
}

pub trait SplitLayerTrait: crate::dnn::LayerTrait + crate::dnn::SplitLayerTraitConst {
	fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_outputs_count(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SplitLayer_setPropOutputsCount_int(self.as_raw_mut_SplitLayer(), val) };
		ret
	}
	
}

pub struct SplitLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SplitLayer }

impl Drop for SplitLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SplitLayer_delete(instance: *mut c_void); }
		unsafe { cv_SplitLayer_delete(self.as_raw_mut_SplitLayer()) };
	}
}

unsafe impl Send for SplitLayer {}

impl core::AlgorithmTraitConst for SplitLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SplitLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SplitLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SplitLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SplitLayerTraitConst for SplitLayer {
	#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SplitLayerTrait for SplitLayer {
	#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SplitLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SplitLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SplitLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SplitLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SplitLayer, core::Algorithm, cv_SplitLayer_to_Algorithm }

boxed_cast_base! { SplitLayer, crate::dnn::Layer, cv_SplitLayer_to_Layer }

pub trait SqrtLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SqrtLayer(&self) -> *const c_void;

}

pub trait SqrtLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SqrtLayerTraitConst {
	fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void;

}

pub struct SqrtLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SqrtLayer }

impl Drop for SqrtLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SqrtLayer_delete(instance: *mut c_void); }
		unsafe { cv_SqrtLayer_delete(self.as_raw_mut_SqrtLayer()) };
	}
}

unsafe impl Send for SqrtLayer {}

impl crate::dnn::ActivationLayerTraitConst for SqrtLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SqrtLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SqrtLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SqrtLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SqrtLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SqrtLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SqrtLayerTraitConst for SqrtLayer {
	#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SqrtLayerTrait for SqrtLayer {
	#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SqrtLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SqrtLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SqrtLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SqrtLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SqrtLayer, crate::dnn::ActivationLayer, cv_SqrtLayer_to_ActivationLayer }

boxed_cast_base! { SqrtLayer, core::Algorithm, cv_SqrtLayer_to_Algorithm }

boxed_cast_base! { SqrtLayer, crate::dnn::Layer, cv_SqrtLayer_to_Layer }

pub trait SwishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SwishLayer(&self) -> *const c_void;

}

pub trait SwishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SwishLayerTraitConst {
	fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void;

}

pub struct SwishLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { SwishLayer }

impl Drop for SwishLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SwishLayer_delete(instance: *mut c_void); }
		unsafe { cv_SwishLayer_delete(self.as_raw_mut_SwishLayer()) };
	}
}

unsafe impl Send for SwishLayer {}

impl crate::dnn::ActivationLayerTraitConst for SwishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for SwishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SwishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for SwishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SwishLayerTraitConst for SwishLayer {
	#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SwishLayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SwishLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SwishLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SwishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SwishLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SwishLayer, crate::dnn::ActivationLayer, cv_SwishLayer_to_ActivationLayer }

boxed_cast_base! { SwishLayer, core::Algorithm, cv_SwishLayer_to_Algorithm }

boxed_cast_base! { SwishLayer, crate::dnn::Layer, cv_SwishLayer_to_Layer }

pub trait TanHLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_TanHLayer(&self) -> *const c_void;

}

pub trait TanHLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::TanHLayerTraitConst {
	fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void;

}

pub struct TanHLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { TanHLayer }

impl Drop for TanHLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_TanHLayer_delete(instance: *mut c_void); }
		unsafe { cv_TanHLayer_delete(self.as_raw_mut_TanHLayer()) };
	}
}

unsafe impl Send for TanHLayer {}

impl crate::dnn::ActivationLayerTraitConst for TanHLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for TanHLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TanHLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for TanHLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TanHLayerTraitConst for TanHLayer {
	#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TanHLayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TanHLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::TanHLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TanHLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::TanHLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { TanHLayer, crate::dnn::ActivationLayer, cv_TanHLayer_to_ActivationLayer }

boxed_cast_base! { TanHLayer, core::Algorithm, cv_TanHLayer_to_Algorithm }

boxed_cast_base! { TanHLayer, crate::dnn::Layer, cv_TanHLayer_to_Layer }

pub trait TanLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_TanLayer(&self) -> *const c_void;

}

pub trait TanLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::TanLayerTraitConst {
	fn as_raw_mut_TanLayer(&mut self) -> *mut c_void;

}

pub struct TanLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { TanLayer }

impl Drop for TanLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_TanLayer_delete(instance: *mut c_void); }
		unsafe { cv_TanLayer_delete(self.as_raw_mut_TanLayer()) };
	}
}

unsafe impl Send for TanLayer {}

impl crate::dnn::ActivationLayerTraitConst for TanLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for TanLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for TanLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TanLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for TanLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for TanLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TanLayerTraitConst for TanLayer {
	#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TanLayerTrait for TanLayer {
	#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TanLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::TanLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TanLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::TanLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { TanLayer, crate::dnn::ActivationLayer, cv_TanLayer_to_ActivationLayer }

boxed_cast_base! { TanLayer, core::Algorithm, cv_TanLayer_to_Algorithm }

boxed_cast_base! { TanLayer, crate::dnn::Layer, cv_TanLayer_to_Layer }

pub trait TextDetectionModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_TextDetectionModel(&self) -> *const c_void;

	#[inline]
	fn detect_with_confidences(&self, frame: &dyn core::ToInputArray, detections: &mut core::Vector<core::Vector<core::Point>>, confidences: &mut core::Vector<f32>) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vector_vector_Point__R_vector_float_R(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfVectorOfPoint(), confidences.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn detect(&self, frame: &dyn core::ToInputArray, detections: &mut core::Vector<core::Vector<core::Point>>) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vector_vector_Point__R(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn detect_text_rectangles(&self, frame: &dyn core::ToInputArray, detections: &mut core::Vector<core::RotatedRect>, confidences: &mut core::Vector<f32>) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vector_RotatedRect_R_vector_float_R(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfRotatedRect(), confidences.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn detect_text_rectangles_1(&self, frame: &dyn core::ToInputArray, detections: &mut core::Vector<core::RotatedRect>) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vector_RotatedRect_R(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfRotatedRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TextDetectionModelTrait: crate::dnn::ModelTrait + crate::dnn::TextDetectionModelTraitConst {
	fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void;

}

pub struct TextDetectionModel {
	ptr: *mut c_void
}

opencv_type_boxed! { TextDetectionModel }

impl Drop for TextDetectionModel {
	fn drop(&mut self) {
		extern "C" { fn cv_TextDetectionModel_delete(instance: *mut c_void); }
		unsafe { cv_TextDetectionModel_delete(self.as_raw_mut_TextDetectionModel()) };
	}
}

unsafe impl Send for TextDetectionModel {}

impl crate::dnn::ModelTraitConst for TextDetectionModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for TextDetectionModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel {
	#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextDetectionModelTrait for TextDetectionModel {
	#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TextDetectionModel {
}

boxed_cast_base! { TextDetectionModel, crate::dnn::Model, cv_TextDetectionModel_to_Model }

pub trait TextDetectionModel_DBTraitConst: crate::dnn::TextDetectionModelTraitConst {
	fn as_raw_TextDetectionModel_DB(&self) -> *const c_void;

	#[inline]
	fn get_binary_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_getBinaryThreshold_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_polygon_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_getPolygonThreshold_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_unclip_ratio(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_getUnclipRatio_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_candidates(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_getMaxCandidates_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TextDetectionModel_DBTrait: crate::dnn::TextDetectionModelTrait + crate::dnn::TextDetectionModel_DBTraitConst {
	fn as_raw_mut_TextDetectionModel_DB(&mut self) -> *mut c_void;

	#[inline]
	fn set_binary_threshold(&mut self, binary_threshold: f32) -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_setBinaryThreshold_float(self.as_raw_mut_TextDetectionModel_DB(), binary_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_polygon_threshold(&mut self, polygon_threshold: f32) -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_setPolygonThreshold_float(self.as_raw_mut_TextDetectionModel_DB(), polygon_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_unclip_ratio(&mut self, unclip_ratio: f64) -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_setUnclipRatio_double(self.as_raw_mut_TextDetectionModel_DB(), unclip_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_max_candidates(&mut self, max_candidates: i32) -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_setMaxCandidates_int(self.as_raw_mut_TextDetectionModel_DB(), max_candidates, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct TextDetectionModel_DB {
	ptr: *mut c_void
}

opencv_type_boxed! { TextDetectionModel_DB }

impl Drop for TextDetectionModel_DB {
	fn drop(&mut self) {
		extern "C" { fn cv_TextDetectionModel_DB_delete(instance: *mut c_void); }
		unsafe { cv_TextDetectionModel_DB_delete(self.as_raw_mut_TextDetectionModel_DB()) };
	}
}

unsafe impl Send for TextDetectionModel_DB {}

impl crate::dnn::ModelTraitConst for TextDetectionModel_DB {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for TextDetectionModel_DB {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel_DB {
	#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextDetectionModelTrait for TextDetectionModel_DB {
	#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextDetectionModel_DBTraitConst for TextDetectionModel_DB {
	#[inline] fn as_raw_TextDetectionModel_DB(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextDetectionModel_DBTrait for TextDetectionModel_DB {
	#[inline] fn as_raw_mut_TextDetectionModel_DB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TextDetectionModel_DB {
	#[inline]
	pub fn default() -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextDetectionModel_DB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn new_1(model: &str, config: &str) -> Result<crate::dnn::TextDetectionModel_DB> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { TextDetectionModel_DB, crate::dnn::Model, cv_TextDetectionModel_DB_to_Model }

boxed_cast_base! { TextDetectionModel_DB, crate::dnn::TextDetectionModel, cv_TextDetectionModel_DB_to_TextDetectionModel }

pub trait TextDetectionModel_EASTTraitConst: crate::dnn::TextDetectionModelTraitConst {
	fn as_raw_TextDetectionModel_EAST(&self) -> *const c_void;

	#[inline]
	fn get_confidence_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_getConfidenceThreshold_const(self.as_raw_TextDetectionModel_EAST(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nms_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_getNMSThreshold_const(self.as_raw_TextDetectionModel_EAST(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TextDetectionModel_EASTTrait: crate::dnn::TextDetectionModelTrait + crate::dnn::TextDetectionModel_EASTTraitConst {
	fn as_raw_mut_TextDetectionModel_EAST(&mut self) -> *mut c_void;

	#[inline]
	fn set_confidence_threshold(&mut self, conf_threshold: f32) -> Result<crate::dnn::TextDetectionModel_EAST> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_setConfidenceThreshold_float(self.as_raw_mut_TextDetectionModel_EAST(), conf_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_nms_threshold(&mut self, nms_threshold: f32) -> Result<crate::dnn::TextDetectionModel_EAST> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_setNMSThreshold_float(self.as_raw_mut_TextDetectionModel_EAST(), nms_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct TextDetectionModel_EAST {
	ptr: *mut c_void
}

opencv_type_boxed! { TextDetectionModel_EAST }

impl Drop for TextDetectionModel_EAST {
	fn drop(&mut self) {
		extern "C" { fn cv_TextDetectionModel_EAST_delete(instance: *mut c_void); }
		unsafe { cv_TextDetectionModel_EAST_delete(self.as_raw_mut_TextDetectionModel_EAST()) };
	}
}

unsafe impl Send for TextDetectionModel_EAST {}

impl crate::dnn::ModelTraitConst for TextDetectionModel_EAST {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for TextDetectionModel_EAST {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel_EAST {
	#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextDetectionModelTrait for TextDetectionModel_EAST {
	#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextDetectionModel_EASTTraitConst for TextDetectionModel_EAST {
	#[inline] fn as_raw_TextDetectionModel_EAST(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextDetectionModel_EASTTrait for TextDetectionModel_EAST {
	#[inline] fn as_raw_mut_TextDetectionModel_EAST(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TextDetectionModel_EAST {
	#[inline]
	pub fn default() -> Result<crate::dnn::TextDetectionModel_EAST> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextDetectionModel_EAST> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn from_file(model: &str, config: &str) -> Result<crate::dnn::TextDetectionModel_EAST> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { TextDetectionModel_EAST, crate::dnn::Model, cv_TextDetectionModel_EAST_to_Model }

boxed_cast_base! { TextDetectionModel_EAST, crate::dnn::TextDetectionModel, cv_TextDetectionModel_EAST_to_TextDetectionModel }

pub trait TextRecognitionModelTraitConst: crate::dnn::ModelTraitConst {
	fn as_raw_TextRecognitionModel(&self) -> *const c_void;

	#[inline]
	fn get_decode_type(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_getDecodeType_const(self.as_raw_TextRecognitionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_vocabulary(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_getVocabulary_const(self.as_raw_TextRecognitionModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn recognize(&self, frame: &dyn core::ToInputArray) -> Result<String> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR(self.as_raw_TextRecognitionModel(), frame.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn recognize_1(&self, frame: &dyn core::ToInputArray, roi_rects: &dyn core::ToInputArray, results: &mut core::Vector<String>) -> Result<()> {
		input_array_arg!(frame);
		input_array_arg!(roi_rects);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR_const__InputArrayR_vector_string_R(self.as_raw_TextRecognitionModel(), frame.as_raw__InputArray(), roi_rects.as_raw__InputArray(), results.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TextRecognitionModelTrait: crate::dnn::ModelTrait + crate::dnn::TextRecognitionModelTraitConst {
	fn as_raw_mut_TextRecognitionModel(&mut self) -> *mut c_void;

	#[inline]
	fn set_decode_type(&mut self, decode_type: &str) -> Result<crate::dnn::TextRecognitionModel> {
		extern_container_arg!(decode_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_setDecodeType_const_stringR(self.as_raw_mut_TextRecognitionModel(), decode_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * voc_prune_size: 0
	#[inline]
	fn set_decode_opts_ctc_prefix_beam_search(&mut self, beam_size: i32, voc_prune_size: i32) -> Result<crate::dnn::TextRecognitionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int_int(self.as_raw_mut_TextRecognitionModel(), beam_size, voc_prune_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_vocabulary(&mut self, vocabulary: &core::Vector<String>) -> Result<crate::dnn::TextRecognitionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_setVocabulary_const_vector_string_R(self.as_raw_mut_TextRecognitionModel(), vocabulary.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub struct TextRecognitionModel {
	ptr: *mut c_void
}

opencv_type_boxed! { TextRecognitionModel }

impl Drop for TextRecognitionModel {
	fn drop(&mut self) {
		extern "C" { fn cv_TextRecognitionModel_delete(instance: *mut c_void); }
		unsafe { cv_TextRecognitionModel_delete(self.as_raw_mut_TextRecognitionModel()) };
	}
}

unsafe impl Send for TextRecognitionModel {}

impl crate::dnn::ModelTraitConst for TextRecognitionModel {
	#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ModelTrait for TextRecognitionModel {
	#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TextRecognitionModelTraitConst for TextRecognitionModel {
	#[inline] fn as_raw_TextRecognitionModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TextRecognitionModelTrait for TextRecognitionModel {
	#[inline] fn as_raw_mut_TextRecognitionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TextRecognitionModel {
	#[inline]
	pub fn default() -> Result<crate::dnn::TextRecognitionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextRecognitionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * config: ""
	#[inline]
	pub fn from_file(model: &str, config: &str) -> Result<crate::dnn::TextRecognitionModel> {
		extern_container_arg!(model);
		extern_container_arg!(config);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { TextRecognitionModel, crate::dnn::Model, cv_TextRecognitionModel_to_Model }

pub trait ThresholdedReluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ThresholdedReluLayer(&self) -> *const c_void;

	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ThresholdedReluLayer_getPropAlpha_const(self.as_raw_ThresholdedReluLayer()) };
		ret
	}
	
}

pub trait ThresholdedReluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ThresholdedReluLayerTraitConst {
	fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void;

	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ThresholdedReluLayer_setPropAlpha_float(self.as_raw_mut_ThresholdedReluLayer(), val) };
		ret
	}
	
}

pub struct ThresholdedReluLayer {
	ptr: *mut c_void
}

opencv_type_boxed! { ThresholdedReluLayer }

impl Drop for ThresholdedReluLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ThresholdedReluLayer_delete(instance: *mut c_void); }
		unsafe { cv_ThresholdedReluLayer_delete(self.as_raw_mut_ThresholdedReluLayer()) };
	}
}

unsafe impl Send for ThresholdedReluLayer {}

impl crate::dnn::ActivationLayerTraitConst for ThresholdedReluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ThresholdedReluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::AlgorithmTraitConst for ThresholdedReluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ThresholdedReluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for ThresholdedReluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ThresholdedReluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ThresholdedReluLayerTraitConst for ThresholdedReluLayer {
	#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ThresholdedReluLayerTrait for ThresholdedReluLayer {
	#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ThresholdedReluLayer {
	#[inline]
	pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ThresholdedReluLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ThresholdedReluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ThresholdedReluLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ThresholdedReluLayer, crate::dnn::ActivationLayer, cv_ThresholdedReluLayer_to_ActivationLayer }

boxed_cast_base! { ThresholdedReluLayer, core::Algorithm, cv_ThresholdedReluLayer_to_Algorithm }

boxed_cast_base! { ThresholdedReluLayer, crate::dnn::Layer, cv_ThresholdedReluLayer_to_Layer }

pub trait _RangeTraitConst: core::RangeTraitConst {
	fn as_raw__Range(&self) -> *const c_void;

}

pub trait _RangeTrait: core::RangeTrait + crate::dnn::_RangeTraitConst {
	fn as_raw_mut__Range(&mut self) -> *mut c_void;

}

pub struct _Range {
	ptr: *mut c_void
}

opencv_type_boxed! { _Range }

impl Drop for _Range {
	fn drop(&mut self) {
		extern "C" { fn cv__Range_delete(instance: *mut c_void); }
		unsafe { cv__Range_delete(self.as_raw_mut__Range()) };
	}
}

unsafe impl Send for _Range {}

impl core::RangeTraitConst for _Range {
	#[inline] fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
}

impl core::RangeTrait for _Range {
	#[inline] fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::_RangeTraitConst for _Range {
	#[inline] fn as_raw__Range(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::_RangeTrait for _Range {
	#[inline] fn as_raw_mut__Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl _Range {
	#[inline]
	pub fn new(r: &core::Range) -> Result<crate::dnn::_Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn__Range__Range_const_RangeR(r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * size_: 1
	#[inline]
	pub fn new_1(start_: i32, size_: i32) -> Result<crate::dnn::_Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn__Range__Range_int_int(start_, size_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { _Range, core::Range, cv__Range_to_Range }
pub use crate::manual::dnn::*;
