extern "C" {
	// NMSBoxes(const std::vector<Rect2d> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int) /usr/include/opencv2/dnn/dnn.hpp:1168
	pub fn cv_dnn_NMSBoxes_const_vector_Rect2d_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result_void);
	// NMSBoxes(const std::vector<Rect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int) /usr/include/opencv2/dnn/dnn.hpp:1163
	pub fn cv_dnn_NMSBoxes_const_vector_Rect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result_void);
	// NMSBoxes(const std::vector<RotatedRect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int) /usr/include/opencv2/dnn/dnn.hpp:1173
	pub fn cv_dnn_NMSBoxes_const_vector_RotatedRect_R_const_vector_float_R_const_float_const_float_vector_int_R_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result_void);
	// blobFromImage(cv::InputArray, cv::OutputArray, double, const cv::Size &, const cv::Scalar &, bool, bool, int) /usr/include/opencv2/dnn/dnn.hpp:1084
	pub fn cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image: *const c_void, blob: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result_void);
	// blobFromImage(cv::InputArray, double, const cv::Size &, const cv::Scalar &, bool, bool, int) /usr/include/opencv2/dnn/dnn.hpp:1076
	pub fn cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// blobFromImages(cv::InputArrayOfArrays, cv::OutputArray, double, cv::Size, const cv::Scalar &, bool, bool, int) /usr/include/opencv2/dnn/dnn.hpp:1114
	pub fn cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(images: *const c_void, blob: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result_void);
	// blobFromImages(cv::InputArrayOfArrays, double, cv::Size, const cv::Scalar &, bool, bool, int) /usr/include/opencv2/dnn/dnn.hpp:1106
	pub fn cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(images: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// concat(const cv::dnn::MatShape &, const cv::dnn::MatShape &) /usr/include/opencv2/dnn/shape_utils.hpp:179
	pub fn cv_dnn_concat_const_MatShapeR_const_MatShapeR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// enableModelDiagnostics(bool) /usr/include/opencv2/dnn/dnn.hpp:116
	pub fn cv_dnn_enableModelDiagnostics_bool(is_diagnostics_mode: bool, ocvrs_return: *mut Result_void);
	// getAvailableTargets(dnn::Backend) /usr/include/opencv2/dnn/dnn.hpp:104
	pub fn cv_dnn_getAvailableTargets_Backend(be: crate::dnn::Backend, ocvrs_return: *mut Result<*mut c_void>);
	// getInferenceEngineBackendType() /usr/include/opencv2/dnn/utils/inference_engine.hpp:31
	pub fn cv_dnn_getInferenceEngineBackendType(ocvrs_return: *mut Result<*mut c_void>);
	// getInferenceEngineCPUType() /usr/include/opencv2/dnn/utils/inference_engine.hpp:72
	pub fn cv_dnn_getInferenceEngineCPUType(ocvrs_return: *mut Result<*mut c_void>);
	// getInferenceEngineVPUType() /usr/include/opencv2/dnn/utils/inference_engine.hpp:66
	pub fn cv_dnn_getInferenceEngineVPUType(ocvrs_return: *mut Result<*mut c_void>);
	// getPlane(const cv::Mat &, int, int) /usr/include/opencv2/dnn/shape_utils.hpp:108
	pub fn cv_dnn_getPlane_const_MatR_int_int(m: *const c_void, n: i32, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
	// imagesFromBlob(const cv::Mat &, cv::OutputArrayOfArrays) /usr/include/opencv2/dnn/dnn.hpp:1127
	pub fn cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(blob_: *const c_void, images_: *const c_void, ocvrs_return: *mut Result_void);
	// readNetFromCaffe(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:877
	pub fn cv_dnn_readNetFromCaffe_const_StringR_const_StringR(prototxt: *const c_char, caffe_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromCaffe(const char *, size_t, const char *, size_t) /usr/include/opencv2/dnn/dnn.hpp:896
	pub fn cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(buffer_proto: *const c_char, len_proto: size_t, buffer_model: *const c_char, len_model: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromCaffe(const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:884
	pub fn cv_dnn_readNetFromCaffe_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_proto: *const c_void, buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromDarknet(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:852
	pub fn cv_dnn_readNetFromDarknet_const_StringR_const_StringR(cfg_file: *const c_char, darknet_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromDarknet(const char *, size_t, const char *, size_t) /usr/include/opencv2/dnn/dnn.hpp:869
	pub fn cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(buffer_cfg: *const c_char, len_cfg: size_t, buffer_model: *const c_char, len_model: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromDarknet(const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:859
	pub fn cv_dnn_readNetFromDarknet_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_cfg: *const c_void, buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromModelOptimizer(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1006
	pub fn cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(xml: *const c_char, bin: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromModelOptimizer(const uchar *, size_t, const uchar *, size_t) /usr/include/opencv2/dnn/dnn.hpp:1028
	pub fn cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr: *const u8, buffer_model_config_size: size_t, buffer_weights_ptr: *const u8, buffer_weights_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:1016
	pub fn cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model_config: *const c_void, buffer_weights: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromONNX(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1035
	pub fn cv_dnn_readNetFromONNX_const_StringR(onnx_file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromONNX(const char *, size_t) /usr/include/opencv2/dnn/dnn.hpp:1044
	pub fn cv_dnn_readNetFromONNX_const_charX_size_t(buffer: *const c_char, size_buffer: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromONNX(const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:1052
	pub fn cv_dnn_readNetFromONNX_const_vector_unsigned_char_R(buffer: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromTensorflow(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:906
	pub fn cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromTensorflow(const char *, size_t, const char *, size_t) /usr/include/opencv2/dnn/dnn.hpp:924
	pub fn cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(buffer_model: *const c_char, len_model: size_t, buffer_config: *const c_char, len_config: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromTensorflow(const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:913
	pub fn cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readNetFromTorch(const cv::String &, bool, bool) /usr/include/opencv2/dnn/dnn.hpp:953
	pub fn cv_dnn_readNetFromTorch_const_StringR_bool_bool(model: *const c_char, is_binary: bool, evaluate: bool, ocvrs_return: *mut Result<*mut c_void>);
	// readNet(const cv::String &, const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:979
	pub fn cv_dnn_readNet_const_StringR_const_StringR_const_StringR(model: *const c_char, config: *const c_char, framework: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readNet(const cv::String &, const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:990
	pub fn cv_dnn_readNet_const_StringR_const_vector_unsigned_char_R_const_vector_unsigned_char_R(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readTensorFromONNX(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1058
	pub fn cv_dnn_readTensorFromONNX_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readTorchBlob(const cv::String &, bool) /usr/include/opencv2/dnn/dnn.hpp:996
	pub fn cv_dnn_readTorchBlob_const_StringR_bool(filename: *const c_char, is_binary: bool, ocvrs_return: *mut Result<*mut c_void>);
	// releaseHDDLPlugin() /usr/include/opencv2/dnn/utils/inference_engine.hpp:76
	pub fn cv_dnn_releaseHDDLPlugin(ocvrs_return: *mut Result_void);
	// resetMyriadDevice() /usr/include/opencv2/dnn/utils/inference_engine.hpp:49
	pub fn cv_dnn_resetMyriadDevice(ocvrs_return: *mut Result_void);
	// setInferenceEngineBackendType(const cv::String &) /usr/include/opencv2/dnn/utils/inference_engine.hpp:41
	pub fn cv_dnn_setInferenceEngineBackendType_const_StringR(new_backend_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// shape(const cv::Mat &) /usr/include/opencv2/dnn/shape_utils.hpp:126
	pub fn cv_dnn_shape_const_MatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// shape(const cv::MatSize &) /usr/include/opencv2/dnn/shape_utils.hpp:131
	pub fn cv_dnn_shape_const_MatSizeR(sz: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// shape(const cv::UMat &) /usr/include/opencv2/dnn/shape_utils.hpp:136
	pub fn cv_dnn_shape_const_UMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// shape(const int *, const int) /usr/include/opencv2/dnn/shape_utils.hpp:119
	pub fn cv_dnn_shape_const_intX_const_int(dims: *const i32, n: i32, ocvrs_return: *mut Result<*mut c_void>);
	// shape(int, int, int, int) /usr/include/opencv2/dnn/shape_utils.hpp:153
	pub fn cv_dnn_shape_int_int_int_int(a0: i32, a1: i32, a2: i32, a3: i32, ocvrs_return: *mut Result<*mut c_void>);
	// shrinkCaffeModel(const cv::String &, const cv::String &, const std::vector<String> &) /usr/include/opencv2/dnn/dnn.hpp:1142
	pub fn cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vector_String_R(src: *const c_char, dst: *const c_char, layers_types: *const c_void, ocvrs_return: *mut Result_void);
	// slice(const cv::Mat &, const cv::dnn::_Range &) /usr/include/opencv2/dnn/shape_utils.hpp:63
	pub fn cv_dnn_slice_const_MatR_const__RangeR(m: *const c_void, r0: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// slice(const cv::Mat &, const cv::dnn::_Range &, const cv::dnn::_Range &) /usr/include/opencv2/dnn/shape_utils.hpp:72
	pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// slice(const cv::Mat &, const cv::dnn::_Range &, const cv::dnn::_Range &, const cv::dnn::_Range &) /usr/include/opencv2/dnn/shape_utils.hpp:83
	pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, r2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// slice(const cv::Mat &, const cv::dnn::_Range &, const cv::dnn::_Range &, const cv::dnn::_Range &, const cv::dnn::_Range &) /usr/include/opencv2/dnn/shape_utils.hpp:95
	pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, r2: *const c_void, r3: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// softNMSBoxes(const std::vector<Rect> &, const std::vector<float> &, std::vector<float> &, const float, const float, std::vector<int> &, size_t, const float, cv::dnn::SoftNMSMethod) /usr/include/opencv2/dnn/dnn.hpp:1201
	pub fn cv_dnn_softNMSBoxes_const_vector_Rect_R_const_vector_float_R_vector_float_R_const_float_const_float_vector_int_R_size_t_const_float_SoftNMSMethod(bboxes: *const c_void, scores: *const c_void, updated_scores: *mut c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, top_k: size_t, sigma: f32, method: crate::dnn::SoftNMSMethod, ocvrs_return: *mut Result_void);
	// total(const cv::dnn::MatShape &, int, int) /usr/include/opencv2/dnn/shape_utils.hpp:161
	pub fn cv_dnn_total_const_MatShapeR_int_int(shape: *const c_void, start: i32, end: i32, ocvrs_return: *mut Result<i32>);
	// writeTextGraph(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1151
	pub fn cv_dnn_writeTextGraph_const_StringR_const_StringR(model: *const c_char, output: *const c_char, ocvrs_return: *mut Result_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:612
	pub fn cv_dnn_AbsLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:922
	pub fn cv_dnn_AccumLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:670
	pub fn cv_dnn_AcosLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:676
	pub fn cv_dnn_AcoshLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// forwardSlice(const float *, float *, int, size_t, int, int) /usr/include/opencv2/dnn/all_layers.hpp:541
	pub fn cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(instance: *const c_void, src: *const f32, dst: *mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32, ocvrs_return: *mut Result_void);
	// forwardSlice(const int *, const int *, int *, int, size_t, int, int) /usr/include/opencv2/dnn/all_layers.hpp:543
	pub fn cv_dnn_ActivationLayer_forwardSlice_const_const_intX_const_intX_intX_int_size_t_int_int(instance: *const c_void, src: *const i32, lut: *const i32, dst: *mut i32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32, ocvrs_return: *mut Result_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:794
	pub fn cv_dnn_ActivationLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:294
	pub fn cv_dnn_ArgLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:682
	pub fn cv_dnn_AsinLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:688
	pub fn cv_dnn_AsinhLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:694
	pub fn cv_dnn_AtanLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:700
	pub fn cv_dnn_AtanhLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:606
	pub fn cv_dnn_BNLLLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// backendId /usr/include/opencv2/dnn/dnn.hpp:143
	pub fn cv_dnn_BackendNode_getPropBackendId_const(instance: *const c_void) -> i32;
	// backendId /usr/include/opencv2/dnn/dnn.hpp:143
	pub fn cv_dnn_BackendNode_setPropBackendId_int(instance: *mut c_void, val: i32);
	// backendId /usr/include/opencv2/dnn/dnn.hpp:187
	pub fn cv_dnn_BackendWrapper_getPropBackendId_const(instance: *const c_void) -> i32;
	// backendId /usr/include/opencv2/dnn/dnn.hpp:187
	pub fn cv_dnn_BackendWrapper_setPropBackendId_int(instance: *mut c_void, val: i32);
	// targetId /usr/include/opencv2/dnn/dnn.hpp:188
	pub fn cv_dnn_BackendWrapper_getPropTargetId_const(instance: *const c_void) -> i32;
	// targetId /usr/include/opencv2/dnn/dnn.hpp:188
	pub fn cv_dnn_BackendWrapper_setPropTargetId_int(instance: *mut c_void, val: i32);
	// copyToHost() /usr/include/opencv2/dnn/dnn.hpp:180
	pub fn cv_dnn_BackendWrapper_copyToHost(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setHostDirty() /usr/include/opencv2/dnn/dnn.hpp:185
	pub fn cv_dnn_BackendWrapper_setHostDirty(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// kernel /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_getPropKernel_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// kernel /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_setPropKernel_Size(instance: *mut c_void, val: *const core::Size);
	// stride /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_getPropStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// stride /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_setPropStride_Size(instance: *mut c_void, val: *const core::Size);
	// pad /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_getPropPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// pad /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_setPropPad_Size(instance: *mut c_void, val: *const core::Size);
	// dilation /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_getPropDilation_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// dilation /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_setPropDilation_Size(instance: *mut c_void, val: *const core::Size);
	// adjustPad /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_getPropAdjustPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// adjustPad /usr/include/opencv2/dnn/all_layers.hpp:247
	pub fn cv_dnn_BaseConvolutionLayer_setPropAdjustPad_Size(instance: *mut c_void, val: *const core::Size);
	// adjust_pads /usr/include/opencv2/dnn/all_layers.hpp:248
	pub fn cv_dnn_BaseConvolutionLayer_getPropAdjust_pads_const(instance: *const c_void) -> *mut c_void;
	// adjust_pads /usr/include/opencv2/dnn/all_layers.hpp:248
	pub fn cv_dnn_BaseConvolutionLayer_setPropAdjust_pads_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// kernel_size /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_getPropKernel_size_const(instance: *const c_void) -> *mut c_void;
	// kernel_size /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_setPropKernel_size_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// strides /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_getPropStrides_const(instance: *const c_void) -> *mut c_void;
	// strides /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_setPropStrides_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// dilations /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_getPropDilations_const(instance: *const c_void) -> *mut c_void;
	// dilations /usr/include/opencv2/dnn/all_layers.hpp:249
	pub fn cv_dnn_BaseConvolutionLayer_setPropDilations_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// pads_begin /usr/include/opencv2/dnn/all_layers.hpp:250
	pub fn cv_dnn_BaseConvolutionLayer_getPropPads_begin_const(instance: *const c_void) -> *mut c_void;
	// pads_begin /usr/include/opencv2/dnn/all_layers.hpp:250
	pub fn cv_dnn_BaseConvolutionLayer_setPropPads_begin_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// pads_end /usr/include/opencv2/dnn/all_layers.hpp:250
	pub fn cv_dnn_BaseConvolutionLayer_getPropPads_end_const(instance: *const c_void) -> *mut c_void;
	// pads_end /usr/include/opencv2/dnn/all_layers.hpp:250
	pub fn cv_dnn_BaseConvolutionLayer_setPropPads_end_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// padMode /usr/include/opencv2/dnn/all_layers.hpp:251
	pub fn cv_dnn_BaseConvolutionLayer_getPropPadMode_const(instance: *const c_void) -> *mut c_void;
	// padMode /usr/include/opencv2/dnn/all_layers.hpp:251
	pub fn cv_dnn_BaseConvolutionLayer_setPropPadMode_String(instance: *mut c_void, val: *mut c_char);
	// numOutput /usr/include/opencv2/dnn/all_layers.hpp:252
	pub fn cv_dnn_BaseConvolutionLayer_getPropNumOutput_const(instance: *const c_void) -> i32;
	// numOutput /usr/include/opencv2/dnn/all_layers.hpp:252
	pub fn cv_dnn_BaseConvolutionLayer_setPropNumOutput_int(instance: *mut c_void, val: i32);
	// hasWeights /usr/include/opencv2/dnn/all_layers.hpp:847
	pub fn cv_dnn_BatchNormLayer_getPropHasWeights_const(instance: *const c_void) -> bool;
	// hasWeights /usr/include/opencv2/dnn/all_layers.hpp:847
	pub fn cv_dnn_BatchNormLayer_setPropHasWeights_bool(instance: *mut c_void, val: bool);
	// hasBias /usr/include/opencv2/dnn/all_layers.hpp:847
	pub fn cv_dnn_BatchNormLayer_getPropHasBias_const(instance: *const c_void) -> bool;
	// hasBias /usr/include/opencv2/dnn/all_layers.hpp:847
	pub fn cv_dnn_BatchNormLayer_setPropHasBias_bool(instance: *mut c_void, val: bool);
	// epsilon /usr/include/opencv2/dnn/all_layers.hpp:848
	pub fn cv_dnn_BatchNormLayer_getPropEpsilon_const(instance: *const c_void) -> f32;
	// epsilon /usr/include/opencv2/dnn/all_layers.hpp:848
	pub fn cv_dnn_BatchNormLayer_setPropEpsilon_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:850
	pub fn cv_dnn_BatchNormLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:856
	pub fn cv_dnn_BatchNormLayerInt8_getPropInput_sc_const(instance: *const c_void) -> f32;
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:856
	pub fn cv_dnn_BatchNormLayerInt8_setPropInput_sc_float(instance: *mut c_void, val: f32);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:856
	pub fn cv_dnn_BatchNormLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:856
	pub fn cv_dnn_BatchNormLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:857
	pub fn cv_dnn_BatchNormLayerInt8_getPropInput_zp_const(instance: *const c_void) -> i32;
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:857
	pub fn cv_dnn_BatchNormLayerInt8_setPropInput_zp_int(instance: *mut c_void, val: i32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:857
	pub fn cv_dnn_BatchNormLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:857
	pub fn cv_dnn_BatchNormLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:858
	pub fn cv_dnn_BatchNormLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:77
	pub fn cv_dnn_BlankLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:634
	pub fn cv_dnn_CeilLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:760
	pub fn cv_dnn_CeluLayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:760
	pub fn cv_dnn_CeluLayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:762
	pub fn cv_dnn_CeluLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:568
	pub fn cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ClassificationModel() /usr/include/opencv2/dnn/dnn.hpp:1329
	pub fn cv_dnn_ClassificationModel_ClassificationModel(ocvrs_return: *mut Result<*mut c_void>);
	// ClassificationModel(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1337
	pub fn cv_dnn_ClassificationModel_ClassificationModel_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// ClassificationModel(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1343
	pub fn cv_dnn_ClassificationModel_ClassificationModel_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setEnableSoftmaxPostProcessing(bool) /usr/include/opencv2/dnn/dnn.hpp:1354
	pub fn cv_dnn_ClassificationModel_setEnableSoftmaxPostProcessing_bool(instance: *mut c_void, enable: bool, ocvrs_return: *mut Result<*mut c_void>);
	// getEnableSoftmaxPostProcessing() /usr/include/opencv2/dnn/dnn.hpp:1361
	pub fn cv_dnn_ClassificationModel_getEnableSoftmaxPostProcessing_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// classify(cv::InputArray, int &, float &) /usr/include/opencv2/dnn/dnn.hpp:1369
	pub fn cv_dnn_ClassificationModel_classify_const__InputArrayR_intR_floatR(instance: *mut c_void, frame: *const c_void, class_id: *mut i32, conf: *mut f32, ocvrs_return: *mut Result_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:904
	pub fn cv_dnn_CompareLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// axis /usr/include/opencv2/dnn/all_layers.hpp:426
	pub fn cv_dnn_ConcatLayer_getPropAxis_const(instance: *const c_void) -> i32;
	// axis /usr/include/opencv2/dnn/all_layers.hpp:426
	pub fn cv_dnn_ConcatLayer_setPropAxis_int(instance: *mut c_void, val: i32);
	// padding /usr/include/opencv2/dnn/all_layers.hpp:433
	pub fn cv_dnn_ConcatLayer_getPropPadding_const(instance: *const c_void) -> bool;
	// padding /usr/include/opencv2/dnn/all_layers.hpp:433
	pub fn cv_dnn_ConcatLayer_setPropPadding_bool(instance: *mut c_void, val: bool);
	// paddingValue /usr/include/opencv2/dnn/all_layers.hpp:434
	pub fn cv_dnn_ConcatLayer_getPropPaddingValue_const(instance: *const c_void) -> i32;
	// paddingValue /usr/include/opencv2/dnn/all_layers.hpp:434
	pub fn cv_dnn_ConcatLayer_setPropPaddingValue_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:436
	pub fn cv_dnn_ConcatLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:86
	pub fn cv_dnn_ConstLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:258
	pub fn cv_dnn_ConvolutionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:264
	pub fn cv_dnn_ConvolutionLayerInt8_getPropInput_zp_const(instance: *const c_void) -> i32;
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:264
	pub fn cv_dnn_ConvolutionLayerInt8_setPropInput_zp_int(instance: *mut c_void, val: i32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:264
	pub fn cv_dnn_ConvolutionLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:264
	pub fn cv_dnn_ConvolutionLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:265
	pub fn cv_dnn_ConvolutionLayerInt8_getPropInput_sc_const(instance: *const c_void) -> f32;
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:265
	pub fn cv_dnn_ConvolutionLayerInt8_setPropInput_sc_float(instance: *mut c_void, val: f32);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:265
	pub fn cv_dnn_ConvolutionLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:265
	pub fn cv_dnn_ConvolutionLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:266
	pub fn cv_dnn_ConvolutionLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:916
	pub fn cv_dnn_CorrelationLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:706
	pub fn cv_dnn_CosLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:712
	pub fn cv_dnn_CoshLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:1030
	pub fn cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:822
	pub fn cv_dnn_CropLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// exclusive /usr/include/opencv2/dnn/all_layers.hpp:1036
	pub fn cv_dnn_CumSumLayer_getPropExclusive_const(instance: *const c_void) -> i32;
	// exclusive /usr/include/opencv2/dnn/all_layers.hpp:1036
	pub fn cv_dnn_CumSumLayer_setPropExclusive_int(instance: *mut c_void, val: i32);
	// reverse /usr/include/opencv2/dnn/all_layers.hpp:1037
	pub fn cv_dnn_CumSumLayer_getPropReverse_const(instance: *const c_void) -> i32;
	// reverse /usr/include/opencv2/dnn/all_layers.hpp:1037
	pub fn cv_dnn_CumSumLayer_setPropReverse_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:1039
	pub fn cv_dnn_CumSumLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:910
	pub fn cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:272
	pub fn cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// scale /usr/include/opencv2/dnn/all_layers.hpp:411
	pub fn cv_dnn_DequantizeLayer_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/dnn/all_layers.hpp:411
	pub fn cv_dnn_DequantizeLayer_setPropScale_float(instance: *mut c_void, val: f32);
	// zeropoint /usr/include/opencv2/dnn/all_layers.hpp:412
	pub fn cv_dnn_DequantizeLayer_getPropZeropoint_const(instance: *const c_void) -> i32;
	// zeropoint /usr/include/opencv2/dnn/all_layers.hpp:412
	pub fn cv_dnn_DequantizeLayer_setPropZeropoint_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:413
	pub fn cv_dnn_DequantizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DetectionModel(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1450
	pub fn cv_dnn_DetectionModel_DetectionModel_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// DetectionModel(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1456
	pub fn cv_dnn_DetectionModel_DetectionModel_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DetectionModel() /usr/include/opencv2/dnn/dnn.hpp:1459
	pub fn cv_dnn_DetectionModel_DetectionModel(ocvrs_return: *mut Result<*mut c_void>);
	// setNmsAcrossClasses(bool) /usr/include/opencv2/dnn/dnn.hpp:1467
	pub fn cv_dnn_DetectionModel_setNmsAcrossClasses_bool(instance: *mut c_void, value: bool, ocvrs_return: *mut Result<*mut c_void>);
	// getNmsAcrossClasses() /usr/include/opencv2/dnn/dnn.hpp:1473
	pub fn cv_dnn_DetectionModel_getNmsAcrossClasses(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// detect(cv::InputArray, std::vector<int> &, std::vector<float> &, std::vector<Rect> &, float, float) /usr/include/opencv2/dnn/dnn.hpp:1483
	pub fn cv_dnn_DetectionModel_detect_const__InputArrayR_vector_int_R_vector_float_R_vector_Rect_R_float_float(instance: *mut c_void, frame: *const c_void, class_ids: *mut c_void, confidences: *mut c_void, boxes: *mut c_void, conf_threshold: f32, nms_threshold: f32, ocvrs_return: *mut Result_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:962
	pub fn cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// has(const cv::String &) /usr/include/opencv2/dnn/dict.hpp:122
	pub fn cv_dnn_Dict_has_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<bool>);
	// ptr(const cv::String &) /usr/include/opencv2/dnn/dict.hpp:125
	pub fn cv_dnn_Dict_ptr_const_StringR(instance: *mut c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// ptr(const cv::String &) /usr/include/opencv2/dnn/dict.hpp:128
	pub fn cv_dnn_Dict_ptr_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// get(const cv::String &) /usr/include/opencv2/dnn/dict.hpp:131
	pub fn cv_dnn_Dict_get_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// set(const cv::String &, const T &) /usr/include/opencv2/dnn/dict.hpp:143
	pub fn cv_dnn_Dict_set_cv_String_const_StringR_const_StringR(instance: *mut c_void, key: *const c_char, value: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// set(const cv::String &, const T &) /usr/include/opencv2/dnn/dict.hpp:143
	pub fn cv_dnn_Dict_set_cv_dnn_DictValue_const_StringR_const_DictValueR(instance: *mut c_void, key: *const c_char, value: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// set(const cv::String &, const T &) /usr/include/opencv2/dnn/dict.hpp:143
	pub fn cv_dnn_Dict_set_double_const_StringR_const_doubleR(instance: *mut c_void, key: *const c_char, value: *const f64, ocvrs_return: *mut Result<f64>);
	// set(const cv::String &, const T &) /usr/include/opencv2/dnn/dict.hpp:143
	pub fn cv_dnn_Dict_set_int64_t_const_StringR_const_int64_tR(instance: *mut c_void, key: *const c_char, value: *const i64, ocvrs_return: *mut Result<i64>);
	// erase(const cv::String &) /usr/include/opencv2/dnn/dict.hpp:146
	pub fn cv_dnn_Dict_erase_const_StringR(instance: *mut c_void, key: *const c_char, ocvrs_return: *mut Result_void);
	// DictValue(const cv::dnn::DictValue &) /usr/include/opencv2/dnn/dict.hpp:62
	pub fn cv_dnn_DictValue_DictValue_const_DictValueR(r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(bool) /usr/include/opencv2/dnn/dict.hpp:63
	pub fn cv_dnn_DictValue_DictValue_bool(i: bool, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(int64) /usr/include/opencv2/dnn/dict.hpp:64
	pub fn cv_dnn_DictValue_DictValue_int64_t(i: i64, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(int) /usr/include/opencv2/dnn/dict.hpp:65
	pub fn cv_dnn_DictValue_DictValue_int(i: i32, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(unsigned int) /usr/include/opencv2/dnn/dict.hpp:66
	pub fn cv_dnn_DictValue_DictValue_unsigned_int(p: u32, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(double) /usr/include/opencv2/dnn/dict.hpp:67
	pub fn cv_dnn_DictValue_DictValue_double(p: f64, ocvrs_return: *mut Result<*mut c_void>);
	// DictValue(const char *) /usr/include/opencv2/dnn/dict.hpp:69
	pub fn cv_dnn_DictValue_DictValue_const_charX(s: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// get(int) /usr/include/opencv2/dnn/dict.hpp:79
	pub fn cv_dnn_DictValue_get_cv_String_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// get(int) /usr/include/opencv2/dnn/dict.hpp:79
	pub fn cv_dnn_DictValue_get_double_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<f64>);
	// get(int) /usr/include/opencv2/dnn/dict.hpp:79
	pub fn cv_dnn_DictValue_get_int_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i32>);
	// get(int) /usr/include/opencv2/dnn/dict.hpp:79
	pub fn cv_dnn_DictValue_get_int64_t_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i64>);
	// size() /usr/include/opencv2/dnn/dict.hpp:81
	pub fn cv_dnn_DictValue_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// isInt() /usr/include/opencv2/dnn/dict.hpp:83
	pub fn cv_dnn_DictValue_isInt_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isString() /usr/include/opencv2/dnn/dict.hpp:84
	pub fn cv_dnn_DictValue_isString_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isReal() /usr/include/opencv2/dnn/dict.hpp:85
	pub fn cv_dnn_DictValue_isReal_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getIntValue(int) /usr/include/opencv2/dnn/dict.hpp:87
	pub fn cv_dnn_DictValue_getIntValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i32>);
	// getRealValue(int) /usr/include/opencv2/dnn/dict.hpp:88
	pub fn cv_dnn_DictValue_getRealValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<f64>);
	// getStringValue(int) /usr/include/opencv2/dnn/dict.hpp:89
	pub fn cv_dnn_DictValue_getStringValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:574
	pub fn cv_dnn_ELULayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:574
	pub fn cv_dnn_ELULayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:576
	pub fn cv_dnn_ELULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:835
	pub fn cv_dnn_EltwiseLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:841
	pub fn cv_dnn_EltwiseLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:718
	pub fn cv_dnn_ErfLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// base /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_getPropBase_const(instance: *const c_void) -> f32;
	// base /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_setPropBase_float(instance: *mut c_void, val: f32);
	// scale /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_setPropScale_float(instance: *mut c_void, val: f32);
	// shift /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_getPropShift_const(instance: *const c_void) -> f32;
	// shift /usr/include/opencv2/dnn/all_layers.hpp:626
	pub fn cv_dnn_ExpLayer_setPropShift_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:628
	pub fn cv_dnn_ExpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:397
	pub fn cv_dnn_FlattenLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:640
	pub fn cv_dnn_FloorLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:928
	pub fn cv_dnn_FlowWarpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:199
	pub fn cv_dnn_GRULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:768
	pub fn cv_dnn_HardSigmoidLayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:768
	pub fn cv_dnn_HardSigmoidLayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// beta /usr/include/opencv2/dnn/all_layers.hpp:769
	pub fn cv_dnn_HardSigmoidLayer_getPropBeta_const(instance: *const c_void) -> f32;
	// beta /usr/include/opencv2/dnn/all_layers.hpp:769
	pub fn cv_dnn_HardSigmoidLayer_setPropBeta_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:771
	pub fn cv_dnn_HardSigmoidLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:724
	pub fn cv_dnn_HardSwishLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// axis /usr/include/opencv2/dnn/all_layers.hpp:362
	pub fn cv_dnn_InnerProductLayer_getPropAxis_const(instance: *const c_void) -> i32;
	// axis /usr/include/opencv2/dnn/all_layers.hpp:362
	pub fn cv_dnn_InnerProductLayer_setPropAxis_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:363
	pub fn cv_dnn_InnerProductLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:369
	pub fn cv_dnn_InnerProductLayerInt8_getPropInput_zp_const(instance: *const c_void) -> i32;
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:369
	pub fn cv_dnn_InnerProductLayerInt8_setPropInput_zp_int(instance: *mut c_void, val: i32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:369
	pub fn cv_dnn_InnerProductLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:369
	pub fn cv_dnn_InnerProductLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:370
	pub fn cv_dnn_InnerProductLayerInt8_getPropInput_sc_const(instance: *const c_void) -> f32;
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:370
	pub fn cv_dnn_InnerProductLayerInt8_setPropInput_sc_float(instance: *mut c_void, val: f32);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:370
	pub fn cv_dnn_InnerProductLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:370
	pub fn cv_dnn_InnerProductLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:371
	pub fn cv_dnn_InnerProductLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:1018
	pub fn cv_dnn_InterpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// KeypointsModel(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1387
	pub fn cv_dnn_KeypointsModel_KeypointsModel_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// KeypointsModel(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1393
	pub fn cv_dnn_KeypointsModel_KeypointsModel_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// estimate(cv::InputArray, float) /usr/include/opencv2/dnn/dnn.hpp:1401
	pub fn cv_dnn_KeypointsModel_estimate_const__InputArrayR_float(instance: *mut c_void, frame: *const c_void, thresh: f32, ocvrs_return: *mut Result<*mut c_void>);
	// type /usr/include/opencv2/dnn/all_layers.hpp:278
	pub fn cv_dnn_LRNLayer_getPropType_const(instance: *const c_void) -> i32;
	// type /usr/include/opencv2/dnn/all_layers.hpp:278
	pub fn cv_dnn_LRNLayer_setPropType_int(instance: *mut c_void, val: i32);
	// size /usr/include/opencv2/dnn/all_layers.hpp:280
	pub fn cv_dnn_LRNLayer_getPropSize_const(instance: *const c_void) -> i32;
	// size /usr/include/opencv2/dnn/all_layers.hpp:280
	pub fn cv_dnn_LRNLayer_setPropSize_int(instance: *mut c_void, val: i32);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// beta /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_getPropBeta_const(instance: *const c_void) -> f32;
	// beta /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_setPropBeta_float(instance: *mut c_void, val: f32);
	// bias /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_getPropBias_const(instance: *const c_void) -> f32;
	// bias /usr/include/opencv2/dnn/all_layers.hpp:281
	pub fn cv_dnn_LRNLayer_setPropBias_float(instance: *mut c_void, val: f32);
	// normBySize /usr/include/opencv2/dnn/all_layers.hpp:282
	pub fn cv_dnn_LRNLayer_getPropNormBySize_const(instance: *const c_void) -> bool;
	// normBySize /usr/include/opencv2/dnn/all_layers.hpp:282
	pub fn cv_dnn_LRNLayer_setPropNormBySize_bool(instance: *mut c_void, val: bool);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:284
	pub fn cv_dnn_LRNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:94
	pub fn cv_dnn_LSTMLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setWeights(const cv::Mat &, const cv::Mat &, const cv::Mat &) /usr/include/opencv2/dnn/all_layers.hpp:128
	pub fn cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(instance: *mut c_void, wh: *const c_void, wx: *const c_void, b: *const c_void, ocvrs_return: *mut Result_void);
	// setOutShape(const cv::dnn::MatShape &) /usr/include/opencv2/dnn/all_layers.hpp:134
	pub fn cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(instance: *mut c_void, out_tail_shape: *const c_void, ocvrs_return: *mut Result_void);
	// setUseTimstampsDim(bool) /usr/include/opencv2/dnn/all_layers.hpp:145
	pub fn cv_dnn_LSTMLayer_setUseTimstampsDim_bool(instance: *mut c_void, use_: bool, ocvrs_return: *mut Result_void);
	// setProduceCellOutput(bool) /usr/include/opencv2/dnn/all_layers.hpp:151
	pub fn cv_dnn_LSTMLayer_setProduceCellOutput_bool(instance: *mut c_void, produce: bool, ocvrs_return: *mut Result_void);
	// inputNameToIndex(cv::String) /usr/include/opencv2/dnn/all_layers.hpp:164
	pub fn cv_dnn_LSTMLayer_inputNameToIndex_String(instance: *mut c_void, input_name: *mut c_char, ocvrs_return: *mut Result<i32>);
	// outputNameToIndex(const cv::String &) /usr/include/opencv2/dnn/all_layers.hpp:165
	pub fn cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<i32>);
	// blobs /usr/include/opencv2/dnn/dnn.hpp:203
	pub fn cv_dnn_Layer_getPropBlobs_const(instance: *const c_void) -> *mut c_void;
	// blobs /usr/include/opencv2/dnn/dnn.hpp:203
	pub fn cv_dnn_Layer_setPropBlobs_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// name /usr/include/opencv2/dnn/dnn.hpp:421
	pub fn cv_dnn_Layer_getPropName_const(instance: *const c_void) -> *mut c_void;
	// name /usr/include/opencv2/dnn/dnn.hpp:421
	pub fn cv_dnn_Layer_setPropName_String(instance: *mut c_void, val: *mut c_char);
	// type /usr/include/opencv2/dnn/dnn.hpp:422
	pub fn cv_dnn_Layer_getPropType_const(instance: *const c_void) -> *mut c_void;
	// type /usr/include/opencv2/dnn/dnn.hpp:422
	pub fn cv_dnn_Layer_setPropType_String(instance: *mut c_void, val: *mut c_char);
	// preferableTarget /usr/include/opencv2/dnn/dnn.hpp:423
	pub fn cv_dnn_Layer_getPropPreferableTarget_const(instance: *const c_void) -> i32;
	// preferableTarget /usr/include/opencv2/dnn/dnn.hpp:423
	pub fn cv_dnn_Layer_setPropPreferableTarget_int(instance: *mut c_void, val: i32);
	// finalize(cv::InputArrayOfArrays, cv::OutputArrayOfArrays) /usr/include/opencv2/dnn/dnn.hpp:223
	pub fn cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, ocvrs_return: *mut Result_void);
	// forward(std::vector<Mat *> &, std::vector<Mat> &, std::vector<Mat> &) /usr/include/opencv2/dnn/dnn.hpp:232
	pub fn cv_dnn_Layer_forward_vector_MatX_R_vector_Mat_R_vector_Mat_R(instance: *mut c_void, input: *mut c_void, output: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result_void);
	// forward(cv::InputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays) /usr/include/opencv2/dnn/dnn.hpp:239
	pub fn cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, internals: *const c_void, ocvrs_return: *mut Result_void);
	// tryQuantize(const std::vector<std::vector<float>> &, const std::vector<std::vector<int>> &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:247
	pub fn cv_dnn_Layer_tryQuantize_const_vector_vector_float__R_const_vector_vector_int__R_LayerParamsR(instance: *mut c_void, scales: *const c_void, zeropoints: *const c_void, params: *mut c_void, ocvrs_return: *mut Result<bool>);
	// forward_fallback(cv::InputArrayOfArrays, cv::OutputArrayOfArrays, cv::OutputArrayOfArrays) /usr/include/opencv2/dnn/dnn.hpp:255
	pub fn cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, internals: *const c_void, ocvrs_return: *mut Result_void);
	// finalize(const std::vector<Mat> &, std::vector<Mat> &) /usr/include/opencv2/dnn/dnn.hpp:262
	pub fn cv_dnn_Layer_finalize_const_vector_Mat_R_vector_Mat_R(instance: *mut c_void, inputs: *const c_void, outputs: *mut c_void, ocvrs_return: *mut Result_void);
	// finalize(const std::vector<Mat> &) /usr/include/opencv2/dnn/dnn.hpp:268
	pub fn cv_dnn_Layer_finalize_const_vector_Mat_R(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// run(const std::vector<Mat> &, std::vector<Mat> &, std::vector<Mat> &) /usr/include/opencv2/dnn/dnn.hpp:273
	pub fn cv_dnn_Layer_run_const_vector_Mat_R_vector_Mat_R_vector_Mat_R(instance: *mut c_void, inputs: *const c_void, outputs: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result_void);
	// inputNameToIndex(cv::String) /usr/include/opencv2/dnn/dnn.hpp:282
	pub fn cv_dnn_Layer_inputNameToIndex_String(instance: *mut c_void, input_name: *mut c_char, ocvrs_return: *mut Result<i32>);
	// outputNameToIndex(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:286
	pub fn cv_dnn_Layer_outputNameToIndex_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<i32>);
	// supportBackend(int) /usr/include/opencv2/dnn/dnn.hpp:293
	pub fn cv_dnn_Layer_supportBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result<bool>);
	// initHalide(const std::vector<Ptr<BackendWrapper>> &) /usr/include/opencv2/dnn/dnn.hpp:305
	pub fn cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__R(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initNgraph(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &) /usr/include/opencv2/dnn/dnn.hpp:307
	pub fn cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendNode__R(instance: *mut c_void, inputs: *const c_void, nodes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initVkCom(const std::vector<Ptr<BackendWrapper>> &) /usr/include/opencv2/dnn/dnn.hpp:309
	pub fn cv_dnn_Layer_initVkCom_const_vector_Ptr_BackendWrapper__R(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initWebnn(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &) /usr/include/opencv2/dnn/dnn.hpp:311
	pub fn cv_dnn_Layer_initWebnn_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendNode__R(instance: *mut c_void, inputs: *const c_void, nodes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initCUDA(void *, const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendWrapper>> &) /usr/include/opencv2/dnn/dnn.hpp:320
	pub fn cv_dnn_Layer_initCUDA_voidX_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendWrapper__R(instance: *mut c_void, context: *mut c_void, inputs: *const c_void, outputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initTimVX(void *, const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendWrapper>> &, bool) /usr/include/opencv2/dnn/dnn.hpp:334
	pub fn cv_dnn_Layer_initTimVX_voidX_const_vector_Ptr_BackendWrapper__R_const_vector_Ptr_BackendWrapper__R_bool(instance: *mut c_void, tim_vx_info: *mut c_void, inputs_wrapper: *const c_void, outputs_wrapper: *const c_void, is_last: bool, ocvrs_return: *mut Result<*mut c_void>);
	// applyHalideScheduler(Ptr<cv::dnn::BackendNode> &, const std::vector<Mat *> &, const std::vector<Mat> &, int) /usr/include/opencv2/dnn/dnn.hpp:350
	pub fn cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_R_const_vector_MatX_R_const_vector_Mat_R_int(instance: *const c_void, node: *mut c_void, inputs: *const c_void, outputs: *const c_void, target_id: i32, ocvrs_return: *mut Result_void);
	// tryAttach(const Ptr<cv::dnn::BackendNode> &) /usr/include/opencv2/dnn/dnn.hpp:364
	pub fn cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_R(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setActivation(const Ptr<cv::dnn::ActivationLayer> &) /usr/include/opencv2/dnn/dnn.hpp:372
	pub fn cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_R(instance: *mut c_void, layer: *const c_void, ocvrs_return: *mut Result<bool>);
	// tryFuse(Ptr<cv::dnn::Layer> &) /usr/include/opencv2/dnn/dnn.hpp:379
	pub fn cv_dnn_Layer_tryFuse_Ptr_Layer_R(instance: *mut c_void, top: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getScaleShift(cv::Mat &, cv::Mat &) /usr/include/opencv2/dnn/dnn.hpp:394
	pub fn cv_dnn_Layer_getScaleShift_const_MatR_MatR(instance: *const c_void, scale: *mut c_void, shift: *mut c_void, ocvrs_return: *mut Result_void);
	// getScaleZeropoint(float &, int &) /usr/include/opencv2/dnn/dnn.hpp:403
	pub fn cv_dnn_Layer_getScaleZeropoint_const_floatR_intR(instance: *const c_void, scale: *mut f32, zeropoint: *mut i32, ocvrs_return: *mut Result_void);
	// unsetAttached() /usr/include/opencv2/dnn/dnn.hpp:409
	pub fn cv_dnn_Layer_unsetAttached(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getMemoryShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:411
	pub fn cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(instance: *const c_void, inputs: *const c_void, required_outputs: i32, outputs: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getFLOPS(const std::vector<MatShape> &, const std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:416
	pub fn cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_R_const_vector_MatShape_R(instance: *const c_void, inputs: *const c_void, outputs: *const c_void, ocvrs_return: *mut Result<i64>);
	// updateMemoryShapes(const std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:419
	pub fn cv_dnn_Layer_updateMemoryShapes_const_vector_MatShape_R(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<bool>);
	// Layer() /usr/include/opencv2/dnn/dnn.hpp:425
	pub fn cv_dnn_Layer_Layer(ocvrs_return: *mut Result<*mut c_void>);
	// Layer(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:426
	pub fn cv_dnn_Layer_Layer_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setParamsFrom(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:427
	pub fn cv_dnn_Layer_setParamsFrom_const_LayerParamsR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result_void);
	// registerLayer(const cv::String &, cv::dnn::LayerFactory::Constructor) /usr/include/opencv2/dnn/layer.hpp:64
	pub fn cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(typ: *const c_char, constructor: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, ocvrs_return: *mut Result_void);
	// unregisterLayer(const cv::String &) /usr/include/opencv2/dnn/layer.hpp:67
	pub fn cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ: *const c_char, ocvrs_return: *mut Result_void);
	// isLayerRegistered(const std::string &) /usr/include/opencv2/dnn/layer.hpp:70
	pub fn cv_dnn_LayerFactory_isLayerRegistered_const_stringR(typ: *const c_char, ocvrs_return: *mut Result<bool>);
	// createLayerInstance(const cv::String &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/layer.hpp:77
	pub fn cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// blobs /usr/include/opencv2/dnn/dnn.hpp:127
	pub fn cv_dnn_LayerParams_getPropBlobs_const(instance: *const c_void) -> *mut c_void;
	// blobs /usr/include/opencv2/dnn/dnn.hpp:127
	pub fn cv_dnn_LayerParams_setPropBlobs_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// name /usr/include/opencv2/dnn/dnn.hpp:129
	pub fn cv_dnn_LayerParams_getPropName_const(instance: *const c_void) -> *mut c_void;
	// name /usr/include/opencv2/dnn/dnn.hpp:129
	pub fn cv_dnn_LayerParams_setPropName_String(instance: *mut c_void, val: *mut c_char);
	// type /usr/include/opencv2/dnn/dnn.hpp:130
	pub fn cv_dnn_LayerParams_getPropType_const(instance: *const c_void) -> *mut c_void;
	// type /usr/include/opencv2/dnn/dnn.hpp:130
	pub fn cv_dnn_LayerParams_setPropType_String(instance: *mut c_void, val: *mut c_char);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:646
	pub fn cv_dnn_LogLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// eps /usr/include/opencv2/dnn/all_layers.hpp:377
	pub fn cv_dnn_MVNLayer_getPropEps_const(instance: *const c_void) -> f32;
	// eps /usr/include/opencv2/dnn/all_layers.hpp:377
	pub fn cv_dnn_MVNLayer_setPropEps_float(instance: *mut c_void, val: f32);
	// normVariance /usr/include/opencv2/dnn/all_layers.hpp:378
	pub fn cv_dnn_MVNLayer_getPropNormVariance_const(instance: *const c_void) -> bool;
	// normVariance /usr/include/opencv2/dnn/all_layers.hpp:378
	pub fn cv_dnn_MVNLayer_setPropNormVariance_bool(instance: *mut c_void, val: bool);
	// acrossChannels /usr/include/opencv2/dnn/all_layers.hpp:378
	pub fn cv_dnn_MVNLayer_getPropAcrossChannels_const(instance: *const c_void) -> bool;
	// acrossChannels /usr/include/opencv2/dnn/all_layers.hpp:378
	pub fn cv_dnn_MVNLayer_setPropAcrossChannels_bool(instance: *mut c_void, val: bool);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:380
	pub fn cv_dnn_MVNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// poolKernel /usr/include/opencv2/dnn/all_layers.hpp:864
	pub fn cv_dnn_MaxUnpoolLayer_getPropPoolKernel_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// poolKernel /usr/include/opencv2/dnn/all_layers.hpp:864
	pub fn cv_dnn_MaxUnpoolLayer_setPropPoolKernel_Size(instance: *mut c_void, val: *const core::Size);
	// poolPad /usr/include/opencv2/dnn/all_layers.hpp:865
	pub fn cv_dnn_MaxUnpoolLayer_getPropPoolPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// poolPad /usr/include/opencv2/dnn/all_layers.hpp:865
	pub fn cv_dnn_MaxUnpoolLayer_setPropPoolPad_Size(instance: *mut c_void, val: *const core::Size);
	// poolStride /usr/include/opencv2/dnn/all_layers.hpp:866
	pub fn cv_dnn_MaxUnpoolLayer_getPropPoolStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// poolStride /usr/include/opencv2/dnn/all_layers.hpp:866
	pub fn cv_dnn_MaxUnpoolLayer_setPropPoolStride_Size(instance: *mut c_void, val: *const core::Size);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:868
	pub fn cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:594
	pub fn cv_dnn_MishLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Model() /usr/include/opencv2/dnn/dnn.hpp:1222
	pub fn cv_dnn_Model_Model(ocvrs_return: *mut Result<*mut c_void>);
	// Model(const cv::dnn::Model &) /usr/include/opencv2/dnn/dnn.hpp:1224
	pub fn cv_dnn_Model_Model_const_ModelR(unnamed: *const c_void) -> *mut c_void;
	// Model(cv::dnn::Model &&) /usr/include/opencv2/dnn/dnn.hpp:1225
	pub fn cv_dnn_Model_Model_ModelR(unnamed: *mut c_void) -> *mut c_void;
	// Model(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1235
	pub fn cv_dnn_Model_Model_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// Model(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1241
	pub fn cv_dnn_Model_Model_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setInputSize(const cv::Size &) /usr/include/opencv2/dnn/dnn.hpp:1247
	pub fn cv_dnn_Model_setInputSize_const_SizeR(instance: *mut c_void, size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// setInputSize(int, int) /usr/include/opencv2/dnn/dnn.hpp:1254
	pub fn cv_dnn_Model_setInputSize_int_int(instance: *mut c_void, width: i32, height: i32, ocvrs_return: *mut Result<*mut c_void>);
	// setInputMean(const cv::Scalar &) /usr/include/opencv2/dnn/dnn.hpp:1259
	pub fn cv_dnn_Model_setInputMean_const_ScalarR(instance: *mut c_void, mean: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// setInputScale(double) /usr/include/opencv2/dnn/dnn.hpp:1264
	pub fn cv_dnn_Model_setInputScale_double(instance: *mut c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// setInputCrop(bool) /usr/include/opencv2/dnn/dnn.hpp:1269
	pub fn cv_dnn_Model_setInputCrop_bool(instance: *mut c_void, crop: bool, ocvrs_return: *mut Result<*mut c_void>);
	// setInputSwapRB(bool) /usr/include/opencv2/dnn/dnn.hpp:1274
	pub fn cv_dnn_Model_setInputSwapRB_bool(instance: *mut c_void, swap_rb: bool, ocvrs_return: *mut Result<*mut c_void>);
	// setInputParams(double, const cv::Size &, const cv::Scalar &, bool, bool) /usr/include/opencv2/dnn/dnn.hpp:1284
	pub fn cv_dnn_Model_setInputParams_double_const_SizeR_const_ScalarR_bool_bool(instance: *mut c_void, scale: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ocvrs_return: *mut Result_void);
	// predict(cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/dnn/dnn.hpp:1291
	pub fn cv_dnn_Model_predict_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, frame: *const c_void, outs: *const c_void, ocvrs_return: *mut Result_void);
	// setPreferableBackend(dnn::Backend) /usr/include/opencv2/dnn/dnn.hpp:1301
	pub fn cv_dnn_Model_setPreferableBackend_Backend(instance: *mut c_void, backend_id: crate::dnn::Backend, ocvrs_return: *mut Result<*mut c_void>);
	// setPreferableTarget(dnn::Target) /usr/include/opencv2/dnn/dnn.hpp:1303
	pub fn cv_dnn_Model_setPreferableTarget_Target(instance: *mut c_void, target_id: crate::dnn::Target, ocvrs_return: *mut Result<*mut c_void>);
	// getNetwork_() /usr/include/opencv2/dnn/dnn.hpp:1309
	pub fn cv_dnn_Model_getNetwork__const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNetwork_() /usr/include/opencv2/dnn/dnn.hpp:1310
	pub fn cv_dnn_Model_getNetwork_(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Net() /usr/include/opencv2/dnn/dnn.hpp:445
	pub fn cv_dnn_Net_Net(ocvrs_return: *mut Result<*mut c_void>);
	// readFromModelOptimizer(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:454
	pub fn cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(xml: *const c_char, bin: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// readFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &) /usr/include/opencv2/dnn/dnn.hpp:462
	pub fn cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_R_const_vector_unsigned_char_R(buffer_model_config: *const c_void, buffer_weights: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readFromModelOptimizer(const uchar *, size_t, const uchar *, size_t) /usr/include/opencv2/dnn/dnn.hpp:472
	pub fn cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr: *const u8, buffer_model_config_size: size_t, buffer_weights_ptr: *const u8, buffer_weights_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/dnn/dnn.hpp:476
	pub fn cv_dnn_Net_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// dump() /usr/include/opencv2/dnn/dnn.hpp:482
	pub fn cv_dnn_Net_dump(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpToFile(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:487
	pub fn cv_dnn_Net_dumpToFile_const_StringR(instance: *mut c_void, path: *const c_char, ocvrs_return: *mut Result_void);
	// addLayer(const cv::String &, const cv::String &, const int &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:495
	pub fn cv_dnn_Net_addLayer_const_StringR_const_StringR_const_intR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, dtype: *const i32, params: *mut c_void, ocvrs_return: *mut Result<i32>);
	// addLayer(const cv::String &, const cv::String &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:498
	pub fn cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<i32>);
	// addLayerToPrev(const cv::String &, const cv::String &, const int &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:503
	pub fn cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_const_intR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, dtype: *const i32, params: *mut c_void, ocvrs_return: *mut Result<i32>);
	// addLayerToPrev(const cv::String &, const cv::String &, cv::dnn::LayerParams &) /usr/include/opencv2/dnn/dnn.hpp:506
	pub fn cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<i32>);
	// getLayerId(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:511
	pub fn cv_dnn_Net_getLayerId_const_const_StringR(instance: *const c_void, layer: *const c_char, ocvrs_return: *mut Result<i32>);
	// getLayerNames() /usr/include/opencv2/dnn/dnn.hpp:513
	pub fn cv_dnn_Net_getLayerNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLayer(int) /usr/include/opencv2/dnn/dnn.hpp:522
	pub fn cv_dnn_Net_getLayer_const_int(instance: *const c_void, layer_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getLayer(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:526
	pub fn cv_dnn_Net_getLayer_const_const_StringR(instance: *const c_void, layer_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getLayer(const cv::dnn::Net::LayerId &) /usr/include/opencv2/dnn/dnn.hpp:530
	pub fn cv_dnn_Net_getLayer_const_const_LayerIdR(instance: *const c_void, layer_id: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLayerInputs(int) /usr/include/opencv2/dnn/dnn.hpp:533
	pub fn cv_dnn_Net_getLayerInputs_const_int(instance: *const c_void, layer_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// connect(cv::String, cv::String) /usr/include/opencv2/dnn/dnn.hpp:548
	pub fn cv_dnn_Net_connect_String_String(instance: *mut c_void, out_pin: *mut c_char, inp_pin: *mut c_char, ocvrs_return: *mut Result_void);
	// connect(int, int, int, int) /usr/include/opencv2/dnn/dnn.hpp:556
	pub fn cv_dnn_Net_connect_int_int_int_int(instance: *mut c_void, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32, ocvrs_return: *mut Result_void);
	// registerOutput(const std::string &, int, int) /usr/include/opencv2/dnn/dnn.hpp:568
	pub fn cv_dnn_Net_registerOutput_const_stringR_int_int(instance: *mut c_void, output_name: *const c_char, layer_id: i32, output_port: i32, ocvrs_return: *mut Result<i32>);
	// setInputsNames(const std::vector<String> &) /usr/include/opencv2/dnn/dnn.hpp:577
	pub fn cv_dnn_Net_setInputsNames_const_vector_String_R(instance: *mut c_void, input_blob_names: *const c_void, ocvrs_return: *mut Result_void);
	// setInputShape(const cv::String &, const cv::dnn::MatShape &) /usr/include/opencv2/dnn/dnn.hpp:581
	pub fn cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(instance: *mut c_void, input_name: *const c_char, shape: *const c_void, ocvrs_return: *mut Result_void);
	// forward(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:588
	pub fn cv_dnn_Net_forward_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// forwardAsync(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:597
	pub fn cv_dnn_Net_forwardAsync_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// forward(cv::OutputArrayOfArrays, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:604
	pub fn cv_dnn_Net_forward_const__OutputArrayR_const_StringR(instance: *mut c_void, output_blobs: *const c_void, output_name: *const c_char, ocvrs_return: *mut Result_void);
	// forward(cv::OutputArrayOfArrays, const std::vector<String> &) /usr/include/opencv2/dnn/dnn.hpp:610
	pub fn cv_dnn_Net_forward_const__OutputArrayR_const_vector_String_R(instance: *mut c_void, output_blobs: *const c_void, out_blob_names: *const c_void, ocvrs_return: *mut Result_void);
	// forward(std::vector<std::vector<Mat>> &, const std::vector<String> &) /usr/include/opencv2/dnn/dnn.hpp:617
	pub fn cv_dnn_Net_forward_vector_vector_Mat__R_const_vector_String_R(instance: *mut c_void, output_blobs: *mut c_void, out_blob_names: *const c_void, ocvrs_return: *mut Result_void);
	// quantize(cv::InputArrayOfArrays, int, int) /usr/include/opencv2/dnn/dnn.hpp:625
	pub fn cv_dnn_Net_quantize_const__InputArrayR_int_int(instance: *mut c_void, calib_data: *const c_void, inputs_dtype: i32, outputs_dtype: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getInputDetails(std::vector<float> &, std::vector<int> &) /usr/include/opencv2/dnn/dnn.hpp:631
	pub fn cv_dnn_Net_getInputDetails_const_vector_float_R_vector_int_R(instance: *const c_void, scales: *mut c_void, zeropoints: *mut c_void, ocvrs_return: *mut Result_void);
	// getOutputDetails(std::vector<float> &, std::vector<int> &) /usr/include/opencv2/dnn/dnn.hpp:637
	pub fn cv_dnn_Net_getOutputDetails_const_vector_float_R_vector_int_R(instance: *const c_void, scales: *mut c_void, zeropoints: *mut c_void, ocvrs_return: *mut Result_void);
	// setHalideScheduler(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:648
	pub fn cv_dnn_Net_setHalideScheduler_const_StringR(instance: *mut c_void, scheduler: *const c_char, ocvrs_return: *mut Result_void);
	// setPreferableBackend(int) /usr/include/opencv2/dnn/dnn.hpp:658
	pub fn cv_dnn_Net_setPreferableBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result_void);
	// setPreferableTarget(int) /usr/include/opencv2/dnn/dnn.hpp:677
	pub fn cv_dnn_Net_setPreferableTarget_int(instance: *mut c_void, target_id: i32, ocvrs_return: *mut Result_void);
	// setInput(cv::InputArray, const cv::String &, double, const cv::Scalar &) /usr/include/opencv2/dnn/dnn.hpp:690
	pub fn cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(instance: *mut c_void, blob: *const c_void, name: *const c_char, scalefactor: f64, mean: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setParam(int, int, const cv::Mat &) /usr/include/opencv2/dnn/dnn.hpp:701
	pub fn cv_dnn_Net_setParam_int_int_const_MatR(instance: *mut c_void, layer: i32, num_param: i32, blob: *const c_void, ocvrs_return: *mut Result_void);
	// setParam(const cv::String &, int, const cv::Mat &) /usr/include/opencv2/dnn/dnn.hpp:702
	pub fn cv_dnn_Net_setParam_const_StringR_int_const_MatR(instance: *mut c_void, layer_name: *const c_char, num_param: i32, blob: *const c_void, ocvrs_return: *mut Result_void);
	// getParam(int, int) /usr/include/opencv2/dnn/dnn.hpp:709
	pub fn cv_dnn_Net_getParam_const_int_int(instance: *const c_void, layer: i32, num_param: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getParam(const cv::String &, int) /usr/include/opencv2/dnn/dnn.hpp:710
	pub fn cv_dnn_Net_getParam_const_const_StringR_int(instance: *const c_void, layer_name: *const c_char, num_param: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getUnconnectedOutLayers() /usr/include/opencv2/dnn/dnn.hpp:716
	pub fn cv_dnn_Net_getUnconnectedOutLayers_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getUnconnectedOutLayersNames() /usr/include/opencv2/dnn/dnn.hpp:722
	pub fn cv_dnn_Net_getUnconnectedOutLayersNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLayersShapes(const std::vector<MatShape> &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &) /usr/include/opencv2/dnn/dnn.hpp:733
	pub fn cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_R_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(instance: *const c_void, net_input_shapes: *const c_void, layers_ids: *mut c_void, in_layers_shapes: *mut c_void, out_layers_shapes: *mut c_void, ocvrs_return: *mut Result_void);
	// getLayersShapes(const cv::dnn::MatShape &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &) /usr/include/opencv2/dnn/dnn.hpp:739
	pub fn cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vector_int_R_vector_vector_MatShape__R_vector_vector_MatShape__R(instance: *const c_void, net_input_shape: *const c_void, layers_ids: *mut c_void, in_layers_shapes: *mut c_void, out_layers_shapes: *mut c_void, ocvrs_return: *mut Result_void);
	// getLayerShapes(const cv::dnn::MatShape &, const int, std::vector<MatShape> &, std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:753
	pub fn cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vector_MatShape_R_vector_MatShape_R(instance: *const c_void, net_input_shape: *const c_void, layer_id: i32, in_layer_shapes: *mut c_void, out_layer_shapes: *mut c_void, ocvrs_return: *mut Result_void);
	// getLayerShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:759
	pub fn cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_R_const_int_vector_MatShape_R_vector_MatShape_R(instance: *const c_void, net_input_shapes: *const c_void, layer_id: i32, in_layer_shapes: *mut c_void, out_layer_shapes: *mut c_void, ocvrs_return: *mut Result_void);
	// getFLOPS(const std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:768
	pub fn cv_dnn_Net_getFLOPS_const_const_vector_MatShape_R(instance: *const c_void, net_input_shapes: *const c_void, ocvrs_return: *mut Result<i64>);
	// getFLOPS(const cv::dnn::MatShape &) /usr/include/opencv2/dnn/dnn.hpp:770
	pub fn cv_dnn_Net_getFLOPS_const_const_MatShapeR(instance: *const c_void, net_input_shape: *const c_void, ocvrs_return: *mut Result<i64>);
	// getFLOPS(const int, const std::vector<MatShape> &) /usr/include/opencv2/dnn/dnn.hpp:772
	pub fn cv_dnn_Net_getFLOPS_const_const_int_const_vector_MatShape_R(instance: *const c_void, layer_id: i32, net_input_shapes: *const c_void, ocvrs_return: *mut Result<i64>);
	// getFLOPS(const int, const cv::dnn::MatShape &) /usr/include/opencv2/dnn/dnn.hpp:775
	pub fn cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(instance: *const c_void, layer_id: i32, net_input_shape: *const c_void, ocvrs_return: *mut Result<i64>);
	// getLayerTypes(std::vector<String> &) /usr/include/opencv2/dnn/dnn.hpp:781
	pub fn cv_dnn_Net_getLayerTypes_const_vector_String_R(instance: *const c_void, layers_types: *mut c_void, ocvrs_return: *mut Result_void);
	// getLayersCount(const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:787
	pub fn cv_dnn_Net_getLayersCount_const_const_StringR(instance: *const c_void, layer_type: *const c_char, ocvrs_return: *mut Result<i32>);
	// getMemoryConsumption(const std::vector<MatShape> &, size_t &, size_t &) /usr/include/opencv2/dnn/dnn.hpp:795
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_size_tR_size_tR(instance: *const c_void, net_input_shapes: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result_void);
	// getMemoryConsumption(const cv::dnn::MatShape &, size_t &, size_t &) /usr/include/opencv2/dnn/dnn.hpp:798
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(instance: *const c_void, net_input_shape: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result_void);
	// getMemoryConsumption(const int, const std::vector<MatShape> &, size_t &, size_t &) /usr/include/opencv2/dnn/dnn.hpp:801
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_int_const_vector_MatShape_R_size_tR_size_tR(instance: *const c_void, layer_id: i32, net_input_shapes: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result_void);
	// getMemoryConsumption(const int, const cv::dnn::MatShape &, size_t &, size_t &) /usr/include/opencv2/dnn/dnn.hpp:805
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(instance: *const c_void, layer_id: i32, net_input_shape: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result_void);
	// getMemoryConsumption(const std::vector<MatShape> &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &) /usr/include/opencv2/dnn/dnn.hpp:816
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_R_vector_int_R_vector_size_t_R_vector_size_t_R(instance: *const c_void, net_input_shapes: *const c_void, layer_ids: *mut c_void, weights: *mut c_void, blobs: *mut c_void, ocvrs_return: *mut Result_void);
	// getMemoryConsumption(const cv::dnn::MatShape &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &) /usr/include/opencv2/dnn/dnn.hpp:821
	pub fn cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vector_int_R_vector_size_t_R_vector_size_t_R(instance: *const c_void, net_input_shape: *const c_void, layer_ids: *mut c_void, weights: *mut c_void, blobs: *mut c_void, ocvrs_return: *mut Result_void);
	// enableFusion(bool) /usr/include/opencv2/dnn/dnn.hpp:829
	pub fn cv_dnn_Net_enableFusion_bool(instance: *mut c_void, fusion: bool, ocvrs_return: *mut Result_void);
	// getPerfProfile(std::vector<double> &) /usr/include/opencv2/dnn/dnn.hpp:839
	pub fn cv_dnn_Net_getPerfProfile_vector_double_R(instance: *mut c_void, timings: *mut c_void, ocvrs_return: *mut Result<i64>);
	// pnorm /usr/include/opencv2/dnn/all_layers.hpp:993
	pub fn cv_dnn_NormalizeBBoxLayer_getPropPnorm_const(instance: *const c_void) -> f32;
	// pnorm /usr/include/opencv2/dnn/all_layers.hpp:993
	pub fn cv_dnn_NormalizeBBoxLayer_setPropPnorm_float(instance: *mut c_void, val: f32);
	// epsilon /usr/include/opencv2/dnn/all_layers.hpp:993
	pub fn cv_dnn_NormalizeBBoxLayer_getPropEpsilon_const(instance: *const c_void) -> f32;
	// epsilon /usr/include/opencv2/dnn/all_layers.hpp:993
	pub fn cv_dnn_NormalizeBBoxLayer_setPropEpsilon_float(instance: *mut c_void, val: f32);
	// acrossSpatial /usr/include/opencv2/dnn/all_layers.hpp:994
	pub fn cv_dnn_NormalizeBBoxLayer_getPropAcrossSpatial_const(instance: *const c_void) -> bool;
	// acrossSpatial /usr/include/opencv2/dnn/all_layers.hpp:994
	pub fn cv_dnn_NormalizeBBoxLayer_setPropAcrossSpatial_bool(instance: *mut c_void, val: bool);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:996
	pub fn cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:664
	pub fn cv_dnn_NotLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:534
	pub fn cv_dnn_PaddingLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:491
	pub fn cv_dnn_PermuteLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// type /usr/include/opencv2/dnn/all_layers.hpp:300
	pub fn cv_dnn_PoolingLayer_getPropType_const(instance: *const c_void) -> i32;
	// type /usr/include/opencv2/dnn/all_layers.hpp:300
	pub fn cv_dnn_PoolingLayer_setPropType_int(instance: *mut c_void, val: i32);
	// kernel_size /usr/include/opencv2/dnn/all_layers.hpp:301
	pub fn cv_dnn_PoolingLayer_getPropKernel_size_const(instance: *const c_void) -> *mut c_void;
	// kernel_size /usr/include/opencv2/dnn/all_layers.hpp:301
	pub fn cv_dnn_PoolingLayer_setPropKernel_size_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// strides /usr/include/opencv2/dnn/all_layers.hpp:301
	pub fn cv_dnn_PoolingLayer_getPropStrides_const(instance: *const c_void) -> *mut c_void;
	// strides /usr/include/opencv2/dnn/all_layers.hpp:301
	pub fn cv_dnn_PoolingLayer_setPropStrides_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// pads_begin /usr/include/opencv2/dnn/all_layers.hpp:302
	pub fn cv_dnn_PoolingLayer_getPropPads_begin_const(instance: *const c_void) -> *mut c_void;
	// pads_begin /usr/include/opencv2/dnn/all_layers.hpp:302
	pub fn cv_dnn_PoolingLayer_setPropPads_begin_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// pads_end /usr/include/opencv2/dnn/all_layers.hpp:302
	pub fn cv_dnn_PoolingLayer_getPropPads_end_const(instance: *const c_void) -> *mut c_void;
	// pads_end /usr/include/opencv2/dnn/all_layers.hpp:302
	pub fn cv_dnn_PoolingLayer_setPropPads_end_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// globalPooling /usr/include/opencv2/dnn/all_layers.hpp:303
	pub fn cv_dnn_PoolingLayer_getPropGlobalPooling_const(instance: *const c_void) -> bool;
	// globalPooling /usr/include/opencv2/dnn/all_layers.hpp:303
	pub fn cv_dnn_PoolingLayer_setPropGlobalPooling_bool(instance: *mut c_void, val: bool);
	// isGlobalPooling /usr/include/opencv2/dnn/all_layers.hpp:304
	pub fn cv_dnn_PoolingLayer_getPropIsGlobalPooling_const(instance: *const c_void) -> *mut c_void;
	// isGlobalPooling /usr/include/opencv2/dnn/all_layers.hpp:304
	pub fn cv_dnn_PoolingLayer_setPropIsGlobalPooling_vector_bool_(instance: *mut c_void, val: *mut c_void);
	// computeMaxIdx /usr/include/opencv2/dnn/all_layers.hpp:305
	pub fn cv_dnn_PoolingLayer_getPropComputeMaxIdx_const(instance: *const c_void) -> bool;
	// computeMaxIdx /usr/include/opencv2/dnn/all_layers.hpp:305
	pub fn cv_dnn_PoolingLayer_setPropComputeMaxIdx_bool(instance: *mut c_void, val: bool);
	// padMode /usr/include/opencv2/dnn/all_layers.hpp:306
	pub fn cv_dnn_PoolingLayer_getPropPadMode_const(instance: *const c_void) -> *mut c_void;
	// padMode /usr/include/opencv2/dnn/all_layers.hpp:306
	pub fn cv_dnn_PoolingLayer_setPropPadMode_String(instance: *mut c_void, val: *mut c_char);
	// ceilMode /usr/include/opencv2/dnn/all_layers.hpp:307
	pub fn cv_dnn_PoolingLayer_getPropCeilMode_const(instance: *const c_void) -> bool;
	// ceilMode /usr/include/opencv2/dnn/all_layers.hpp:307
	pub fn cv_dnn_PoolingLayer_setPropCeilMode_bool(instance: *mut c_void, val: bool);
	// avePoolPaddedArea /usr/include/opencv2/dnn/all_layers.hpp:311
	pub fn cv_dnn_PoolingLayer_getPropAvePoolPaddedArea_const(instance: *const c_void) -> bool;
	// avePoolPaddedArea /usr/include/opencv2/dnn/all_layers.hpp:311
	pub fn cv_dnn_PoolingLayer_setPropAvePoolPaddedArea_bool(instance: *mut c_void, val: bool);
	// pooledSize /usr/include/opencv2/dnn/all_layers.hpp:313
	pub fn cv_dnn_PoolingLayer_getPropPooledSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// pooledSize /usr/include/opencv2/dnn/all_layers.hpp:313
	pub fn cv_dnn_PoolingLayer_setPropPooledSize_Size(instance: *mut c_void, val: *const core::Size);
	// spatialScale /usr/include/opencv2/dnn/all_layers.hpp:314
	pub fn cv_dnn_PoolingLayer_getPropSpatialScale_const(instance: *const c_void) -> f32;
	// spatialScale /usr/include/opencv2/dnn/all_layers.hpp:314
	pub fn cv_dnn_PoolingLayer_setPropSpatialScale_float(instance: *mut c_void, val: f32);
	// psRoiOutChannels /usr/include/opencv2/dnn/all_layers.hpp:316
	pub fn cv_dnn_PoolingLayer_getPropPsRoiOutChannels_const(instance: *const c_void) -> i32;
	// psRoiOutChannels /usr/include/opencv2/dnn/all_layers.hpp:316
	pub fn cv_dnn_PoolingLayer_setPropPsRoiOutChannels_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:318
	pub fn cv_dnn_PoolingLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:324
	pub fn cv_dnn_PoolingLayerInt8_getPropInput_zp_const(instance: *const c_void) -> i32;
	// input_zp /usr/include/opencv2/dnn/all_layers.hpp:324
	pub fn cv_dnn_PoolingLayerInt8_setPropInput_zp_int(instance: *mut c_void, val: i32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:324
	pub fn cv_dnn_PoolingLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:324
	pub fn cv_dnn_PoolingLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:325
	pub fn cv_dnn_PoolingLayerInt8_getPropInput_sc_const(instance: *const c_void) -> f32;
	// input_sc /usr/include/opencv2/dnn/all_layers.hpp:325
	pub fn cv_dnn_PoolingLayerInt8_setPropInput_sc_float(instance: *mut c_void, val: f32);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:325
	pub fn cv_dnn_PoolingLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:325
	pub fn cv_dnn_PoolingLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:326
	pub fn cv_dnn_PoolingLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// power /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_getPropPower_const(instance: *const c_void) -> f32;
	// power /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_setPropPower_float(instance: *mut c_void, val: f32);
	// scale /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_setPropScale_float(instance: *mut c_void, val: f32);
	// shift /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_getPropShift_const(instance: *const c_void) -> f32;
	// shift /usr/include/opencv2/dnn/all_layers.hpp:618
	pub fn cv_dnn_PowerLayer_setPropShift_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:620
	pub fn cv_dnn_PowerLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:934
	pub fn cv_dnn_PriorBoxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:1024
	pub fn cv_dnn_ProposalLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// scale /usr/include/opencv2/dnn/all_layers.hpp:403
	pub fn cv_dnn_QuantizeLayer_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/dnn/all_layers.hpp:403
	pub fn cv_dnn_QuantizeLayer_setPropScale_float(instance: *mut c_void, val: f32);
	// zeropoint /usr/include/opencv2/dnn/all_layers.hpp:404
	pub fn cv_dnn_QuantizeLayer_getPropZeropoint_const(instance: *const c_void) -> i32;
	// zeropoint /usr/include/opencv2/dnn/all_layers.hpp:404
	pub fn cv_dnn_QuantizeLayer_setPropZeropoint_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:405
	pub fn cv_dnn_QuantizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:219
	pub fn cv_dnn_RNNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setWeights(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &) /usr/include/opencv2/dnn/all_layers.hpp:235
	pub fn cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(instance: *mut c_void, wxh: *const c_void, bh: *const c_void, whh: *const c_void, who: *const c_void, bo: *const c_void, ocvrs_return: *mut Result_void);
	// setProduceHiddenOutput(bool) /usr/include/opencv2/dnn/all_layers.hpp:240
	pub fn cv_dnn_RNNLayer_setProduceHiddenOutput_bool(instance: *mut c_void, produce: bool, ocvrs_return: *mut Result_void);
	// minValue /usr/include/opencv2/dnn/all_layers.hpp:560
	pub fn cv_dnn_ReLU6Layer_getPropMinValue_const(instance: *const c_void) -> f32;
	// minValue /usr/include/opencv2/dnn/all_layers.hpp:560
	pub fn cv_dnn_ReLU6Layer_setPropMinValue_float(instance: *mut c_void, val: f32);
	// maxValue /usr/include/opencv2/dnn/all_layers.hpp:560
	pub fn cv_dnn_ReLU6Layer_getPropMaxValue_const(instance: *const c_void) -> f32;
	// maxValue /usr/include/opencv2/dnn/all_layers.hpp:560
	pub fn cv_dnn_ReLU6Layer_setPropMaxValue_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:562
	pub fn cv_dnn_ReLU6Layer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// negativeSlope /usr/include/opencv2/dnn/all_layers.hpp:552
	pub fn cv_dnn_ReLULayer_getPropNegativeSlope_const(instance: *const c_void) -> f32;
	// negativeSlope /usr/include/opencv2/dnn/all_layers.hpp:552
	pub fn cv_dnn_ReLULayer_setPropNegativeSlope_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:554
	pub fn cv_dnn_ReLULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:814
	pub fn cv_dnn_ReciprocalLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reduceType /usr/include/opencv2/dnn/all_layers.hpp:332
	pub fn cv_dnn_ReduceLayer_getPropReduceType_const(instance: *const c_void) -> i32;
	// reduceType /usr/include/opencv2/dnn/all_layers.hpp:332
	pub fn cv_dnn_ReduceLayer_setPropReduceType_int(instance: *mut c_void, val: i32);
	// reduceDims /usr/include/opencv2/dnn/all_layers.hpp:333
	pub fn cv_dnn_ReduceLayer_getPropReduceDims_const(instance: *const c_void) -> *mut c_void;
	// reduceDims /usr/include/opencv2/dnn/all_layers.hpp:333
	pub fn cv_dnn_ReduceLayer_setPropReduceDims_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:334
	pub fn cv_dnn_ReduceLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:340
	pub fn cv_dnn_ReduceLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// nmsThreshold /usr/include/opencv2/dnn/all_layers.hpp:946
	pub fn cv_dnn_RegionLayer_getPropNmsThreshold_const(instance: *const c_void) -> f32;
	// nmsThreshold /usr/include/opencv2/dnn/all_layers.hpp:946
	pub fn cv_dnn_RegionLayer_setPropNmsThreshold_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:948
	pub fn cv_dnn_RegionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:940
	pub fn cv_dnn_ReorgLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// scale /usr/include/opencv2/dnn/all_layers.hpp:419
	pub fn cv_dnn_RequantizeLayer_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/dnn/all_layers.hpp:419
	pub fn cv_dnn_RequantizeLayer_setPropScale_float(instance: *mut c_void, val: f32);
	// shift /usr/include/opencv2/dnn/all_layers.hpp:419
	pub fn cv_dnn_RequantizeLayer_getPropShift_const(instance: *const c_void) -> f32;
	// shift /usr/include/opencv2/dnn/all_layers.hpp:419
	pub fn cv_dnn_RequantizeLayer_setPropShift_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:420
	pub fn cv_dnn_RequantizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// newShapeDesc /usr/include/opencv2/dnn/all_layers.hpp:388
	pub fn cv_dnn_ReshapeLayer_getPropNewShapeDesc_const(instance: *const c_void) -> *mut c_void;
	// newShapeDesc /usr/include/opencv2/dnn/all_layers.hpp:388
	pub fn cv_dnn_ReshapeLayer_setPropNewShapeDesc_MatShape(instance: *mut c_void, val: *mut c_void);
	// newShapeRange /usr/include/opencv2/dnn/all_layers.hpp:389
	pub fn cv_dnn_ReshapeLayer_getPropNewShapeRange_const(instance: *const c_void) -> *mut c_void;
	// newShapeRange /usr/include/opencv2/dnn/all_layers.hpp:389
	pub fn cv_dnn_ReshapeLayer_setPropNewShapeRange_Range(instance: *mut c_void, val: *mut c_void);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:391
	pub fn cv_dnn_ReshapeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:1007
	pub fn cv_dnn_ResizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:652
	pub fn cv_dnn_RoundLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// hasBias /usr/include/opencv2/dnn/all_layers.hpp:874
	pub fn cv_dnn_ScaleLayer_getPropHasBias_const(instance: *const c_void) -> bool;
	// hasBias /usr/include/opencv2/dnn/all_layers.hpp:874
	pub fn cv_dnn_ScaleLayer_setPropHasBias_bool(instance: *mut c_void, val: bool);
	// axis /usr/include/opencv2/dnn/all_layers.hpp:875
	pub fn cv_dnn_ScaleLayer_getPropAxis_const(instance: *const c_void) -> i32;
	// axis /usr/include/opencv2/dnn/all_layers.hpp:875
	pub fn cv_dnn_ScaleLayer_setPropAxis_int(instance: *mut c_void, val: i32);
	// mode /usr/include/opencv2/dnn/all_layers.hpp:876
	pub fn cv_dnn_ScaleLayer_getPropMode_const(instance: *const c_void) -> *mut c_void;
	// mode /usr/include/opencv2/dnn/all_layers.hpp:876
	pub fn cv_dnn_ScaleLayer_setPropMode_String(instance: *mut c_void, val: *mut c_char);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:878
	pub fn cv_dnn_ScaleLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:884
	pub fn cv_dnn_ScaleLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:884
	pub fn cv_dnn_ScaleLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:885
	pub fn cv_dnn_ScaleLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:885
	pub fn cv_dnn_ScaleLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:886
	pub fn cv_dnn_ScaleLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// SegmentationModel(const cv::String &, const cv::String &) /usr/include/opencv2/dnn/dnn.hpp:1419
	pub fn cv_dnn_SegmentationModel_SegmentationModel_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// SegmentationModel(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1425
	pub fn cv_dnn_SegmentationModel_SegmentationModel_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// segment(cv::InputArray, cv::OutputArray) /usr/include/opencv2/dnn/dnn.hpp:1431
	pub fn cv_dnn_SegmentationModel_segment_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:777
	pub fn cv_dnn_SeluLayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:777
	pub fn cv_dnn_SeluLayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// gamma /usr/include/opencv2/dnn/all_layers.hpp:778
	pub fn cv_dnn_SeluLayer_getPropGamma_const(instance: *const c_void) -> f32;
	// gamma /usr/include/opencv2/dnn/all_layers.hpp:778
	pub fn cv_dnn_SeluLayer_setPropGamma_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:780
	pub fn cv_dnn_SeluLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:892
	pub fn cv_dnn_ShiftLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:898
	pub fn cv_dnn_ShiftLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bias /usr/include/opencv2/dnn/all_layers.hpp:806
	pub fn cv_dnn_ShrinkLayer_getPropBias_const(instance: *const c_void) -> f32;
	// bias /usr/include/opencv2/dnn/all_layers.hpp:806
	pub fn cv_dnn_ShrinkLayer_setPropBias_float(instance: *mut c_void, val: f32);
	// lambd /usr/include/opencv2/dnn/all_layers.hpp:807
	pub fn cv_dnn_ShrinkLayer_getPropLambd_const(instance: *const c_void) -> f32;
	// lambd /usr/include/opencv2/dnn/all_layers.hpp:807
	pub fn cv_dnn_ShrinkLayer_setPropLambd_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:808
	pub fn cv_dnn_ShrinkLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// group /usr/include/opencv2/dnn/all_layers.hpp:508
	pub fn cv_dnn_ShuffleChannelLayer_getPropGroup_const(instance: *const c_void) -> i32;
	// group /usr/include/opencv2/dnn/all_layers.hpp:508
	pub fn cv_dnn_ShuffleChannelLayer_setPropGroup_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:506
	pub fn cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:600
	pub fn cv_dnn_SigmoidLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:800
	pub fn cv_dnn_SignLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:730
	pub fn cv_dnn_SinLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:736
	pub fn cv_dnn_SinhLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// sliceRanges /usr/include/opencv2/dnn/all_layers.hpp:480
	pub fn cv_dnn_SliceLayer_getPropSliceRanges_const(instance: *const c_void) -> *mut c_void;
	// sliceRanges /usr/include/opencv2/dnn/all_layers.hpp:480
	pub fn cv_dnn_SliceLayer_setPropSliceRanges_vector_vector_Range__(instance: *mut c_void, val: *mut c_void);
	// sliceSteps /usr/include/opencv2/dnn/all_layers.hpp:481
	pub fn cv_dnn_SliceLayer_getPropSliceSteps_const(instance: *const c_void) -> *mut c_void;
	// sliceSteps /usr/include/opencv2/dnn/all_layers.hpp:481
	pub fn cv_dnn_SliceLayer_setPropSliceSteps_vector_vector_int__(instance: *mut c_void, val: *mut c_void);
	// axis /usr/include/opencv2/dnn/all_layers.hpp:482
	pub fn cv_dnn_SliceLayer_getPropAxis_const(instance: *const c_void) -> i32;
	// axis /usr/include/opencv2/dnn/all_layers.hpp:482
	pub fn cv_dnn_SliceLayer_setPropAxis_int(instance: *mut c_void, val: i32);
	// num_split /usr/include/opencv2/dnn/all_layers.hpp:483
	pub fn cv_dnn_SliceLayer_getPropNum_split_const(instance: *const c_void) -> i32;
	// num_split /usr/include/opencv2/dnn/all_layers.hpp:483
	pub fn cv_dnn_SliceLayer_setPropNum_split_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:485
	pub fn cv_dnn_SliceLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// logSoftMax /usr/include/opencv2/dnn/all_layers.hpp:346
	pub fn cv_dnn_SoftmaxLayer_getPropLogSoftMax_const(instance: *const c_void) -> bool;
	// logSoftMax /usr/include/opencv2/dnn/all_layers.hpp:346
	pub fn cv_dnn_SoftmaxLayer_setPropLogSoftMax_bool(instance: *mut c_void, val: bool);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:348
	pub fn cv_dnn_SoftmaxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:354
	pub fn cv_dnn_SoftmaxLayerInt8_getPropOutput_sc_const(instance: *const c_void) -> f32;
	// output_sc /usr/include/opencv2/dnn/all_layers.hpp:354
	pub fn cv_dnn_SoftmaxLayerInt8_setPropOutput_sc_float(instance: *mut c_void, val: f32);
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:355
	pub fn cv_dnn_SoftmaxLayerInt8_getPropOutput_zp_const(instance: *const c_void) -> i32;
	// output_zp /usr/include/opencv2/dnn/all_layers.hpp:355
	pub fn cv_dnn_SoftmaxLayerInt8_setPropOutput_zp_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:356
	pub fn cv_dnn_SoftmaxLayerInt8_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:742
	pub fn cv_dnn_SoftplusLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:748
	pub fn cv_dnn_SoftsignLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// outputsCount /usr/include/opencv2/dnn/all_layers.hpp:442
	pub fn cv_dnn_SplitLayer_getPropOutputsCount_const(instance: *const c_void) -> i32;
	// outputsCount /usr/include/opencv2/dnn/all_layers.hpp:442
	pub fn cv_dnn_SplitLayer_setPropOutputsCount_int(instance: *mut c_void, val: i32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:444
	pub fn cv_dnn_SplitLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:658
	pub fn cv_dnn_SqrtLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:588
	pub fn cv_dnn_SwishLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:582
	pub fn cv_dnn_TanHLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:754
	pub fn cv_dnn_TanLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detect(cv::InputArray, std::vector<std::vector<Point>> &, std::vector<float> &) /usr/include/opencv2/dnn/dnn.hpp:1606
	pub fn cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vector_vector_Point__R_vector_float_R(instance: *const c_void, frame: *const c_void, detections: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, std::vector<std::vector<Point>> &) /usr/include/opencv2/dnn/dnn.hpp:1614
	pub fn cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vector_vector_Point__R(instance: *const c_void, frame: *const c_void, detections: *mut c_void, ocvrs_return: *mut Result_void);
	// detectTextRectangles(cv::InputArray, std::vector<cv::RotatedRect> &, std::vector<float> &) /usr/include/opencv2/dnn/dnn.hpp:1632
	pub fn cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vector_RotatedRect_R_vector_float_R(instance: *const c_void, frame: *const c_void, detections: *mut c_void, confidences: *mut c_void, ocvrs_return: *mut Result_void);
	// detectTextRectangles(cv::InputArray, std::vector<cv::RotatedRect> &) /usr/include/opencv2/dnn/dnn.hpp:1640
	pub fn cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vector_RotatedRect_R(instance: *const c_void, frame: *const c_void, detections: *mut c_void, ocvrs_return: *mut Result_void);
	// TextDetectionModel_DB() /usr/include/opencv2/dnn/dnn.hpp:1717
	pub fn cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB(ocvrs_return: *mut Result<*mut c_void>);
	// TextDetectionModel_DB(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1723
	pub fn cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// TextDetectionModel_DB(const std::string &, const std::string &) /usr/include/opencv2/dnn/dnn.hpp:1732
	pub fn cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR_const_stringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// setBinaryThreshold(float) /usr/include/opencv2/dnn/dnn.hpp:1735
	pub fn cv_dnn_TextDetectionModel_DB_setBinaryThreshold_float(instance: *mut c_void, binary_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// getBinaryThreshold() /usr/include/opencv2/dnn/dnn.hpp:1736
	pub fn cv_dnn_TextDetectionModel_DB_getBinaryThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setPolygonThreshold(float) /usr/include/opencv2/dnn/dnn.hpp:1738
	pub fn cv_dnn_TextDetectionModel_DB_setPolygonThreshold_float(instance: *mut c_void, polygon_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// getPolygonThreshold() /usr/include/opencv2/dnn/dnn.hpp:1739
	pub fn cv_dnn_TextDetectionModel_DB_getPolygonThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setUnclipRatio(double) /usr/include/opencv2/dnn/dnn.hpp:1741
	pub fn cv_dnn_TextDetectionModel_DB_setUnclipRatio_double(instance: *mut c_void, unclip_ratio: f64, ocvrs_return: *mut Result<*mut c_void>);
	// getUnclipRatio() /usr/include/opencv2/dnn/dnn.hpp:1742
	pub fn cv_dnn_TextDetectionModel_DB_getUnclipRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxCandidates(int) /usr/include/opencv2/dnn/dnn.hpp:1744
	pub fn cv_dnn_TextDetectionModel_DB_setMaxCandidates_int(instance: *mut c_void, max_candidates: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getMaxCandidates() /usr/include/opencv2/dnn/dnn.hpp:1745
	pub fn cv_dnn_TextDetectionModel_DB_getMaxCandidates_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// TextDetectionModel_EAST() /usr/include/opencv2/dnn/dnn.hpp:1656
	pub fn cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST(ocvrs_return: *mut Result<*mut c_void>);
	// TextDetectionModel_EAST(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1662
	pub fn cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// TextDetectionModel_EAST(const std::string &, const std::string &) /usr/include/opencv2/dnn/dnn.hpp:1671
	pub fn cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR_const_stringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// setConfidenceThreshold(float) /usr/include/opencv2/dnn/dnn.hpp:1679
	pub fn cv_dnn_TextDetectionModel_EAST_setConfidenceThreshold_float(instance: *mut c_void, conf_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// getConfidenceThreshold() /usr/include/opencv2/dnn/dnn.hpp:1685
	pub fn cv_dnn_TextDetectionModel_EAST_getConfidenceThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setNMSThreshold(float) /usr/include/opencv2/dnn/dnn.hpp:1692
	pub fn cv_dnn_TextDetectionModel_EAST_setNMSThreshold_float(instance: *mut c_void, nms_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// getNMSThreshold() /usr/include/opencv2/dnn/dnn.hpp:1698
	pub fn cv_dnn_TextDetectionModel_EAST_getNMSThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// TextRecognitionModel() /usr/include/opencv2/dnn/dnn.hpp:1500
	pub fn cv_dnn_TextRecognitionModel_TextRecognitionModel(ocvrs_return: *mut Result<*mut c_void>);
	// TextRecognitionModel(const cv::dnn::Net &) /usr/include/opencv2/dnn/dnn.hpp:1507
	pub fn cv_dnn_TextRecognitionModel_TextRecognitionModel_const_NetR(network: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// TextRecognitionModel(const std::string &, const std::string &) /usr/include/opencv2/dnn/dnn.hpp:1516
	pub fn cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR_const_stringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// setDecodeType(const std::string &) /usr/include/opencv2/dnn/dnn.hpp:1526
	pub fn cv_dnn_TextRecognitionModel_setDecodeType_const_stringR(instance: *mut c_void, decode_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getDecodeType() /usr/include/opencv2/dnn/dnn.hpp:1533
	pub fn cv_dnn_TextRecognitionModel_getDecodeType_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setDecodeOptsCTCPrefixBeamSearch(int, int) /usr/include/opencv2/dnn/dnn.hpp:1542
	pub fn cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int_int(instance: *mut c_void, beam_size: i32, voc_prune_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// setVocabulary(const std::vector<std::string> &) /usr/include/opencv2/dnn/dnn.hpp:1549
	pub fn cv_dnn_TextRecognitionModel_setVocabulary_const_vector_string_R(instance: *mut c_void, vocabulary: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getVocabulary() /usr/include/opencv2/dnn/dnn.hpp:1556
	pub fn cv_dnn_TextRecognitionModel_getVocabulary_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// recognize(cv::InputArray) /usr/include/opencv2/dnn/dnn.hpp:1564
	pub fn cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR(instance: *const c_void, frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// recognize(cv::InputArray, cv::InputArrayOfArrays, std::vector<std::string> &) /usr/include/opencv2/dnn/dnn.hpp:1573
	pub fn cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR_const__InputArrayR_vector_string_R(instance: *const c_void, frame: *const c_void, roi_rects: *const c_void, results: *mut c_void, ocvrs_return: *mut Result_void);
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:786
	pub fn cv_dnn_ThresholdedReluLayer_getPropAlpha_const(instance: *const c_void) -> f32;
	// alpha /usr/include/opencv2/dnn/all_layers.hpp:786
	pub fn cv_dnn_ThresholdedReluLayer_setPropAlpha_float(instance: *mut c_void, val: f32);
	// create(const cv::dnn::LayerParams &) /usr/include/opencv2/dnn/all_layers.hpp:788
	pub fn cv_dnn_ThresholdedReluLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _Range(const cv::Range &) /usr/include/opencv2/dnn/shape_utils.hpp:59
	pub fn cv_dnn__Range__Range_const_RangeR(r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _Range(int, int) /usr/include/opencv2/dnn/shape_utils.hpp:60
	pub fn cv_dnn__Range__Range_int_int(start_: i32, size_: i32, ocvrs_return: *mut Result<*mut c_void>);
}
