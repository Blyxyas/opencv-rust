extern "C" {
	// calcGlobalOrientation(cv::InputArray, cv::InputArray, cv::InputArray, double, double) /usr/include/opencv2/optflow/motempl.hpp:119
	pub fn cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation: *const c_void, mask: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result<f64>);
	// calcMotionGradient(cv::InputArray, cv::OutputArray, cv::OutputArray, double, double, int) /usr/include/opencv2/optflow/motempl.hpp:102
	pub fn cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi: *const c_void, mask: *const c_void, orientation: *const c_void, delta1: f64, delta2: f64, aperture_size: i32, ocvrs_return: *mut Result_void);
	// segmentMotion(cv::InputArray, cv::OutputArray, std::vector<Rect> &, double, double) /usr/include/opencv2/optflow/motempl.hpp:137
	pub fn cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vector_Rect_R_double_double(mhi: *const c_void, segmask: *const c_void, bounding_rects: *mut c_void, timestamp: f64, seg_thresh: f64, ocvrs_return: *mut Result_void);
	// updateMotionHistory(cv::InputArray, cv::InputOutputArray, double, double) /usr/include/opencv2/optflow/motempl.hpp:71
	pub fn cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette: *const c_void, mhi: *const c_void, timestamp: f64, duration: f64, ocvrs_return: *mut Result_void);
	// calcOpticalFlowDenseRLOF(cv::InputArray, cv::InputArray, cv::InputOutputArray, Ptr<cv::optflow::RLOFOpticalFlowParameter>, float, cv::Size, cv::optflow::InterpolationType, int, float, float, int, int, bool, float, float, bool) /usr/include/opencv2/optflow/rlofflow.hpp:501
	pub fn cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(i0: *const c_void, i1: *const c_void, flow: *const c_void, rlof_param: *mut c_void, forward_backward_threshold: f32, grid_step: *const core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool, ocvrs_return: *mut Result_void);
	// calcOpticalFlowSF(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/optflow.hpp:81
	pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, ocvrs_return: *mut Result_void);
	// calcOpticalFlowSF(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int, double, double, int, double, double, double, int, double, double, double) /usr/include/opencv2/optflow.hpp:110
	pub fn cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from: *const c_void, to: *const c_void, flow: *const c_void, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64, ocvrs_return: *mut Result_void);
	// calcOpticalFlowSparseRLOF(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, Ptr<cv::optflow::RLOFOpticalFlowParameter>, float) /usr/include/opencv2/optflow/rlofflow.hpp:538
	pub fn cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_RLOFOpticalFlowParameter__float(prev_img: *const c_void, next_img: *const c_void, prev_pts: *const c_void, next_pts: *const c_void, status: *const c_void, err: *const c_void, rlof_param: *mut c_void, forward_backward_threshold: f32, ocvrs_return: *mut Result_void);
	// calcOpticalFlowSparseToDense(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool, float, float) /usr/include/opencv2/optflow.hpp:135
	pub fn cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from: *const c_void, to: *const c_void, flow: *const c_void, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, ocvrs_return: *mut Result_void);
	// createOptFlow_DeepFlow() /usr/include/opencv2/optflow.hpp:165
	pub fn cv_optflow_createOptFlow_DeepFlow(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_DenseRLOF() /usr/include/opencv2/optflow/rlofflow.hpp:545
	pub fn cv_optflow_createOptFlow_DenseRLOF(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_DualTVL1() /usr/include/opencv2/optflow.hpp:300
	pub fn cv_optflow_createOptFlow_DualTVL1(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_Farneback() /usr/include/opencv2/optflow.hpp:171
	pub fn cv_optflow_createOptFlow_Farneback(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_PCAFlow() /usr/include/opencv2/optflow/pcaflow.hpp:142
	pub fn cv_optflow_createOptFlow_PCAFlow(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_SimpleFlow() /usr/include/opencv2/optflow.hpp:168
	pub fn cv_optflow_createOptFlow_SimpleFlow(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_SparseRLOF() /usr/include/opencv2/optflow/rlofflow.hpp:548
	pub fn cv_optflow_createOptFlow_SparseRLOF(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_SparseToDense() /usr/include/opencv2/optflow.hpp:174
	pub fn cv_optflow_createOptFlow_SparseToDense(ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &, optflow::GPCTree::Node &, optflow::GPCTree::Node) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:369
	pub fn cv_read_const_FileNodeR_NodeR_Node(fn_: *const c_void, node: *mut crate::optflow::GPCTree_Node, unnamed: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const optflow::GPCTree::Node &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:367
	pub fn cv_write_FileStorageR_const_StringR_const_NodeR(fs: *mut c_void, name: *const c_char, node: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result_void);
	// setRLOFOpticalFlowParameter(Ptr<cv::optflow::RLOFOpticalFlowParameter>) /usr/include/opencv2/optflow/rlofflow.hpp:240
	pub fn cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// getRLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:244
	pub fn cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setForwardBackward(float) /usr/include/opencv2/optflow/rlofflow.hpp:253
	pub fn cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getForwardBackward() /usr/include/opencv2/optflow/rlofflow.hpp:257
	pub fn cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// getGridStep() /usr/include/opencv2/optflow/rlofflow.hpp:263
	pub fn cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setGridStep(cv::Size) /usr/include/opencv2/optflow/rlofflow.hpp:267
	pub fn cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result_void);
	// setInterpolation(cv::optflow::InterpolationType) /usr/include/opencv2/optflow/rlofflow.hpp:275
	pub fn cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(instance: *mut c_void, val: crate::optflow::InterpolationType, ocvrs_return: *mut Result_void);
	// getInterpolation() /usr/include/opencv2/optflow/rlofflow.hpp:279
	pub fn cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::InterpolationType>);
	// getEPICK() /usr/include/opencv2/optflow/rlofflow.hpp:285
	pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setEPICK(int) /usr/include/opencv2/optflow/rlofflow.hpp:289
	pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getEPICSigma() /usr/include/opencv2/optflow/rlofflow.hpp:296
	pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setEPICSigma(float) /usr/include/opencv2/optflow/rlofflow.hpp:300
	pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getEPICLambda() /usr/include/opencv2/optflow/rlofflow.hpp:306
	pub fn cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setEPICLambda(float) /usr/include/opencv2/optflow/rlofflow.hpp:310
	pub fn cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getFgsLambda() /usr/include/opencv2/optflow/rlofflow.hpp:315
	pub fn cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setFgsLambda(float) /usr/include/opencv2/optflow/rlofflow.hpp:319
	pub fn cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getFgsSigma() /usr/include/opencv2/optflow/rlofflow.hpp:324
	pub fn cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setFgsSigma(float) /usr/include/opencv2/optflow/rlofflow.hpp:328
	pub fn cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// setUsePostProc(bool) /usr/include/opencv2/optflow/rlofflow.hpp:333
	pub fn cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUsePostProc() /usr/include/opencv2/optflow/rlofflow.hpp:337
	pub fn cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseVariationalRefinement(bool) /usr/include/opencv2/optflow/rlofflow.hpp:342
	pub fn cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseVariationalRefinement() /usr/include/opencv2/optflow/rlofflow.hpp:346
	pub fn cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setRICSPSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:351
	pub fn cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getRICSPSize() /usr/include/opencv2/optflow/rlofflow.hpp:355
	pub fn cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRICSLICType(int) /usr/include/opencv2/optflow/rlofflow.hpp:362
	pub fn cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getRICSLICType() /usr/include/opencv2/optflow/rlofflow.hpp:366
	pub fn cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// create(Ptr<cv::optflow::RLOFOpticalFlowParameter>, float, cv::Size, cv::optflow::InterpolationType, int, float, float, int, int, bool, float, float, bool) /usr/include/opencv2/optflow/rlofflow.hpp:383
	pub fn cv_optflow_DenseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(rlof_param: *mut c_void, forward_backward_threshold: f32, grid_step: *const core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool, ocvrs_return: *mut Result<*mut c_void>);
	// getTau() /usr/include/opencv2/optflow.hpp:223
	pub fn cv_optflow_DualTVL1OpticalFlow_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTau(double) /usr/include/opencv2/optflow.hpp:225
	pub fn cv_optflow_DualTVL1OpticalFlow_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/optflow.hpp:228
	pub fn cv_optflow_DualTVL1OpticalFlow_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setLambda(double) /usr/include/opencv2/optflow.hpp:230
	pub fn cv_optflow_DualTVL1OpticalFlow_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTheta() /usr/include/opencv2/optflow.hpp:233
	pub fn cv_optflow_DualTVL1OpticalFlow_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTheta(double) /usr/include/opencv2/optflow.hpp:235
	pub fn cv_optflow_DualTVL1OpticalFlow_setTheta_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/optflow.hpp:238
	pub fn cv_optflow_DualTVL1OpticalFlow_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setGamma(double) /usr/include/opencv2/optflow.hpp:240
	pub fn cv_optflow_DualTVL1OpticalFlow_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getScalesNumber() /usr/include/opencv2/optflow.hpp:243
	pub fn cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setScalesNumber(int) /usr/include/opencv2/optflow.hpp:245
	pub fn cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWarpingsNumber() /usr/include/opencv2/optflow.hpp:248
	pub fn cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWarpingsNumber(int) /usr/include/opencv2/optflow.hpp:250
	pub fn cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getEpsilon() /usr/include/opencv2/optflow.hpp:253
	pub fn cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setEpsilon(double) /usr/include/opencv2/optflow.hpp:255
	pub fn cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getInnerIterations() /usr/include/opencv2/optflow.hpp:258
	pub fn cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setInnerIterations(int) /usr/include/opencv2/optflow.hpp:260
	pub fn cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getOuterIterations() /usr/include/opencv2/optflow.hpp:263
	pub fn cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setOuterIterations(int) /usr/include/opencv2/optflow.hpp:265
	pub fn cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getUseInitialFlow() /usr/include/opencv2/optflow.hpp:268
	pub fn cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseInitialFlow(bool) /usr/include/opencv2/optflow.hpp:270
	pub fn cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getScaleStep() /usr/include/opencv2/optflow.hpp:273
	pub fn cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setScaleStep(double) /usr/include/opencv2/optflow.hpp:275
	pub fn cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMedianFiltering() /usr/include/opencv2/optflow.hpp:278
	pub fn cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMedianFiltering(int) /usr/include/opencv2/optflow.hpp:280
	pub fn cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// create(double, double, double, int, int, double, int, int, double, double, int, bool) /usr/include/opencv2/optflow.hpp:283
	pub fn cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool, ocvrs_return: *mut Result<*mut c_void>);
	// getAllDescriptorsForImage(const cv::Mat *, std::vector<GPCPatchDescriptor> &, const cv::optflow::GPCMatchingParams &, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:306
	pub fn cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_R_const_GPCMatchingParamsR_int(img_ch: *const c_void, descr: *mut c_void, mp: *const crate::optflow::GPCMatchingParams, typ: i32, ocvrs_return: *mut Result_void);
	// getCoordinatesFromIndex(size_t, cv::Size, int &, int &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:309
	pub fn cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index: size_t, sz: *const core::Size, x: *mut i32, y: *mut i32, ocvrs_return: *mut Result_void);
	// GPCMatchingParams(bool) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:147
	pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl: bool, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
	// GPCMatchingParams(const cv::optflow::GPCMatchingParams &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:149
	pub fn cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(params: *const crate::optflow::GPCMatchingParams, ocvrs_return: *mut Result<crate::optflow::GPCMatchingParams>);
	// feature /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	pub fn cv_optflow_GPCPatchDescriptor_getPropFeature_const(instance: *const c_void, ocvrs_return: *mut core::VecN<f64, 18>);
	// feature /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:68
	pub fn cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(instance: *mut c_void, val: *const core::VecN<f64, 18>);
	// dot(const Vec<double, nFeatures> &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:70
	pub fn cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_R(instance: *const c_void, coef: *const core::VecN<f64, 18>, ocvrs_return: *mut Result<f64>);
	// markAsSeparated() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:72
	pub fn cv_optflow_GPCPatchDescriptor_markAsSeparated(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// isSeparated() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:74
	pub fn cv_optflow_GPCPatchDescriptor_isSeparated_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// ref /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	pub fn cv_optflow_GPCPatchSample_getPropRef_const(instance: *const c_void) -> *mut c_void;
	// ref /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:79
	pub fn cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(instance: *mut c_void, val: *mut c_void);
	// pos /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	pub fn cv_optflow_GPCPatchSample_getPropPos_const(instance: *const c_void) -> *mut c_void;
	// pos /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:80
	pub fn cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(instance: *mut c_void, val: *mut c_void);
	// neg /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	pub fn cv_optflow_GPCPatchSample_getPropNeg_const(instance: *const c_void) -> *mut c_void;
	// neg /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:81
	pub fn cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(instance: *mut c_void, val: *mut c_void);
	// getDirections(bool &, bool &, bool &, const Vec<double, GPCPatchDescriptor::nFeatures> &, double) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:83
	pub fn cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_R_double(instance: *const c_void, refdir: *mut bool, posdir: *mut bool, negdir: *mut bool, coef: *const core::VecN<f64, 18>, rhs: f64, ocvrs_return: *mut Result_void);
	// GPCTrainingParams(unsigned int, int, cv::optflow::GPCDescType, bool) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:130
	pub fn cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool, ocvrs_return: *mut Result<crate::optflow::GPCTrainingParams>);
	// check() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:138
	pub fn cv_optflow_GPCTrainingParams_check_const(instance: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result<bool>);
	// create(const std::vector<String> &, const std::vector<String> &, const std::vector<String> &, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:108
	pub fn cv_optflow_GPCTrainingSamples_create_const_vector_String_R_const_vector_String_R_const_vector_String_R_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::InputArrayOfArrays, int) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:111
	pub fn cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from: *const c_void, images_to: *const c_void, gt: *const c_void, descriptor_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// size() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:114
	pub fn cv_optflow_GPCTrainingSamples_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// type() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:116
	pub fn cv_optflow_GPCTrainingSamples_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// train(cv::optflow::GPCTrainingSamples &, const cv::optflow::GPCTrainingParams) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:176
	pub fn cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(instance: *mut c_void, samples: *mut c_void, params: *const crate::optflow::GPCTrainingParams, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:178
	pub fn cv_optflow_GPCTree_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:180
	pub fn cv_optflow_GPCTree_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// findLeafForPatch(const cv::optflow::GPCPatchDescriptor &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:182
	pub fn cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(instance: *const c_void, descr: *const c_void, ocvrs_return: *mut Result<u32>);
	// create() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:184
	pub fn cv_optflow_GPCTree_create(ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::optflow::GPCTree &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:186
	pub fn cv_optflow_GPCTree_operatorEQ_const_const_GPCTreeR(instance: *const c_void, t: *const c_void, ocvrs_return: *mut Result<bool>);
	// getDescriptorType() /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:188
	pub fn cv_optflow_GPCTree_getDescriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// operator==(const cv::optflow::GPCTree::Node &) /usr/include/opencv2/optflow/sparse_matching_gpc.hpp:164
	pub fn cv_optflow_GPCTree_Node_operatorEQ_const_const_NodeR(instance: *const crate::optflow::GPCTree_Node, n: *const crate::optflow::GPCTree_Node, ocvrs_return: *mut Result<bool>);
	// OpticalFlowPCAFlow(Ptr<const cv::optflow::PCAPrior>, const cv::Size, float, float, float, float, float) /usr/include/opencv2/optflow/pcaflow.hpp:116
	pub fn cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_const_PCAPrior__const_Size_float_float_float_float_float(_prior: *const c_void, _basis_size: *const core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32, ocvrs_return: *mut Result<*mut c_void>);
	// calc(cv::InputArray, cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/optflow/pcaflow.hpp:120
	pub fn cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(instance: *mut c_void, i0: *const c_void, i1: *const c_void, flow: *const c_void, ocvrs_return: *mut Result_void);
	// collectGarbage() /usr/include/opencv2/optflow/pcaflow.hpp:121
	pub fn cv_optflow_OpticalFlowPCAFlow_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// PCAPrior(const char *) /usr/include/opencv2/optflow/pcaflow.hpp:83
	pub fn cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getPadding() /usr/include/opencv2/optflow/pcaflow.hpp:85
	pub fn cv_optflow_PCAPrior_getPadding_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getBasisSize() /usr/include/opencv2/optflow/pcaflow.hpp:87
	pub fn cv_optflow_PCAPrior_getBasisSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// fillConstraints(float *, float *, float *, float *) /usr/include/opencv2/optflow/pcaflow.hpp:89
	pub fn cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(instance: *const c_void, a1: *mut f32, a2: *mut f32, b1: *mut f32, b2: *mut f32, ocvrs_return: *mut Result_void);
	// solverType /usr/include/opencv2/optflow/rlofflow.hpp:83
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropSolverType_const(instance: *const c_void, ocvrs_return: *mut crate::optflow::SolverType);
	// solverType /usr/include/opencv2/optflow/rlofflow.hpp:83
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropSolverType_SolverType(instance: *mut c_void, val: crate::optflow::SolverType);
	// supportRegionType /usr/include/opencv2/optflow/rlofflow.hpp:88
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropSupportRegionType_const(instance: *const c_void, ocvrs_return: *mut crate::optflow::SupportRegionType);
	// supportRegionType /usr/include/opencv2/optflow/rlofflow.hpp:88
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropSupportRegionType_SupportRegionType(instance: *mut c_void, val: crate::optflow::SupportRegionType);
	// normSigma0 /usr/include/opencv2/optflow/rlofflow.hpp:92
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma0_const(instance: *const c_void) -> f32;
	// normSigma0 /usr/include/opencv2/optflow/rlofflow.hpp:92
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma0_float(instance: *mut c_void, val: f32);
	// normSigma1 /usr/include/opencv2/optflow/rlofflow.hpp:98
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma1_const(instance: *const c_void) -> f32;
	// normSigma1 /usr/include/opencv2/optflow/rlofflow.hpp:98
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma1_float(instance: *mut c_void, val: f32);
	// smallWinSize /usr/include/opencv2/optflow/rlofflow.hpp:104
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropSmallWinSize_const(instance: *const c_void) -> i32;
	// smallWinSize /usr/include/opencv2/optflow/rlofflow.hpp:104
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropSmallWinSize_int(instance: *mut c_void, val: i32);
	// largeWinSize /usr/include/opencv2/optflow/rlofflow.hpp:107
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropLargeWinSize_const(instance: *const c_void) -> i32;
	// largeWinSize /usr/include/opencv2/optflow/rlofflow.hpp:107
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropLargeWinSize_int(instance: *mut c_void, val: i32);
	// crossSegmentationThreshold /usr/include/opencv2/optflow/rlofflow.hpp:112
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropCrossSegmentationThreshold_const(instance: *const c_void) -> i32;
	// crossSegmentationThreshold /usr/include/opencv2/optflow/rlofflow.hpp:112
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropCrossSegmentationThreshold_int(instance: *mut c_void, val: i32);
	// maxLevel /usr/include/opencv2/optflow/rlofflow.hpp:117
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropMaxLevel_const(instance: *const c_void) -> i32;
	// maxLevel /usr/include/opencv2/optflow/rlofflow.hpp:117
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropMaxLevel_int(instance: *mut c_void, val: i32);
	// useInitialFlow /usr/include/opencv2/optflow/rlofflow.hpp:122
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropUseInitialFlow_const(instance: *const c_void) -> bool;
	// useInitialFlow /usr/include/opencv2/optflow/rlofflow.hpp:122
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropUseInitialFlow_bool(instance: *mut c_void, val: bool);
	// useIlluminationModel /usr/include/opencv2/optflow/rlofflow.hpp:126
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropUseIlluminationModel_const(instance: *const c_void) -> bool;
	// useIlluminationModel /usr/include/opencv2/optflow/rlofflow.hpp:126
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropUseIlluminationModel_bool(instance: *mut c_void, val: bool);
	// useGlobalMotionPrior /usr/include/opencv2/optflow/rlofflow.hpp:134
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropUseGlobalMotionPrior_const(instance: *const c_void) -> bool;
	// useGlobalMotionPrior /usr/include/opencv2/optflow/rlofflow.hpp:134
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropUseGlobalMotionPrior_bool(instance: *mut c_void, val: bool);
	// maxIteration /usr/include/opencv2/optflow/rlofflow.hpp:139
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropMaxIteration_const(instance: *const c_void) -> i32;
	// maxIteration /usr/include/opencv2/optflow/rlofflow.hpp:139
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropMaxIteration_int(instance: *mut c_void, val: i32);
	// minEigenValue /usr/include/opencv2/optflow/rlofflow.hpp:143
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropMinEigenValue_const(instance: *const c_void) -> f32;
	// minEigenValue /usr/include/opencv2/optflow/rlofflow.hpp:143
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropMinEigenValue_float(instance: *mut c_void, val: f32);
	// globalMotionRansacThreshold /usr/include/opencv2/optflow/rlofflow.hpp:147
	pub fn cv_optflow_RLOFOpticalFlowParameter_getPropGlobalMotionRansacThreshold_const(instance: *const c_void) -> f32;
	// globalMotionRansacThreshold /usr/include/opencv2/optflow/rlofflow.hpp:147
	pub fn cv_optflow_RLOFOpticalFlowParameter_setPropGlobalMotionRansacThreshold_float(instance: *mut c_void, val: f32);
	// RLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:66
	pub fn cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter(ocvrs_return: *mut Result<*mut c_void>);
	// setUseMEstimator(bool) /usr/include/opencv2/optflow/rlofflow.hpp:160
	pub fn cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// setSolverType(cv::optflow::SolverType) /usr/include/opencv2/optflow/rlofflow.hpp:162
	pub fn cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(instance: *mut c_void, val: crate::optflow::SolverType, ocvrs_return: *mut Result_void);
	// getSolverType() /usr/include/opencv2/optflow/rlofflow.hpp:163
	pub fn cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::SolverType>);
	// setSupportRegionType(cv::optflow::SupportRegionType) /usr/include/opencv2/optflow/rlofflow.hpp:165
	pub fn cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(instance: *mut c_void, val: crate::optflow::SupportRegionType, ocvrs_return: *mut Result_void);
	// getSupportRegionType() /usr/include/opencv2/optflow/rlofflow.hpp:166
	pub fn cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(instance: *const c_void, ocvrs_return: *mut Result<crate::optflow::SupportRegionType>);
	// setNormSigma0(float) /usr/include/opencv2/optflow/rlofflow.hpp:168
	pub fn cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getNormSigma0() /usr/include/opencv2/optflow/rlofflow.hpp:169
	pub fn cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setNormSigma1(float) /usr/include/opencv2/optflow/rlofflow.hpp:171
	pub fn cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getNormSigma1() /usr/include/opencv2/optflow/rlofflow.hpp:172
	pub fn cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSmallWinSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:174
	pub fn cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getSmallWinSize() /usr/include/opencv2/optflow/rlofflow.hpp:175
	pub fn cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setLargeWinSize(int) /usr/include/opencv2/optflow/rlofflow.hpp:177
	pub fn cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getLargeWinSize() /usr/include/opencv2/optflow/rlofflow.hpp:178
	pub fn cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCrossSegmentationThreshold(int) /usr/include/opencv2/optflow/rlofflow.hpp:180
	pub fn cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getCrossSegmentationThreshold() /usr/include/opencv2/optflow/rlofflow.hpp:181
	pub fn cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxLevel(int) /usr/include/opencv2/optflow/rlofflow.hpp:183
	pub fn cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxLevel() /usr/include/opencv2/optflow/rlofflow.hpp:184
	pub fn cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setUseInitialFlow(bool) /usr/include/opencv2/optflow/rlofflow.hpp:186
	pub fn cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseInitialFlow() /usr/include/opencv2/optflow/rlofflow.hpp:187
	pub fn cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseIlluminationModel(bool) /usr/include/opencv2/optflow/rlofflow.hpp:189
	pub fn cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseIlluminationModel() /usr/include/opencv2/optflow/rlofflow.hpp:190
	pub fn cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseGlobalMotionPrior(bool) /usr/include/opencv2/optflow/rlofflow.hpp:192
	pub fn cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getUseGlobalMotionPrior() /usr/include/opencv2/optflow/rlofflow.hpp:193
	pub fn cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setMaxIteration(int) /usr/include/opencv2/optflow/rlofflow.hpp:195
	pub fn cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxIteration() /usr/include/opencv2/optflow/rlofflow.hpp:196
	pub fn cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinEigenValue(float) /usr/include/opencv2/optflow/rlofflow.hpp:198
	pub fn cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getMinEigenValue() /usr/include/opencv2/optflow/rlofflow.hpp:199
	pub fn cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setGlobalMotionRansacThreshold(float) /usr/include/opencv2/optflow/rlofflow.hpp:201
	pub fn cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getGlobalMotionRansacThreshold() /usr/include/opencv2/optflow/rlofflow.hpp:202
	pub fn cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// create() /usr/include/opencv2/optflow/rlofflow.hpp:205
	pub fn cv_optflow_RLOFOpticalFlowParameter_create(ocvrs_return: *mut Result<*mut c_void>);
	// setRLOFOpticalFlowParameter(Ptr<cv::optflow::RLOFOpticalFlowParameter>) /usr/include/opencv2/optflow/rlofflow.hpp:417
	pub fn cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// getRLOFOpticalFlowParameter() /usr/include/opencv2/optflow/rlofflow.hpp:421
	pub fn cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setForwardBackward(float) /usr/include/opencv2/optflow/rlofflow.hpp:430
	pub fn cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getForwardBackward() /usr/include/opencv2/optflow/rlofflow.hpp:434
	pub fn cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// create(Ptr<cv::optflow::RLOFOpticalFlowParameter>, float) /usr/include/opencv2/optflow/rlofflow.hpp:441
	pub fn cv_optflow_SparseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float(rlof_param: *mut c_void, forward_backward_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
}
