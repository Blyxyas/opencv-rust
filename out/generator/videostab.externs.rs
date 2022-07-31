extern "C" {
	// calcBlurriness(const cv::Mat &) /usr/include/opencv2/videostab/deblurring.hpp:57
	pub fn cv_videostab_calcBlurriness_const_MatR(frame: *const c_void, ocvrs_return: *mut Result<f32>);
	// calcFlowMask(const cv::Mat &, const cv::Mat &, const cv::Mat &, float, const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:199
	pub fn cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(flow_x: *const c_void, flow_y: *const c_void, errors: *const c_void, max_error: f32, mask0: *const c_void, mask1: *const c_void, flow_mask: *mut c_void, ocvrs_return: *mut Result_void);
	// completeFrameAccordingToFlow(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, float, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:203
	pub fn cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(flow_mask: *const c_void, flow_x: *const c_void, flow_y: *const c_void, frame1: *const c_void, mask1: *const c_void, dist_thresh: f32, frame0: *mut c_void, mask0: *mut c_void, ocvrs_return: *mut Result_void);
	// ensureInclusionConstraint(const cv::Mat &, cv::Size, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:165
	pub fn cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(m: *const c_void, size: *const core::Size, trim_ratio: f32, ocvrs_return: *mut Result<*mut c_void>);
	// estimateGlobalMotionLeastSquares(cv::InputOutputArray, cv::InputOutputArray, int, float *) /usr/include/opencv2/videostab/global_motion.hpp:77
	pub fn cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(points0: *const c_void, points1: *const c_void, model: i32, rmse: *mut f32, ocvrs_return: *mut Result<*mut c_void>);
	// estimateGlobalMotionRansac(cv::InputArray, cv::InputArray, int, const cv::videostab::RansacParams &, float *, int *) /usr/include/opencv2/videostab/global_motion.hpp:90
	pub fn cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(points0: *const c_void, points1: *const c_void, model: i32, params: *const c_void, rmse: *mut f32, ninliers: *mut i32, ocvrs_return: *mut Result<*mut c_void>);
	// estimateOptimalTrimRatio(const cv::Mat &, cv::Size) /usr/include/opencv2/videostab/motion_stabilizing.hpp:167
	pub fn cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(m: *const c_void, size: *const core::Size, ocvrs_return: *mut Result<f32>);
	// getMotion(int, int, const std::vector<Mat> &) /usr/include/opencv2/videostab/global_motion.hpp:304
	pub fn cv_videostab_getMotion_int_int_const_vector_Mat_R(from: i32, to: i32, motions: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:177
	pub fn cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// ColorInpainter(int, double) /usr/include/opencv2/videostab/inpainting.hpp:186
	pub fn cv_videostab_ColorInpainter_ColorInpainter_int_double(method: i32, radius: f64, ocvrs_return: *mut Result<*mut c_void>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:188
	pub fn cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// ConsistentMosaicInpainter() /usr/include/opencv2/videostab/inpainting.hpp:130
	pub fn cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter(ocvrs_return: *mut Result<*mut c_void>);
	// setStdevThresh(float) /usr/include/opencv2/videostab/inpainting.hpp:132
	pub fn cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// stdevThresh() /usr/include/opencv2/videostab/inpainting.hpp:133
	pub fn cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:135
	pub fn cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// setRadius(int) /usr/include/opencv2/videostab/deblurring.hpp:66
	pub fn cv_videostab_DeblurerBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// radius() /usr/include/opencv2/videostab/deblurring.hpp:67
	pub fn cv_videostab_DeblurerBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:69
	pub fn cv_videostab_DeblurerBase_deblur_int_MatR_const_RangeR(instance: *mut c_void, idx: i32, frame: *mut c_void, range: *const c_void, ocvrs_return: *mut Result_void);
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/deblurring.hpp:74
	pub fn cv_videostab_DeblurerBase_setFrames_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// frames() /usr/include/opencv2/videostab/deblurring.hpp:75
	pub fn cv_videostab_DeblurerBase_frames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/deblurring.hpp:77
	pub fn cv_videostab_DeblurerBase_setMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// motions() /usr/include/opencv2/videostab/deblurring.hpp:78
	pub fn cv_videostab_DeblurerBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setBlurrinessRates(const std::vector<float> &) /usr/include/opencv2/videostab/deblurring.hpp:80
	pub fn cv_videostab_DeblurerBase_setBlurrinessRates_const_vector_float_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// blurrinessRates() /usr/include/opencv2/videostab/deblurring.hpp:81
	pub fn cv_videostab_DeblurerBase_blurrinessRates_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// FastMarchingMethod() /usr/include/opencv2/videostab/fast_marching.hpp:66
	pub fn cv_videostab_FastMarchingMethod_FastMarchingMethod(ocvrs_return: *mut Result<*mut c_void>);
	// distanceMap() /usr/include/opencv2/videostab/fast_marching.hpp:81
	pub fn cv_videostab_FastMarchingMethod_distanceMap_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// FromFileMotionReader(const cv::String &) /usr/include/opencv2/videostab/global_motion.hpp:201
	pub fn cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:203
	pub fn cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// GaussianMotionFilter(int, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:100
	pub fn cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(radius: i32, stdev: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setParams(int, float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:102
	pub fn cv_videostab_GaussianMotionFilter_setParams_int_float(instance: *mut c_void, radius: i32, stdev: f32, ocvrs_return: *mut Result_void);
	// radius() /usr/include/opencv2/videostab/motion_stabilizing.hpp:103
	pub fn cv_videostab_GaussianMotionFilter_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// stdev() /usr/include/opencv2/videostab/motion_stabilizing.hpp:104
	pub fn cv_videostab_GaussianMotionFilter_stdev_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &) /usr/include/opencv2/videostab/motion_stabilizing.hpp:106
	pub fn cv_videostab_GaussianMotionFilter_stabilize_int_const_vector_Mat_R_const_RangeR(instance: *mut c_void, idx: i32, motions: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:74
	pub fn cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow_x: *const c_void, flow_y: *const c_void, errors: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:61
	pub fn cv_videostab_IFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:62
	pub fn cv_videostab_IFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:60
	pub fn cv_videostab_ILog_print_const_charX(instance: *mut c_void, format: *const c_char, ocvrs_return: *mut Result_void);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:65
	pub fn cv_videostab_IMotionStabilizer_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result_void);
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:63
	pub fn cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:65
	pub fn cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *const c_void, status: *const c_void, errors: *const c_void, ocvrs_return: *mut Result_void);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:180
	pub fn cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:181
	pub fn cv_videostab_ImageMotionEstimatorBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:183
	pub fn cv_videostab_ImageMotionEstimatorBase_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:189
	pub fn cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// setRadius(int) /usr/include/opencv2/videostab/inpainting.hpp:70
	pub fn cv_videostab_InpainterBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// radius() /usr/include/opencv2/videostab/inpainting.hpp:71
	pub fn cv_videostab_InpainterBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/inpainting.hpp:73
	pub fn cv_videostab_InpainterBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/inpainting.hpp:74
	pub fn cv_videostab_InpainterBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:76
	pub fn cv_videostab_InpainterBase_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:81
	pub fn cv_videostab_InpainterBase_setFrames_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// frames() /usr/include/opencv2/videostab/inpainting.hpp:82
	pub fn cv_videostab_InpainterBase_frames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:84
	pub fn cv_videostab_InpainterBase_setMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// motions() /usr/include/opencv2/videostab/inpainting.hpp:85
	pub fn cv_videostab_InpainterBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setStabilizedFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:87
	pub fn cv_videostab_InpainterBase_setStabilizedFrames_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// stabilizedFrames() /usr/include/opencv2/videostab/inpainting.hpp:88
	pub fn cv_videostab_InpainterBase_stabilizedFrames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:90
	pub fn cv_videostab_InpainterBase_setStabilizationMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// stabilizationMotions() /usr/include/opencv2/videostab/inpainting.hpp:91
	pub fn cv_videostab_InpainterBase_stabilizationMotions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// pushBack(Ptr<cv::videostab::InpainterBase>) /usr/include/opencv2/videostab/inpainting.hpp:111
	pub fn cv_videostab_InpaintingPipeline_pushBack_Ptr_InpainterBase_(instance: *mut c_void, inpainter: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/videostab/inpainting.hpp:112
	pub fn cv_videostab_InpaintingPipeline_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setRadius(int) /usr/include/opencv2/videostab/inpainting.hpp:114
	pub fn cv_videostab_InpaintingPipeline_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/inpainting.hpp:115
	pub fn cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// setFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:116
	pub fn cv_videostab_InpaintingPipeline_setFrames_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:117
	pub fn cv_videostab_InpaintingPipeline_setMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// setStabilizedFrames(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:118
	pub fn cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/inpainting.hpp:119
	pub fn cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:121
	pub fn cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// KeypointBasedMotionEstimator(Ptr<cv::videostab::MotionEstimatorBase>) /usr/include/opencv2/videostab/global_motion.hpp:232
	pub fn cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_Ptr_MotionEstimatorBase_(estimator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:234
	pub fn cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:235
	pub fn cv_videostab_KeypointBasedMotionEstimator_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// setDetector(Ptr<cv::FeatureDetector>) /usr/include/opencv2/videostab/global_motion.hpp:237
	pub fn cv_videostab_KeypointBasedMotionEstimator_setDetector_Ptr_Feature2D_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// detector() /usr/include/opencv2/videostab/global_motion.hpp:238
	pub fn cv_videostab_KeypointBasedMotionEstimator_detector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setOpticalFlowEstimator(Ptr<cv::videostab::ISparseOptFlowEstimator>) /usr/include/opencv2/videostab/global_motion.hpp:240
	pub fn cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_Ptr_ISparseOptFlowEstimator_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// opticalFlowEstimator() /usr/include/opencv2/videostab/global_motion.hpp:241
	pub fn cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setOutlierRejector(Ptr<cv::videostab::IOutlierRejector>) /usr/include/opencv2/videostab/global_motion.hpp:243
	pub fn cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_Ptr_IOutlierRejector_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// outlierRejector() /usr/include/opencv2/videostab/global_motion.hpp:244
	pub fn cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:246
	pub fn cv_videostab_KeypointBasedMotionEstimator_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:248
	pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:249
	pub fn cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:72
	pub fn cv_videostab_LogToStdout_print_const_charX(instance: *mut c_void, format: *const c_char, ocvrs_return: *mut Result_void);
	// LpMotionStabilizer(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_stabilizing.hpp:120
	pub fn cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_stabilizing.hpp:122
	pub fn cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/motion_stabilizing.hpp:123
	pub fn cv_videostab_LpMotionStabilizer_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// setFrameSize(cv::Size) /usr/include/opencv2/videostab/motion_stabilizing.hpp:125
	pub fn cv_videostab_LpMotionStabilizer_setFrameSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result_void);
	// frameSize() /usr/include/opencv2/videostab/motion_stabilizing.hpp:126
	pub fn cv_videostab_LpMotionStabilizer_frameSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setTrimRatio(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:128
	pub fn cv_videostab_LpMotionStabilizer_setTrimRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// trimRatio() /usr/include/opencv2/videostab/motion_stabilizing.hpp:129
	pub fn cv_videostab_LpMotionStabilizer_trimRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeight1(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:131
	pub fn cv_videostab_LpMotionStabilizer_setWeight1_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// weight1() /usr/include/opencv2/videostab/motion_stabilizing.hpp:132
	pub fn cv_videostab_LpMotionStabilizer_weight1_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeight2(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:134
	pub fn cv_videostab_LpMotionStabilizer_setWeight2_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// weight2() /usr/include/opencv2/videostab/motion_stabilizing.hpp:135
	pub fn cv_videostab_LpMotionStabilizer_weight2_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeight3(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:137
	pub fn cv_videostab_LpMotionStabilizer_setWeight3_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// weight3() /usr/include/opencv2/videostab/motion_stabilizing.hpp:138
	pub fn cv_videostab_LpMotionStabilizer_weight3_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeight4(float) /usr/include/opencv2/videostab/motion_stabilizing.hpp:140
	pub fn cv_videostab_LpMotionStabilizer_setWeight4_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// weight4() /usr/include/opencv2/videostab/motion_stabilizing.hpp:141
	pub fn cv_videostab_LpMotionStabilizer_weight4_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:143
	pub fn cv_videostab_LpMotionStabilizer_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result_void);
	// MaskFrameSource(const Ptr<cv::videostab::IFrameSource> &) /usr/include/opencv2/videostab/frame_source.hpp:92
	pub fn cv_videostab_MaskFrameSource_MaskFrameSource_const_Ptr_IFrameSource_R(source: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:94
	pub fn cv_videostab_MaskFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:95
	pub fn cv_videostab_MaskFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:116
	pub fn cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result_void);
	// setPeriod(int) /usr/include/opencv2/videostab/wobble_suppression.hpp:104
	pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// period() /usr/include/opencv2/videostab/wobble_suppression.hpp:105
	pub fn cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:106
	pub fn cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:111
	pub fn cv_videostab_MotionEstimatorBase_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:120
	pub fn cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// MotionEstimatorL1(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:156
	pub fn cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:158
	pub fn cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// MotionEstimatorRansacL2(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:134
	pub fn cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
	// setRansacParams(const cv::videostab::RansacParams &) /usr/include/opencv2/videostab/global_motion.hpp:136
	pub fn cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// ransacParams() /usr/include/opencv2/videostab/global_motion.hpp:137
	pub fn cv_videostab_MotionEstimatorRansacL2_ransacParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMinInlierRatio(float) /usr/include/opencv2/videostab/global_motion.hpp:139
	pub fn cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// minInlierRatio() /usr/include/opencv2/videostab/global_motion.hpp:140
	pub fn cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// estimate(cv::InputArray, cv::InputArray, bool *) /usr/include/opencv2/videostab/global_motion.hpp:142
	pub fn cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(instance: *mut c_void, points0: *const c_void, points1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &) /usr/include/opencv2/videostab/motion_stabilizing.hpp:89
	pub fn cv_videostab_MotionFilterBase_stabilize_int_const_vector_Mat_R_const_RangeR(instance: *mut c_void, idx: i32, motions: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:92
	pub fn cv_videostab_MotionFilterBase_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result_void);
	// MotionInpainter() /usr/include/opencv2/videostab/inpainting.hpp:144
	pub fn cv_videostab_MotionInpainter_MotionInpainter(ocvrs_return: *mut Result<*mut c_void>);
	// setOptFlowEstimator(Ptr<cv::videostab::IDenseOptFlowEstimator>) /usr/include/opencv2/videostab/inpainting.hpp:146
	pub fn cv_videostab_MotionInpainter_setOptFlowEstimator_Ptr_IDenseOptFlowEstimator_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// optFlowEstimator() /usr/include/opencv2/videostab/inpainting.hpp:147
	pub fn cv_videostab_MotionInpainter_optFlowEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setFlowErrorThreshold(float) /usr/include/opencv2/videostab/inpainting.hpp:149
	pub fn cv_videostab_MotionInpainter_setFlowErrorThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// flowErrorThreshold() /usr/include/opencv2/videostab/inpainting.hpp:150
	pub fn cv_videostab_MotionInpainter_flowErrorThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setDistThreshold(float) /usr/include/opencv2/videostab/inpainting.hpp:152
	pub fn cv_videostab_MotionInpainter_setDistThreshold_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// distThresh() /usr/include/opencv2/videostab/inpainting.hpp:153
	pub fn cv_videostab_MotionInpainter_distThresh_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setBorderMode(int) /usr/include/opencv2/videostab/inpainting.hpp:155
	pub fn cv_videostab_MotionInpainter_setBorderMode_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// borderMode() /usr/include/opencv2/videostab/inpainting.hpp:156
	pub fn cv_videostab_MotionInpainter_borderMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:158
	pub fn cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, idx: i32, frame: *mut c_void, mask: *mut c_void, ocvrs_return: *mut Result_void);
	// pushBack(Ptr<cv::videostab::IMotionStabilizer>) /usr/include/opencv2/videostab/motion_stabilizing.hpp:73
	pub fn cv_videostab_MotionStabilizationPipeline_pushBack_Ptr_IMotionStabilizer_(instance: *mut c_void, stabilizer: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/videostab/motion_stabilizing.hpp:74
	pub fn cv_videostab_MotionStabilizationPipeline_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// stabilize(int, const std::vector<Mat> &, const cv::Range &, cv::Mat *) /usr/include/opencv2/videostab/motion_stabilizing.hpp:76
	pub fn cv_videostab_MotionStabilizationPipeline_stabilize_int_const_vector_Mat_R_const_RangeR_MatX(instance: *mut c_void, size: i32, motions: *const c_void, range: *const c_void, stabilization_motions: *mut c_void, ocvrs_return: *mut Result_void);
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:93
	pub fn cv_videostab_NullDeblurer_deblur_int_MatR_const_RangeR(instance: *mut c_void, unnamed: i32, unnamed_1: *mut c_void, unnamed_2: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:68
	pub fn cv_videostab_NullFrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:69
	pub fn cv_videostab_NullFrameSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// inpaint(int, cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/inpainting.hpp:105
	pub fn cv_videostab_NullInpainter_inpaint_int_MatR_MatR(instance: *mut c_void, unnamed: i32, unnamed_1: *mut c_void, unnamed_2: *mut c_void, ocvrs_return: *mut Result_void);
	// print(const char *, ...) /usr/include/opencv2/videostab/log.hpp:66
	pub fn cv_videostab_NullLog_print_const_charX(instance: *mut c_void, unnamed: *const c_char, ocvrs_return: *mut Result_void);
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:70
	pub fn cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:98
	pub fn cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result_void);
	// OnePassStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:146
	pub fn cv_videostab_OnePassStabilizer_OnePassStabilizer(ocvrs_return: *mut Result<*mut c_void>);
	// setMotionFilter(Ptr<cv::videostab::MotionFilterBase>) /usr/include/opencv2/videostab/stabilizer.hpp:148
	pub fn cv_videostab_OnePassStabilizer_setMotionFilter_Ptr_MotionFilterBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// motionFilter() /usr/include/opencv2/videostab/stabilizer.hpp:149
	pub fn cv_videostab_OnePassStabilizer_motionFilter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reset() /usr/include/opencv2/videostab/stabilizer.hpp:151
	pub fn cv_videostab_OnePassStabilizer_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/stabilizer.hpp:152
	pub fn cv_videostab_OnePassStabilizer_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PyrLkOptFlowEstimatorBase() /usr/include/opencv2/videostab/optical_flow.hpp:82
	pub fn cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase(ocvrs_return: *mut Result<*mut c_void>);
	// setWinSize(cv::Size) /usr/include/opencv2/videostab/optical_flow.hpp:84
	pub fn cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result_void);
	// winSize() /usr/include/opencv2/videostab/optical_flow.hpp:85
	pub fn cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setMaxLevel(int) /usr/include/opencv2/videostab/optical_flow.hpp:87
	pub fn cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// maxLevel() /usr/include/opencv2/videostab/optical_flow.hpp:88
	pub fn cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// size /usr/include/opencv2/videostab/motion_core.hpp:75
	pub fn cv_videostab_RansacParams_getPropSize_const(instance: *const c_void) -> i32;
	// size /usr/include/opencv2/videostab/motion_core.hpp:75
	pub fn cv_videostab_RansacParams_setPropSize_int(instance: *mut c_void, val: i32);
	// thresh /usr/include/opencv2/videostab/motion_core.hpp:76
	pub fn cv_videostab_RansacParams_getPropThresh_const(instance: *const c_void) -> f32;
	// thresh /usr/include/opencv2/videostab/motion_core.hpp:76
	pub fn cv_videostab_RansacParams_setPropThresh_float(instance: *mut c_void, val: f32);
	// eps /usr/include/opencv2/videostab/motion_core.hpp:77
	pub fn cv_videostab_RansacParams_getPropEps_const(instance: *const c_void) -> f32;
	// eps /usr/include/opencv2/videostab/motion_core.hpp:77
	pub fn cv_videostab_RansacParams_setPropEps_float(instance: *mut c_void, val: f32);
	// prob /usr/include/opencv2/videostab/motion_core.hpp:78
	pub fn cv_videostab_RansacParams_getPropProb_const(instance: *const c_void) -> f32;
	// prob /usr/include/opencv2/videostab/motion_core.hpp:78
	pub fn cv_videostab_RansacParams_setPropProb_float(instance: *mut c_void, val: f32);
	// RansacParams() /usr/include/opencv2/videostab/motion_core.hpp:80
	pub fn cv_videostab_RansacParams_RansacParams(ocvrs_return: *mut Result<*mut c_void>);
	// RansacParams(int, float, float, float) /usr/include/opencv2/videostab/motion_core.hpp:87
	pub fn cv_videostab_RansacParams_RansacParams_int_float_float_float(size: i32, thresh: f32, eps: f32, prob: f32, ocvrs_return: *mut Result<*mut c_void>);
	// niters() /usr/include/opencv2/videostab/motion_core.hpp:92
	pub fn cv_videostab_RansacParams_niters_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// default2dMotion(cv::videostab::MotionModel) /usr/include/opencv2/videostab/motion_core.hpp:102
	pub fn cv_videostab_RansacParams_default2dMotion_MotionModel(model: crate::videostab::MotionModel, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/videostab/optical_flow.hpp:100
	pub fn cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, points0: *const c_void, points1: *const c_void, status: *const c_void, errors: *const c_void, ocvrs_return: *mut Result_void);
	// setLog(Ptr<cv::videostab::ILog>) /usr/include/opencv2/videostab/stabilizer.hpp:71
	pub fn cv_videostab_StabilizerBase_setLog_Ptr_ILog_(instance: *mut c_void, ilog: *mut c_void, ocvrs_return: *mut Result_void);
	// log() /usr/include/opencv2/videostab/stabilizer.hpp:72
	pub fn cv_videostab_StabilizerBase_log_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setRadius(int) /usr/include/opencv2/videostab/stabilizer.hpp:74
	pub fn cv_videostab_StabilizerBase_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// radius() /usr/include/opencv2/videostab/stabilizer.hpp:75
	pub fn cv_videostab_StabilizerBase_radius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFrameSource(Ptr<cv::videostab::IFrameSource>) /usr/include/opencv2/videostab/stabilizer.hpp:77
	pub fn cv_videostab_StabilizerBase_setFrameSource_Ptr_IFrameSource_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// frameSource() /usr/include/opencv2/videostab/stabilizer.hpp:78
	pub fn cv_videostab_StabilizerBase_frameSource_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMaskSource(const Ptr<cv::videostab::IFrameSource> &) /usr/include/opencv2/videostab/stabilizer.hpp:80
	pub fn cv_videostab_StabilizerBase_setMaskSource_const_Ptr_IFrameSource_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// maskSource() /usr/include/opencv2/videostab/stabilizer.hpp:81
	pub fn cv_videostab_StabilizerBase_maskSource_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotionEstimator(Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/stabilizer.hpp:83
	pub fn cv_videostab_StabilizerBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// motionEstimator() /usr/include/opencv2/videostab/stabilizer.hpp:84
	pub fn cv_videostab_StabilizerBase_motionEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setDeblurer(Ptr<cv::videostab::DeblurerBase>) /usr/include/opencv2/videostab/stabilizer.hpp:86
	pub fn cv_videostab_StabilizerBase_setDeblurer_Ptr_DeblurerBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// deblurrer() /usr/include/opencv2/videostab/stabilizer.hpp:87
	pub fn cv_videostab_StabilizerBase_deblurrer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setTrimRatio(float) /usr/include/opencv2/videostab/stabilizer.hpp:89
	pub fn cv_videostab_StabilizerBase_setTrimRatio_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// trimRatio() /usr/include/opencv2/videostab/stabilizer.hpp:90
	pub fn cv_videostab_StabilizerBase_trimRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setCorrectionForInclusion(bool) /usr/include/opencv2/videostab/stabilizer.hpp:92
	pub fn cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// doCorrectionForInclusion() /usr/include/opencv2/videostab/stabilizer.hpp:93
	pub fn cv_videostab_StabilizerBase_doCorrectionForInclusion_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setBorderMode(int) /usr/include/opencv2/videostab/stabilizer.hpp:95
	pub fn cv_videostab_StabilizerBase_setBorderMode_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// borderMode() /usr/include/opencv2/videostab/stabilizer.hpp:96
	pub fn cv_videostab_StabilizerBase_borderMode_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setInpainter(Ptr<cv::videostab::InpainterBase>) /usr/include/opencv2/videostab/stabilizer.hpp:98
	pub fn cv_videostab_StabilizerBase_setInpainter_Ptr_InpainterBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// inpainter() /usr/include/opencv2/videostab/stabilizer.hpp:99
	pub fn cv_videostab_StabilizerBase_inpainter_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ToFileMotionWriter(const cv::String &, Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/global_motion.hpp:212
	pub fn cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_Ptr_ImageMotionEstimatorBase_(path: *const c_char, estimator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotionModel(cv::videostab::MotionModel) /usr/include/opencv2/videostab/global_motion.hpp:214
	pub fn cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(instance: *mut c_void, val: crate::videostab::MotionModel, ocvrs_return: *mut Result_void);
	// motionModel() /usr/include/opencv2/videostab/global_motion.hpp:215
	pub fn cv_videostab_ToFileMotionWriter_motionModel_const(instance: *const c_void, ocvrs_return: *mut Result<crate::videostab::MotionModel>);
	// setFrameMask(cv::InputArray) /usr/include/opencv2/videostab/global_motion.hpp:217
	pub fn cv_videostab_ToFileMotionWriter_setFrameMask_const__InputArrayR(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// estimate(const cv::Mat &, const cv::Mat &, bool *) /usr/include/opencv2/videostab/global_motion.hpp:219
	pub fn cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, ok: *mut bool, ocvrs_return: *mut Result<*mut c_void>);
	// TranslationBasedLocalOutlierRejector() /usr/include/opencv2/videostab/outlier_rejection.hpp:77
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector(ocvrs_return: *mut Result<*mut c_void>);
	// setCellSize(cv::Size) /usr/include/opencv2/videostab/outlier_rejection.hpp:79
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(instance: *mut c_void, val: *const core::Size, ocvrs_return: *mut Result_void);
	// cellSize() /usr/include/opencv2/videostab/outlier_rejection.hpp:80
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// setRansacParams(cv::videostab::RansacParams) /usr/include/opencv2/videostab/outlier_rejection.hpp:82
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// ransacParams() /usr/include/opencv2/videostab/outlier_rejection.hpp:83
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// process(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/videostab/outlier_rejection.hpp:85
	pub fn cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, frame_size: *const core::Size, points0: *const c_void, points1: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// TwoPassStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:166
	pub fn cv_videostab_TwoPassStabilizer_TwoPassStabilizer(ocvrs_return: *mut Result<*mut c_void>);
	// setMotionStabilizer(Ptr<cv::videostab::IMotionStabilizer>) /usr/include/opencv2/videostab/stabilizer.hpp:168
	pub fn cv_videostab_TwoPassStabilizer_setMotionStabilizer_Ptr_IMotionStabilizer_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// motionStabilizer() /usr/include/opencv2/videostab/stabilizer.hpp:169
	pub fn cv_videostab_TwoPassStabilizer_motionStabilizer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setWobbleSuppressor(Ptr<cv::videostab::WobbleSuppressorBase>) /usr/include/opencv2/videostab/stabilizer.hpp:171
	pub fn cv_videostab_TwoPassStabilizer_setWobbleSuppressor_Ptr_WobbleSuppressorBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// wobbleSuppressor() /usr/include/opencv2/videostab/stabilizer.hpp:172
	pub fn cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setEstimateTrimRatio(bool) /usr/include/opencv2/videostab/stabilizer.hpp:174
	pub fn cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// mustEstimateTrimaRatio() /usr/include/opencv2/videostab/stabilizer.hpp:175
	pub fn cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// reset() /usr/include/opencv2/videostab/stabilizer.hpp:177
	pub fn cv_videostab_TwoPassStabilizer_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/stabilizer.hpp:178
	pub fn cv_videostab_TwoPassStabilizer_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// VideoFileSource(const cv::String &, bool) /usr/include/opencv2/videostab/frame_source.hpp:75
	pub fn cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(path: *const c_char, volatile_frame: bool, ocvrs_return: *mut Result<*mut c_void>);
	// reset() /usr/include/opencv2/videostab/frame_source.hpp:77
	pub fn cv_videostab_VideoFileSource_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// nextFrame() /usr/include/opencv2/videostab/frame_source.hpp:78
	pub fn cv_videostab_VideoFileSource_nextFrame(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// width() /usr/include/opencv2/videostab/frame_source.hpp:80
	pub fn cv_videostab_VideoFileSource_width(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// height() /usr/include/opencv2/videostab/frame_source.hpp:81
	pub fn cv_videostab_VideoFileSource_height(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// count() /usr/include/opencv2/videostab/frame_source.hpp:82
	pub fn cv_videostab_VideoFileSource_count(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// fps() /usr/include/opencv2/videostab/frame_source.hpp:83
	pub fn cv_videostab_VideoFileSource_fps(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// WeightingDeblurer() /usr/include/opencv2/videostab/deblurring.hpp:99
	pub fn cv_videostab_WeightingDeblurer_WeightingDeblurer(ocvrs_return: *mut Result<*mut c_void>);
	// setSensitivity(float) /usr/include/opencv2/videostab/deblurring.hpp:101
	pub fn cv_videostab_WeightingDeblurer_setSensitivity_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// sensitivity() /usr/include/opencv2/videostab/deblurring.hpp:102
	pub fn cv_videostab_WeightingDeblurer_sensitivity_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// deblur(int, cv::Mat &, const cv::Range &) /usr/include/opencv2/videostab/deblurring.hpp:104
	pub fn cv_videostab_WeightingDeblurer_deblur_int_MatR_const_RangeR(instance: *mut c_void, idx: i32, frame: *mut c_void, range: *const c_void, ocvrs_return: *mut Result_void);
	// setMotionEstimator(Ptr<cv::videostab::ImageMotionEstimatorBase>) /usr/include/opencv2/videostab/wobble_suppression.hpp:67
	pub fn cv_videostab_WobbleSuppressorBase_setMotionEstimator_Ptr_ImageMotionEstimatorBase_(instance: *mut c_void, val: *mut c_void, ocvrs_return: *mut Result_void);
	// motionEstimator() /usr/include/opencv2/videostab/wobble_suppression.hpp:68
	pub fn cv_videostab_WobbleSuppressorBase_motionEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// suppress(int, const cv::Mat &, cv::Mat &) /usr/include/opencv2/videostab/wobble_suppression.hpp:70
	pub fn cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(instance: *mut c_void, idx: i32, frame: *const c_void, result: *mut c_void, ocvrs_return: *mut Result_void);
	// setFrameCount(int) /usr/include/opencv2/videostab/wobble_suppression.hpp:75
	pub fn cv_videostab_WobbleSuppressorBase_setFrameCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// frameCount() /usr/include/opencv2/videostab/wobble_suppression.hpp:76
	pub fn cv_videostab_WobbleSuppressorBase_frameCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:78
	pub fn cv_videostab_WobbleSuppressorBase_setMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// motions() /usr/include/opencv2/videostab/wobble_suppression.hpp:79
	pub fn cv_videostab_WobbleSuppressorBase_motions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMotions2(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:81
	pub fn cv_videostab_WobbleSuppressorBase_setMotions2_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// motions2() /usr/include/opencv2/videostab/wobble_suppression.hpp:82
	pub fn cv_videostab_WobbleSuppressorBase_motions2_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setStabilizationMotions(const std::vector<Mat> &) /usr/include/opencv2/videostab/wobble_suppression.hpp:84
	pub fn cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vector_Mat_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// stabilizationMotions() /usr/include/opencv2/videostab/wobble_suppression.hpp:85
	pub fn cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
}
