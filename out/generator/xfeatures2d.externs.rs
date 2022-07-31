extern "C" {
	// FASTForPointSet(cv::InputArray, std::vector<KeyPoint> &, int, bool, cv::FastFeatureDetector::DetectorType) /usr/include/opencv2/xfeatures2d.hpp:1043
	pub fn cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image: *const c_void, keypoints: *mut c_void, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType, ocvrs_return: *mut Result_void);
	// matchGMS(const cv::Size &, const cv::Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double) /usr/include/opencv2/xfeatures2d.hpp:1068
	pub fn cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_DMatch_R_vector_DMatch_R_const_bool_const_bool_const_double(size1: *const core::Size, size2: *const core::Size, keypoints1: *const c_void, keypoints2: *const c_void, matches1to2: *const c_void, matches_gms: *mut c_void, with_rotation: bool, with_scale: bool, threshold_factor: f64, ocvrs_return: *mut Result_void);
	// matchLOGOS(const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<int> &, const std::vector<int> &, std::vector<DMatch> &) /usr/include/opencv2/xfeatures2d.hpp:1083
	pub fn cv_xfeatures2d_matchLOGOS_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_int_R_const_vector_int_R_vector_DMatch_R(keypoints1: *const c_void, keypoints2: *const c_void, nn1: *const c_void, nn2: *const c_void, matches1to2: *mut c_void, ocvrs_return: *mut Result_void);
	// hessianThreshold /usr/include/opencv2/xfeatures2d/cuda.hpp:182
	pub fn cv_cuda_SURF_CUDA_getPropHessianThreshold_const(instance: *const c_void) -> f64;
	// hessianThreshold /usr/include/opencv2/xfeatures2d/cuda.hpp:182
	pub fn cv_cuda_SURF_CUDA_setPropHessianThreshold_double(instance: *mut c_void, val: f64);
	// nOctaves /usr/include/opencv2/xfeatures2d/cuda.hpp:183
	pub fn cv_cuda_SURF_CUDA_getPropNOctaves_const(instance: *const c_void) -> i32;
	// nOctaves /usr/include/opencv2/xfeatures2d/cuda.hpp:183
	pub fn cv_cuda_SURF_CUDA_setPropNOctaves_int(instance: *mut c_void, val: i32);
	// nOctaveLayers /usr/include/opencv2/xfeatures2d/cuda.hpp:184
	pub fn cv_cuda_SURF_CUDA_getPropNOctaveLayers_const(instance: *const c_void) -> i32;
	// nOctaveLayers /usr/include/opencv2/xfeatures2d/cuda.hpp:184
	pub fn cv_cuda_SURF_CUDA_setPropNOctaveLayers_int(instance: *mut c_void, val: i32);
	// extended /usr/include/opencv2/xfeatures2d/cuda.hpp:185
	pub fn cv_cuda_SURF_CUDA_getPropExtended_const(instance: *const c_void) -> bool;
	// extended /usr/include/opencv2/xfeatures2d/cuda.hpp:185
	pub fn cv_cuda_SURF_CUDA_setPropExtended_bool(instance: *mut c_void, val: bool);
	// upright /usr/include/opencv2/xfeatures2d/cuda.hpp:186
	pub fn cv_cuda_SURF_CUDA_getPropUpright_const(instance: *const c_void) -> bool;
	// upright /usr/include/opencv2/xfeatures2d/cuda.hpp:186
	pub fn cv_cuda_SURF_CUDA_setPropUpright_bool(instance: *mut c_void, val: bool);
	// keypointsRatio /usr/include/opencv2/xfeatures2d/cuda.hpp:189
	pub fn cv_cuda_SURF_CUDA_getPropKeypointsRatio_const(instance: *const c_void) -> f32;
	// keypointsRatio /usr/include/opencv2/xfeatures2d/cuda.hpp:189
	pub fn cv_cuda_SURF_CUDA_setPropKeypointsRatio_float(instance: *mut c_void, val: f32);
	// sum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_getPropSum_const(instance: *const c_void) -> *mut c_void;
	// sum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_setPropSum_GpuMat(instance: *mut c_void, val: *mut c_void);
	// mask1 /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_getPropMask1_const(instance: *const c_void) -> *mut c_void;
	// mask1 /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_setPropMask1_GpuMat(instance: *mut c_void, val: *mut c_void);
	// maskSum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_getPropMaskSum_const(instance: *const c_void) -> *mut c_void;
	// maskSum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	pub fn cv_cuda_SURF_CUDA_setPropMaskSum_GpuMat(instance: *mut c_void, val: *mut c_void);
	// det /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	pub fn cv_cuda_SURF_CUDA_getPropDet_const(instance: *const c_void) -> *mut c_void;
	// det /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	pub fn cv_cuda_SURF_CUDA_setPropDet_GpuMat(instance: *mut c_void, val: *mut c_void);
	// trace /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	pub fn cv_cuda_SURF_CUDA_getPropTrace_const(instance: *const c_void) -> *mut c_void;
	// trace /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	pub fn cv_cuda_SURF_CUDA_setPropTrace_GpuMat(instance: *mut c_void, val: *mut c_void);
	// maxPosBuffer /usr/include/opencv2/xfeatures2d/cuda.hpp:195
	pub fn cv_cuda_SURF_CUDA_getPropMaxPosBuffer_const(instance: *const c_void) -> *mut c_void;
	// maxPosBuffer /usr/include/opencv2/xfeatures2d/cuda.hpp:195
	pub fn cv_cuda_SURF_CUDA_setPropMaxPosBuffer_GpuMat(instance: *mut c_void, val: *mut c_void);
	// SURF_CUDA() /usr/include/opencv2/xfeatures2d/cuda.hpp:102
	pub fn cv_cuda_SURF_CUDA_SURF_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// SURF_CUDA(double, int, int, bool, float, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:104
	pub fn cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(double, int, int, bool, float, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:117
	pub fn cv_cuda_SURF_CUDA_create_double_int_int_bool_float_bool(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool, ocvrs_return: *mut Result<*mut c_void>);
	// descriptorSize() /usr/include/opencv2/xfeatures2d/cuda.hpp:121
	pub fn cv_cuda_SURF_CUDA_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// defaultNorm() /usr/include/opencv2/xfeatures2d/cuda.hpp:123
	pub fn cv_cuda_SURF_CUDA_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// uploadKeypoints(const std::vector<KeyPoint> &, cv::cuda::GpuMat &) /usr/include/opencv2/xfeatures2d/cuda.hpp:126
	pub fn cv_cuda_SURF_CUDA_uploadKeypoints_const_vector_KeyPoint_R_GpuMatR(instance: *mut c_void, keypoints: *const c_void, keypoints_gpu: *mut c_void, ocvrs_return: *mut Result_void);
	// downloadKeypoints(const cv::cuda::GpuMat &, std::vector<KeyPoint> &) /usr/include/opencv2/xfeatures2d/cuda.hpp:128
	pub fn cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vector_KeyPoint_R(instance: *mut c_void, keypoints_gpu: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result_void);
	// downloadDescriptors(const cv::cuda::GpuMat &, std::vector<float> &) /usr/include/opencv2/xfeatures2d/cuda.hpp:131
	pub fn cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vector_float_R(instance: *mut c_void, descriptors_gpu: *const c_void, descriptors: *mut c_void, ocvrs_return: *mut Result_void);
	// detect(const cv::cuda::GpuMat &, const cv::cuda::GpuMat &, cv::cuda::GpuMat &) /usr/include/opencv2/xfeatures2d/cuda.hpp:155
	pub fn cv_cuda_SURF_CUDA_detect_const_GpuMatR_const_GpuMatR_GpuMatR(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result_void);
	// detectWithDescriptors(const cv::cuda::GpuMat &, const cv::cuda::GpuMat &, cv::cuda::GpuMat &, cv::cuda::GpuMat &, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:171
	pub fn cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(instance: *mut c_void, img: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *mut c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result_void);
	// releaseMemory() /usr/include/opencv2/xfeatures2d/cuda.hpp:179
	pub fn cv_cuda_SURF_CUDA_releaseMemory(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>) /usr/include/opencv2/xfeatures2d.hpp:956
	pub fn cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D__Ptr_Feature2D_(keypoint_detector: *mut c_void, descriptor_extractor: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(Ptr<cv::FeatureDetector>) /usr/include/opencv2/xfeatures2d.hpp:964
	pub fn cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D_(keypoint_detector: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detect(cv::InputArray, std::vector<Elliptic_KeyPoint> &, cv::InputArray) /usr/include/opencv2/xfeatures2d.hpp:975
	pub fn cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__InputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// detectAndCompute(cv::InputArray, cv::InputArray, std::vector<Elliptic_KeyPoint> &, cv::OutputArray, bool) /usr/include/opencv2/xfeatures2d.hpp:985
	pub fn cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__OutputArrayR_bool(instance: *mut c_void, image: *const c_void, mask: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, use_provided_keypoints: bool, ocvrs_return: *mut Result_void);
	// create(float, int) /usr/include/opencv2/xfeatures2d.hpp:224
	pub fn cv_xfeatures2d_BEBLID_create_float_int(scale_factor: f32, n_bits: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, bool, float) /usr/include/opencv2/xfeatures2d.hpp:419
	pub fn cv_xfeatures2d_BoostDesc_create_int_bool_float(desc: i32, use_scale_orientation: bool, scale_factor: f32, ocvrs_return: *mut Result<*mut c_void>);
	// setUseScaleOrientation(const bool) /usr/include/opencv2/xfeatures2d.hpp:422
	pub fn cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(instance: *mut c_void, use_scale_orientation: bool, ocvrs_return: *mut Result_void);
	// getUseScaleOrientation() /usr/include/opencv2/xfeatures2d.hpp:423
	pub fn cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setScaleFactor(const float) /usr/include/opencv2/xfeatures2d.hpp:425
	pub fn cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(instance: *mut c_void, scale_factor: f32, ocvrs_return: *mut Result_void);
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:426
	pub fn cv_xfeatures2d_BoostDesc_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// create(int, bool) /usr/include/opencv2/xfeatures2d.hpp:133
	pub fn cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes: i32, use_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(float, int, int, int, DAISY::NormalizationType, cv::InputArray, bool, bool) /usr/include/opencv2/xfeatures2d.hpp:250
	pub fn cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayR_bool_bool(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: crate::xfeatures2d::DAISY_NormalizationType, h: *const c_void, interpolation: bool, use_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:259
	pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::OutputArrayOfArrays) /usr/include/opencv2/xfeatures2d.hpp:261
	pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(instance: *mut c_void, images: *const c_void, keypoints: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, cv::Rect, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:270
	pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(instance: *mut c_void, image: *const c_void, roi: *const core::Rect, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:276
	pub fn cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, descriptors: *const c_void, ocvrs_return: *mut Result_void);
	// GetDescriptor(double, double, int, float *) /usr/include/opencv2/xfeatures2d.hpp:284
	pub fn cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, ocvrs_return: *mut Result_void);
	// GetDescriptor(double, double, int, float *, double *) /usr/include/opencv2/xfeatures2d.hpp:293
	pub fn cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, h: *mut f64, ocvrs_return: *mut Result<bool>);
	// GetUnnormalizedDescriptor(double, double, int, float *) /usr/include/opencv2/xfeatures2d.hpp:301
	pub fn cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, ocvrs_return: *mut Result_void);
	// GetUnnormalizedDescriptor(double, double, int, float *, double *) /usr/include/opencv2/xfeatures2d.hpp:310
	pub fn cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(instance: *const c_void, y: f64, x: f64, orientation: i32, descriptor: *mut f32, h: *mut f64, ocvrs_return: *mut Result<bool>);
	// axes /usr/include/opencv2/xfeatures2d.hpp:908
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_getPropAxes_const(instance: *const c_void, ocvrs_return: *mut core::Size_<f32>);
	// axes /usr/include/opencv2/xfeatures2d.hpp:908
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_setPropAxes_Size__float_(instance: *mut c_void, val: *const core::Size_<f32>);
	// si /usr/include/opencv2/xfeatures2d.hpp:909
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_getPropSi_const(instance: *const c_void) -> f32;
	// si /usr/include/opencv2/xfeatures2d.hpp:909
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_setPropSi_float(instance: *mut c_void, val: f32);
	// transf /usr/include/opencv2/xfeatures2d.hpp:910
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_getPropTransf_const(instance: *const c_void, ocvrs_return: *mut core::Matx23f);
	// transf /usr/include/opencv2/xfeatures2d.hpp:910
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_setPropTransf_Matx23f(instance: *mut c_void, val: *const core::Matx23f);
	// Elliptic_KeyPoint() /usr/include/opencv2/xfeatures2d.hpp:911
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(ocvrs_return: *mut Result<*mut c_void>);
	// Elliptic_KeyPoint(cv::Point2f, float, cv::Size, float, float) /usr/include/opencv2/xfeatures2d.hpp:912
	pub fn cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(pt: *const core::Point2f, angle: f32, axes: *const core::Size, size: f32, si: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(bool, bool, float, int, const std::vector<int> &) /usr/include/opencv2/xfeatures2d.hpp:100
	pub fn cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vector_int_R(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, float, float, int, int) /usr/include/opencv2/xfeatures2d.hpp:931
	pub fn cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, bool, int, double) /usr/include/opencv2/xfeatures2d.hpp:179
	pub fn cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64, ocvrs_return: *mut Result<*mut c_void>);
	// create(const int, const int) /usr/include/opencv2/xfeatures2d.hpp:150
	pub fn cv_xfeatures2d_LUCID_create_const_int_const_int(lucid_kernel: i32, blur_kernel: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, int, float, int, float, int, bool) /usr/include/opencv2/xfeatures2d.hpp:331
	pub fn cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(const int, const int, const int) /usr/include/opencv2/xfeatures2d.hpp:497
	pub fn cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(init_sample_count: i32, init_seed_count: i32, point_distribution: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const std::vector<Point2f> &, const int) /usr/include/opencv2/xfeatures2d.hpp:511
	pub fn cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_int(init_sampling_points: *const c_void, init_seed_count: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const std::vector<Point2f> &, const std::vector<int> &) /usr/include/opencv2/xfeatures2d.hpp:523
	pub fn cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_vector_int_R(init_sampling_points: *const c_void, init_cluster_seed_indexes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// computeSignature(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:534
	pub fn cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, signature: *const c_void, ocvrs_return: *mut Result_void);
	// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &) /usr/include/opencv2/xfeatures2d.hpp:543
	pub fn cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vector_Mat_R_vector_Mat_R(instance: *const c_void, images: *const c_void, signatures: *mut c_void, ocvrs_return: *mut Result_void);
	// drawSignature(cv::InputArray, cv::InputArray, cv::OutputArray, float, int) /usr/include/opencv2/xfeatures2d.hpp:559
	pub fn cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(source: *const c_void, signature: *const c_void, result: *const c_void, radius_to_shorter_side_ratio: f32, border_thickness: i32, ocvrs_return: *mut Result_void);
	// generateInitPoints(std::vector<Point2f> &, const int, int) /usr/include/opencv2/xfeatures2d.hpp:574
	pub fn cv_xfeatures2d_PCTSignatures_generateInitPoints_vector_Point2f_R_const_int_int(init_points: *mut c_void, count: i32, point_distribution: i32, ocvrs_return: *mut Result_void);
	// getSampleCount() /usr/include/opencv2/xfeatures2d.hpp:585
	pub fn cv_xfeatures2d_PCTSignatures_getSampleCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getGrayscaleBits() /usr/include/opencv2/xfeatures2d.hpp:592
	pub fn cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setGrayscaleBits(int) /usr/include/opencv2/xfeatures2d.hpp:598
	pub fn cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(instance: *mut c_void, grayscale_bits: i32, ocvrs_return: *mut Result_void);
	// getWindowRadius() /usr/include/opencv2/xfeatures2d.hpp:605
	pub fn cv_xfeatures2d_PCTSignatures_getWindowRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWindowRadius(int) /usr/include/opencv2/xfeatures2d.hpp:611
	pub fn cv_xfeatures2d_PCTSignatures_setWindowRadius_int(instance: *mut c_void, radius: i32, ocvrs_return: *mut Result_void);
	// getWeightX() /usr/include/opencv2/xfeatures2d.hpp:618
	pub fn cv_xfeatures2d_PCTSignatures_getWeightX_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightX(float) /usr/include/opencv2/xfeatures2d.hpp:623
	pub fn cv_xfeatures2d_PCTSignatures_setWeightX_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightY() /usr/include/opencv2/xfeatures2d.hpp:629
	pub fn cv_xfeatures2d_PCTSignatures_getWeightY_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightY(float) /usr/include/opencv2/xfeatures2d.hpp:634
	pub fn cv_xfeatures2d_PCTSignatures_setWeightY_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightL() /usr/include/opencv2/xfeatures2d.hpp:640
	pub fn cv_xfeatures2d_PCTSignatures_getWeightL_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightL(float) /usr/include/opencv2/xfeatures2d.hpp:645
	pub fn cv_xfeatures2d_PCTSignatures_setWeightL_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightA() /usr/include/opencv2/xfeatures2d.hpp:651
	pub fn cv_xfeatures2d_PCTSignatures_getWeightA_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightA(float) /usr/include/opencv2/xfeatures2d.hpp:656
	pub fn cv_xfeatures2d_PCTSignatures_setWeightA_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightB() /usr/include/opencv2/xfeatures2d.hpp:662
	pub fn cv_xfeatures2d_PCTSignatures_getWeightB_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightB(float) /usr/include/opencv2/xfeatures2d.hpp:667
	pub fn cv_xfeatures2d_PCTSignatures_setWeightB_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightContrast() /usr/include/opencv2/xfeatures2d.hpp:673
	pub fn cv_xfeatures2d_PCTSignatures_getWeightContrast_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightContrast(float) /usr/include/opencv2/xfeatures2d.hpp:678
	pub fn cv_xfeatures2d_PCTSignatures_setWeightContrast_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getWeightEntropy() /usr/include/opencv2/xfeatures2d.hpp:684
	pub fn cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setWeightEntropy(float) /usr/include/opencv2/xfeatures2d.hpp:689
	pub fn cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(instance: *mut c_void, weight: f32, ocvrs_return: *mut Result_void);
	// getSamplingPoints() /usr/include/opencv2/xfeatures2d.hpp:695
	pub fn cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setWeight(int, float) /usr/include/opencv2/xfeatures2d.hpp:713
	pub fn cv_xfeatures2d_PCTSignatures_setWeight_int_float(instance: *mut c_void, idx: i32, value: f32, ocvrs_return: *mut Result_void);
	// setWeights(const std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:727
	pub fn cv_xfeatures2d_PCTSignatures_setWeights_const_vector_float_R(instance: *mut c_void, weights: *const c_void, ocvrs_return: *mut Result_void);
	// setTranslation(int, float) /usr/include/opencv2/xfeatures2d.hpp:743
	pub fn cv_xfeatures2d_PCTSignatures_setTranslation_int_float(instance: *mut c_void, idx: i32, value: f32, ocvrs_return: *mut Result_void);
	// setTranslations(const std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:757
	pub fn cv_xfeatures2d_PCTSignatures_setTranslations_const_vector_float_R(instance: *mut c_void, translations: *const c_void, ocvrs_return: *mut Result_void);
	// setSamplingPoints(std::vector<Point2f>) /usr/include/opencv2/xfeatures2d.hpp:764
	pub fn cv_xfeatures2d_PCTSignatures_setSamplingPoints_vector_Point2f_(instance: *mut c_void, sampling_points: *mut c_void, ocvrs_return: *mut Result_void);
	// getInitSeedIndexes() /usr/include/opencv2/xfeatures2d.hpp:772
	pub fn cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setInitSeedIndexes(std::vector<int>) /usr/include/opencv2/xfeatures2d.hpp:776
	pub fn cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vector_int_(instance: *mut c_void, init_seed_indexes: *mut c_void, ocvrs_return: *mut Result_void);
	// getInitSeedCount() /usr/include/opencv2/xfeatures2d.hpp:780
	pub fn cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getIterationCount() /usr/include/opencv2/xfeatures2d.hpp:787
	pub fn cv_xfeatures2d_PCTSignatures_getIterationCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterationCount(int) /usr/include/opencv2/xfeatures2d.hpp:793
	pub fn cv_xfeatures2d_PCTSignatures_setIterationCount_int(instance: *mut c_void, iteration_count: i32, ocvrs_return: *mut Result_void);
	// getMaxClustersCount() /usr/include/opencv2/xfeatures2d.hpp:799
	pub fn cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxClustersCount(int) /usr/include/opencv2/xfeatures2d.hpp:804
	pub fn cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(instance: *mut c_void, max_clusters_count: i32, ocvrs_return: *mut Result_void);
	// getClusterMinSize() /usr/include/opencv2/xfeatures2d.hpp:811
	pub fn cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setClusterMinSize(int) /usr/include/opencv2/xfeatures2d.hpp:817
	pub fn cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(instance: *mut c_void, cluster_min_size: i32, ocvrs_return: *mut Result_void);
	// getJoiningDistance() /usr/include/opencv2/xfeatures2d.hpp:824
	pub fn cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setJoiningDistance(float) /usr/include/opencv2/xfeatures2d.hpp:830
	pub fn cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(instance: *mut c_void, joining_distance: f32, ocvrs_return: *mut Result_void);
	// getDropThreshold() /usr/include/opencv2/xfeatures2d.hpp:835
	pub fn cv_xfeatures2d_PCTSignatures_getDropThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setDropThreshold(float) /usr/include/opencv2/xfeatures2d.hpp:839
	pub fn cv_xfeatures2d_PCTSignatures_setDropThreshold_float(instance: *mut c_void, drop_threshold: f32, ocvrs_return: *mut Result_void);
	// getDistanceFunction() /usr/include/opencv2/xfeatures2d.hpp:844
	pub fn cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDistanceFunction(int) /usr/include/opencv2/xfeatures2d.hpp:849
	pub fn cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(instance: *mut c_void, distance_function: i32, ocvrs_return: *mut Result_void);
	// create(const int, const int, const float) /usr/include/opencv2/xfeatures2d.hpp:874
	pub fn cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(distance_function: i32, similarity_function: i32, similarity_parameter: f32, ocvrs_return: *mut Result<*mut c_void>);
	// computeQuadraticFormDistance(cv::InputArray, cv::InputArray) /usr/include/opencv2/xfeatures2d.hpp:884
	pub fn cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, _signature0: *const c_void, _signature1: *const c_void, ocvrs_return: *mut Result<f32>);
	// computeQuadraticFormDistances(const cv::Mat &, const std::vector<Mat> &, std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:895
	pub fn cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vector_Mat_R_vector_float_R(instance: *const c_void, source_signature: *const c_void, image_signatures: *const c_void, distances: *mut c_void, ocvrs_return: *mut Result_void);
	// create(double, int, int, bool, bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:94
	pub fn cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool, ocvrs_return: *mut Result<*mut c_void>);
	// setHessianThreshold(double) /usr/include/opencv2/xfeatures2d/nonfree.hpp:98
	pub fn cv_xfeatures2d_SURF_setHessianThreshold_double(instance: *mut c_void, hessian_threshold: f64, ocvrs_return: *mut Result_void);
	// getHessianThreshold() /usr/include/opencv2/xfeatures2d/nonfree.hpp:99
	pub fn cv_xfeatures2d_SURF_getHessianThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNOctaves(int) /usr/include/opencv2/xfeatures2d/nonfree.hpp:101
	pub fn cv_xfeatures2d_SURF_setNOctaves_int(instance: *mut c_void, n_octaves: i32, ocvrs_return: *mut Result_void);
	// getNOctaves() /usr/include/opencv2/xfeatures2d/nonfree.hpp:102
	pub fn cv_xfeatures2d_SURF_getNOctaves_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNOctaveLayers(int) /usr/include/opencv2/xfeatures2d/nonfree.hpp:104
	pub fn cv_xfeatures2d_SURF_setNOctaveLayers_int(instance: *mut c_void, n_octave_layers: i32, ocvrs_return: *mut Result_void);
	// getNOctaveLayers() /usr/include/opencv2/xfeatures2d/nonfree.hpp:105
	pub fn cv_xfeatures2d_SURF_getNOctaveLayers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setExtended(bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:107
	pub fn cv_xfeatures2d_SURF_setExtended_bool(instance: *mut c_void, extended: bool, ocvrs_return: *mut Result_void);
	// getExtended() /usr/include/opencv2/xfeatures2d/nonfree.hpp:108
	pub fn cv_xfeatures2d_SURF_getExtended_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUpright(bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:110
	pub fn cv_xfeatures2d_SURF_setUpright_bool(instance: *mut c_void, upright: bool, ocvrs_return: *mut Result_void);
	// getUpright() /usr/include/opencv2/xfeatures2d/nonfree.hpp:111
	pub fn cv_xfeatures2d_SURF_getUpright_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(int, int, int, int, int) /usr/include/opencv2/xfeatures2d.hpp:114
	pub fn cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, float, float, int) /usr/include/opencv2/xfeatures2d.hpp:1013
	pub fn cv_xfeatures2d_TBMR_create_int_float_float_int(min_area: i32, max_area_relative: f32, scale_factor: f32, n_scales: i32, ocvrs_return: *mut Result<*mut c_void>);
	// setMinArea(int) /usr/include/opencv2/xfeatures2d.hpp:1018
	pub fn cv_xfeatures2d_TBMR_setMinArea_int(instance: *mut c_void, min_area: i32, ocvrs_return: *mut Result_void);
	// getMinArea() /usr/include/opencv2/xfeatures2d.hpp:1019
	pub fn cv_xfeatures2d_TBMR_getMinArea_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxAreaRelative(float) /usr/include/opencv2/xfeatures2d.hpp:1020
	pub fn cv_xfeatures2d_TBMR_setMaxAreaRelative_float(instance: *mut c_void, max_area: f32, ocvrs_return: *mut Result_void);
	// getMaxAreaRelative() /usr/include/opencv2/xfeatures2d.hpp:1021
	pub fn cv_xfeatures2d_TBMR_getMaxAreaRelative_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setScaleFactor(float) /usr/include/opencv2/xfeatures2d.hpp:1022
	pub fn cv_xfeatures2d_TBMR_setScaleFactor_float(instance: *mut c_void, scale_factor: f32, ocvrs_return: *mut Result_void);
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:1023
	pub fn cv_xfeatures2d_TBMR_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setNScales(int) /usr/include/opencv2/xfeatures2d.hpp:1024
	pub fn cv_xfeatures2d_TBMR_setNScales_int(instance: *mut c_void, n_scales: i32, ocvrs_return: *mut Result_void);
	// getNScales() /usr/include/opencv2/xfeatures2d.hpp:1025
	pub fn cv_xfeatures2d_TBMR_getNScales_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// create(int, float, bool, bool, float, bool) /usr/include/opencv2/xfeatures2d.hpp:362
	pub fn cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool, ocvrs_return: *mut Result<*mut c_void>);
	// setSigma(const float) /usr/include/opencv2/xfeatures2d.hpp:366
	pub fn cv_xfeatures2d_VGG_setSigma_const_float(instance: *mut c_void, isigma: f32, ocvrs_return: *mut Result_void);
	// getSigma() /usr/include/opencv2/xfeatures2d.hpp:367
	pub fn cv_xfeatures2d_VGG_getSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setUseNormalizeImage(const bool) /usr/include/opencv2/xfeatures2d.hpp:369
	pub fn cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(instance: *mut c_void, img_normalize: bool, ocvrs_return: *mut Result_void);
	// getUseNormalizeImage() /usr/include/opencv2/xfeatures2d.hpp:370
	pub fn cv_xfeatures2d_VGG_getUseNormalizeImage_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseScaleOrientation(const bool) /usr/include/opencv2/xfeatures2d.hpp:372
	pub fn cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(instance: *mut c_void, use_scale_orientation: bool, ocvrs_return: *mut Result_void);
	// getUseScaleOrientation() /usr/include/opencv2/xfeatures2d.hpp:373
	pub fn cv_xfeatures2d_VGG_getUseScaleOrientation_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setScaleFactor(const float) /usr/include/opencv2/xfeatures2d.hpp:375
	pub fn cv_xfeatures2d_VGG_setScaleFactor_const_float(instance: *mut c_void, scale_factor: f32, ocvrs_return: *mut Result_void);
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:376
	pub fn cv_xfeatures2d_VGG_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setUseNormalizeDescriptor(const bool) /usr/include/opencv2/xfeatures2d.hpp:378
	pub fn cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(instance: *mut c_void, dsc_normalize: bool, ocvrs_return: *mut Result_void);
	// getUseNormalizeDescriptor() /usr/include/opencv2/xfeatures2d.hpp:379
	pub fn cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
}
