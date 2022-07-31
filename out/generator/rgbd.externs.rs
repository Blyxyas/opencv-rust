extern "C" {
	// makeVolume(cv::kinfu::VolumeType, float, cv::Matx44f, float, float, int, float, cv::Vec3i) /usr/include/opencv2/rgbd/volume.hpp:122
	pub fn cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(_volume_type: crate::rgbd::Kinfu_VolumeType, _voxel_size: f32, _pose: *const core::Matx44f, _raycast_step_factor: f32, _trunc_dist: f32, _max_weight: i32, _truncate_threshold: f32, _resolution: *const core::Vec3i, ocvrs_return: *mut Result<*mut c_void>);
	// colormap(const cv::Mat &, cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:245
	pub fn cv_linemod_colormap_const_MatR_MatR(quantized: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// drawFeatures(cv::InputOutputArray, const std::vector<Template> &, const cv::Point2i &, int) /usr/include/opencv2/rgbd/linemod.hpp:254
	pub fn cv_linemod_drawFeatures_const__InputOutputArrayR_const_vector_Template_R_const_Point2iR_int(img: *const c_void, templates: *const c_void, tl: *const core::Point2i, size: i32, ocvrs_return: *mut Result_void);
	// getDefaultLINE() /usr/include/opencv2/rgbd/linemod.hpp:420
	pub fn cv_linemod_getDefaultLINE(ocvrs_return: *mut Result<*mut c_void>);
	// getDefaultLINEMOD() /usr/include/opencv2/rgbd/linemod.hpp:428
	pub fn cv_linemod_getDefaultLINEMOD(ocvrs_return: *mut Result<*mut c_void>);
	// depthTo3dSparse(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/depth.hpp:299
	pub fn cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth: *const c_void, in_k: *const c_void, in_points: *const c_void, points3d: *const c_void, ocvrs_return: *mut Result_void);
	// depthTo3d(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/rgbd/depth.hpp:312
	pub fn cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth: *const c_void, k: *const c_void, points3d: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// isValidDepth(const double &) /usr/include/opencv2/rgbd/depth.hpp:33
	pub fn cv_rgbd_isValidDepth_const_doubleR(depth: *const f64, ocvrs_return: *mut Result<bool>);
	// isValidDepth(const float &) /usr/include/opencv2/rgbd/depth.hpp:27
	pub fn cv_rgbd_isValidDepth_const_floatR(depth: *const f32, ocvrs_return: *mut Result<bool>);
	// isValidDepth(const int &) /usr/include/opencv2/rgbd/depth.hpp:52
	pub fn cv_rgbd_isValidDepth_const_intR(depth: *const i32, ocvrs_return: *mut Result<bool>);
	// isValidDepth(const short &) /usr/include/opencv2/rgbd/depth.hpp:39
	pub fn cv_rgbd_isValidDepth_const_shortR(depth: *const i16, ocvrs_return: *mut Result<bool>);
	// isValidDepth(const unsigned int &) /usr/include/opencv2/rgbd/depth.hpp:58
	pub fn cv_rgbd_isValidDepth_const_unsigned_intR(depth: *const u32, ocvrs_return: *mut Result<bool>);
	// isValidDepth(const unsigned short &) /usr/include/opencv2/rgbd/depth.hpp:45
	pub fn cv_rgbd_isValidDepth_const_unsigned_shortR(depth: *const u16, ocvrs_return: *mut Result<bool>);
	// registerDepth(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, cv::OutputArray, bool) /usr/include/opencv2/rgbd/depth.hpp:287
	pub fn cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix: *const c_void, registered_camera_matrix: *const c_void, registered_dist_coeffs: *const c_void, rt: *const c_void, unregistered_depth: *const c_void, output_image_plane_size: *const core::Size, registered_depth: *const c_void, depth_dilation: bool, ocvrs_return: *mut Result_void);
	// rescaleDepth(cv::InputArray, int, cv::OutputArray, double) /usr/include/opencv2/rgbd/depth.hpp:325
	pub fn cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_: *const c_void, depth: i32, out: *const c_void, depth_factor: f64, ocvrs_return: *mut Result_void);
	// warpFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/depth.hpp:1179
	pub fn cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image: *const c_void, depth: *const c_void, mask: *const c_void, rt: *const c_void, camera_matrix: *const c_void, dist_coeff: *const c_void, warped_image: *const c_void, warped_depth: *const c_void, warped_mask: *const c_void, ocvrs_return: *mut Result_void);
	// create(const Ptr<cv::colored_kinfu::Params> &) /usr/include/opencv2/rgbd/colored_kinfu.hpp:198
	pub fn cv_colored_kinfu_ColoredKinFu_create_const_Ptr_Params_R(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:202
	pub fn cv_colored_kinfu_ColoredKinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:212
	pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/colored_kinfu.hpp:224
	pub fn cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// getCloud(cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:235
	pub fn cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result_void);
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:243
	pub fn cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result_void);
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:249
	pub fn cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/rgbd/colored_kinfu.hpp:255
	pub fn cv_colored_kinfu_ColoredKinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getPose() /usr/include/opencv2/rgbd/colored_kinfu.hpp:258
	pub fn cv_colored_kinfu_ColoredKinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
	// update(cv::InputArray, cv::InputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:266
	pub fn cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(instance: *mut c_void, depth: *const c_void, rgb: *const c_void, ocvrs_return: *mut Result<bool>);
	// frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:83
	pub fn cv_colored_kinfu_Params_getPropFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:83
	pub fn cv_colored_kinfu_Params_setPropFrameSize_Size(instance: *mut c_void, val: *const core::Size);
	// rgb_frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:86
	pub fn cv_colored_kinfu_Params_getPropRgb_frameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// rgb_frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:86
	pub fn cv_colored_kinfu_Params_setPropRgb_frameSize_Size(instance: *mut c_void, val: *const core::Size);
	// volumeType /usr/include/opencv2/rgbd/colored_kinfu.hpp:88
	pub fn cv_colored_kinfu_Params_getPropVolumeType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
	// volumeType /usr/include/opencv2/rgbd/colored_kinfu.hpp:88
	pub fn cv_colored_kinfu_Params_setPropVolumeType_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
	// intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:91
	pub fn cv_colored_kinfu_Params_getPropIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:91
	pub fn cv_colored_kinfu_Params_setPropIntr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:94
	pub fn cv_colored_kinfu_Params_getPropRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:94
	pub fn cv_colored_kinfu_Params_setPropRgb_intr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// depthFactor /usr/include/opencv2/rgbd/colored_kinfu.hpp:103
	pub fn cv_colored_kinfu_Params_getPropDepthFactor_const(instance: *const c_void) -> f32;
	// depthFactor /usr/include/opencv2/rgbd/colored_kinfu.hpp:103
	pub fn cv_colored_kinfu_Params_setPropDepthFactor_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/colored_kinfu.hpp:106
	pub fn cv_colored_kinfu_Params_getPropBilateral_sigma_depth_const(instance: *const c_void) -> f32;
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/colored_kinfu.hpp:106
	pub fn cv_colored_kinfu_Params_setPropBilateral_sigma_depth_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/colored_kinfu.hpp:108
	pub fn cv_colored_kinfu_Params_getPropBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/colored_kinfu.hpp:108
	pub fn cv_colored_kinfu_Params_setPropBilateral_sigma_spatial_float(instance: *mut c_void, val: f32);
	// bilateral_kernel_size /usr/include/opencv2/rgbd/colored_kinfu.hpp:110
	pub fn cv_colored_kinfu_Params_getPropBilateral_kernel_size_const(instance: *const c_void) -> i32;
	// bilateral_kernel_size /usr/include/opencv2/rgbd/colored_kinfu.hpp:110
	pub fn cv_colored_kinfu_Params_setPropBilateral_kernel_size_int(instance: *mut c_void, val: i32);
	// pyramidLevels /usr/include/opencv2/rgbd/colored_kinfu.hpp:113
	pub fn cv_colored_kinfu_Params_getPropPyramidLevels_const(instance: *const c_void) -> i32;
	// pyramidLevels /usr/include/opencv2/rgbd/colored_kinfu.hpp:113
	pub fn cv_colored_kinfu_Params_setPropPyramidLevels_int(instance: *mut c_void, val: i32);
	// volumeDims /usr/include/opencv2/rgbd/colored_kinfu.hpp:119
	pub fn cv_colored_kinfu_Params_getPropVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
	// volumeDims /usr/include/opencv2/rgbd/colored_kinfu.hpp:119
	pub fn cv_colored_kinfu_Params_setPropVolumeDims_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
	// voxelSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:121
	pub fn cv_colored_kinfu_Params_getPropVoxelSize_const(instance: *const c_void) -> f32;
	// voxelSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:121
	pub fn cv_colored_kinfu_Params_setPropVoxelSize_float(instance: *mut c_void, val: f32);
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/colored_kinfu.hpp:127
	pub fn cv_colored_kinfu_Params_getPropTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/colored_kinfu.hpp:127
	pub fn cv_colored_kinfu_Params_setPropTsdf_min_camera_movement_float(instance: *mut c_void, val: f32);
	// volumePose /usr/include/opencv2/rgbd/colored_kinfu.hpp:130
	pub fn cv_colored_kinfu_Params_getPropVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
	// volumePose /usr/include/opencv2/rgbd/colored_kinfu.hpp:130
	pub fn cv_colored_kinfu_Params_setPropVolumePose_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/colored_kinfu.hpp:136
	pub fn cv_colored_kinfu_Params_getPropTsdf_trunc_dist_const(instance: *const c_void) -> f32;
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/colored_kinfu.hpp:136
	pub fn cv_colored_kinfu_Params_setPropTsdf_trunc_dist_float(instance: *mut c_void, val: f32);
	// tsdf_max_weight /usr/include/opencv2/rgbd/colored_kinfu.hpp:142
	pub fn cv_colored_kinfu_Params_getPropTsdf_max_weight_const(instance: *const c_void) -> i32;
	// tsdf_max_weight /usr/include/opencv2/rgbd/colored_kinfu.hpp:142
	pub fn cv_colored_kinfu_Params_setPropTsdf_max_weight_int(instance: *mut c_void, val: i32);
	// raycast_step_factor /usr/include/opencv2/rgbd/colored_kinfu.hpp:148
	pub fn cv_colored_kinfu_Params_getPropRaycast_step_factor_const(instance: *const c_void) -> f32;
	// raycast_step_factor /usr/include/opencv2/rgbd/colored_kinfu.hpp:148
	pub fn cv_colored_kinfu_Params_setPropRaycast_step_factor_float(instance: *mut c_void, val: f32);
	// lightPose /usr/include/opencv2/rgbd/colored_kinfu.hpp:155
	pub fn cv_colored_kinfu_Params_getPropLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
	// lightPose /usr/include/opencv2/rgbd/colored_kinfu.hpp:155
	pub fn cv_colored_kinfu_Params_setPropLightPose_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
	// icpDistThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:158
	pub fn cv_colored_kinfu_Params_getPropIcpDistThresh_const(instance: *const c_void) -> f32;
	// icpDistThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:158
	pub fn cv_colored_kinfu_Params_setPropIcpDistThresh_float(instance: *mut c_void, val: f32);
	// icpAngleThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:160
	pub fn cv_colored_kinfu_Params_getPropIcpAngleThresh_const(instance: *const c_void) -> f32;
	// icpAngleThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:160
	pub fn cv_colored_kinfu_Params_setPropIcpAngleThresh_float(instance: *mut c_void, val: f32);
	// icpIterations /usr/include/opencv2/rgbd/colored_kinfu.hpp:162
	pub fn cv_colored_kinfu_Params_getPropIcpIterations_const(instance: *const c_void) -> *mut c_void;
	// icpIterations /usr/include/opencv2/rgbd/colored_kinfu.hpp:162
	pub fn cv_colored_kinfu_Params_setPropIcpIterations_vector_int_(instance: *mut c_void, val: *mut c_void);
	// truncateThreshold /usr/include/opencv2/rgbd/colored_kinfu.hpp:168
	pub fn cv_colored_kinfu_Params_getPropTruncateThreshold_const(instance: *const c_void) -> f32;
	// truncateThreshold /usr/include/opencv2/rgbd/colored_kinfu.hpp:168
	pub fn cv_colored_kinfu_Params_setPropTruncateThreshold_float(instance: *mut c_void, val: f32);
	// Params() /usr/include/opencv2/rgbd/colored_kinfu.hpp:22
	pub fn cv_colored_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// Params(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:30
	pub fn cv_colored_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
	// Params(cv::Matx44f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:40
	pub fn cv_colored_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
	// setInitialVolumePose(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:51
	pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result_void);
	// setInitialVolumePose(cv::Matx44f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:58
	pub fn cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// defaultParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:64
	pub fn cv_colored_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
	// coarseParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:70
	pub fn cv_colored_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/colored_kinfu.hpp:75
	pub fn cv_colored_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
	// coloredTSDFParams(bool) /usr/include/opencv2/rgbd/colored_kinfu.hpp:80
	pub fn cv_colored_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(const Ptr<kinfu::Params> &) /usr/include/opencv2/rgbd/dynafu.hpp:49
	pub fn cv_dynafu_DynaFu_create_const_Ptr_Params_R(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getParams() /usr/include/opencv2/rgbd/dynafu.hpp:53
	pub fn cv_dynafu_DynaFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/dynafu.hpp:65
	pub fn cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:75
	pub fn cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:83
	pub fn cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result_void);
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:89
	pub fn cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/rgbd/dynafu.hpp:95
	pub fn cv_dynafu_DynaFu_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getPose() /usr/include/opencv2/rgbd/dynafu.hpp:98
	pub fn cv_dynafu_DynaFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
	// update(cv::InputArray) /usr/include/opencv2/rgbd/dynafu.hpp:108
	pub fn cv_dynafu_DynaFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
	// getNodesPos() /usr/include/opencv2/rgbd/dynafu.hpp:110
	pub fn cv_dynafu_DynaFu_getNodesPos_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// marchCubes(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:112
	pub fn cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, vertices: *const c_void, edges: *const c_void, ocvrs_return: *mut Result_void);
	// renderSurface(cv::OutputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/rgbd/dynafu.hpp:114
	pub fn cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(instance: *mut c_void, depth_image: *const c_void, vert_image: *const c_void, norm_image: *const c_void, warp: bool, ocvrs_return: *mut Result_void);
	// Intr() /usr/include/opencv2/rgbd/intrinsics.hpp:61
	pub fn cv_kinfu_Intr_Intr(ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
	// Intr(float, float, float, float) /usr/include/opencv2/rgbd/intrinsics.hpp:62
	pub fn cv_kinfu_Intr_Intr_float_float_float_float(_fx: f32, _fy: f32, _cx: f32, _cy: f32, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
	// Intr(cv::Matx33f) /usr/include/opencv2/rgbd/intrinsics.hpp:63
	pub fn cv_kinfu_Intr_Intr_Matx33f(m: *const core::Matx33f, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
	// scale(int) /usr/include/opencv2/rgbd/intrinsics.hpp:65
	pub fn cv_kinfu_Intr_scale_const_int(instance: *const crate::rgbd::Kinfu_Intr, pyr: i32, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr>);
	// makeReprojector() /usr/include/opencv2/rgbd/intrinsics.hpp:70
	pub fn cv_kinfu_Intr_makeReprojector_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
	// makeProjector() /usr/include/opencv2/rgbd/intrinsics.hpp:71
	pub fn cv_kinfu_Intr_makeProjector_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Projector>);
	// getMat() /usr/include/opencv2/rgbd/intrinsics.hpp:73
	pub fn cv_kinfu_Intr_getMat_const(instance: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<core::Matx33f>);
	// Projector(cv::kinfu::Intr) /usr/include/opencv2/rgbd/intrinsics.hpp:41
	pub fn cv_kinfu_Intr_Projector_Projector_Intr(intr: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Projector>);
	// Reprojector() /usr/include/opencv2/rgbd/intrinsics.hpp:21
	pub fn cv_kinfu_Intr_Reprojector_Reprojector(ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
	// Reprojector(cv::kinfu::Intr) /usr/include/opencv2/rgbd/intrinsics.hpp:22
	pub fn cv_kinfu_Intr_Reprojector_Reprojector_Intr(intr: *const crate::rgbd::Kinfu_Intr, ocvrs_return: *mut Result<crate::rgbd::Kinfu_Intr_Reprojector>);
	// create(const Ptr<cv::kinfu::Params> &) /usr/include/opencv2/rgbd/kinfu.hpp:195
	pub fn cv_kinfu_KinFu_create_const_Ptr_Params_R(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getParams() /usr/include/opencv2/rgbd/kinfu.hpp:199
	pub fn cv_kinfu_KinFu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:209
	pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/kinfu.hpp:221
	pub fn cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:231
	pub fn cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:239
	pub fn cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result_void);
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:245
	pub fn cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/rgbd/kinfu.hpp:251
	pub fn cv_kinfu_KinFu_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getPose() /usr/include/opencv2/rgbd/kinfu.hpp:254
	pub fn cv_kinfu_KinFu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
	// update(cv::InputArray) /usr/include/opencv2/rgbd/kinfu.hpp:264
	pub fn cv_kinfu_KinFu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
	// frameSize /usr/include/opencv2/rgbd/kinfu.hpp:83
	pub fn cv_kinfu_Params_getPropFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// frameSize /usr/include/opencv2/rgbd/kinfu.hpp:83
	pub fn cv_kinfu_Params_setPropFrameSize_Size(instance: *mut c_void, val: *const core::Size);
	// volumeType /usr/include/opencv2/rgbd/kinfu.hpp:86
	pub fn cv_kinfu_Params_getPropVolumeType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
	// volumeType /usr/include/opencv2/rgbd/kinfu.hpp:86
	pub fn cv_kinfu_Params_setPropVolumeType_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
	// intr /usr/include/opencv2/rgbd/kinfu.hpp:89
	pub fn cv_kinfu_Params_getPropIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// intr /usr/include/opencv2/rgbd/kinfu.hpp:89
	pub fn cv_kinfu_Params_setPropIntr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/kinfu.hpp:92
	pub fn cv_kinfu_Params_getPropRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/kinfu.hpp:92
	pub fn cv_kinfu_Params_setPropRgb_intr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// depthFactor /usr/include/opencv2/rgbd/kinfu.hpp:100
	pub fn cv_kinfu_Params_getPropDepthFactor_const(instance: *const c_void) -> f32;
	// depthFactor /usr/include/opencv2/rgbd/kinfu.hpp:100
	pub fn cv_kinfu_Params_setPropDepthFactor_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/kinfu.hpp:103
	pub fn cv_kinfu_Params_getPropBilateral_sigma_depth_const(instance: *const c_void) -> f32;
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/kinfu.hpp:103
	pub fn cv_kinfu_Params_setPropBilateral_sigma_depth_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/kinfu.hpp:105
	pub fn cv_kinfu_Params_getPropBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/kinfu.hpp:105
	pub fn cv_kinfu_Params_setPropBilateral_sigma_spatial_float(instance: *mut c_void, val: f32);
	// bilateral_kernel_size /usr/include/opencv2/rgbd/kinfu.hpp:107
	pub fn cv_kinfu_Params_getPropBilateral_kernel_size_const(instance: *const c_void) -> i32;
	// bilateral_kernel_size /usr/include/opencv2/rgbd/kinfu.hpp:107
	pub fn cv_kinfu_Params_setPropBilateral_kernel_size_int(instance: *mut c_void, val: i32);
	// pyramidLevels /usr/include/opencv2/rgbd/kinfu.hpp:110
	pub fn cv_kinfu_Params_getPropPyramidLevels_const(instance: *const c_void) -> i32;
	// pyramidLevels /usr/include/opencv2/rgbd/kinfu.hpp:110
	pub fn cv_kinfu_Params_setPropPyramidLevels_int(instance: *mut c_void, val: i32);
	// volumeDims /usr/include/opencv2/rgbd/kinfu.hpp:116
	pub fn cv_kinfu_Params_getPropVolumeDims_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
	// volumeDims /usr/include/opencv2/rgbd/kinfu.hpp:116
	pub fn cv_kinfu_Params_setPropVolumeDims_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
	// voxelSize /usr/include/opencv2/rgbd/kinfu.hpp:118
	pub fn cv_kinfu_Params_getPropVoxelSize_const(instance: *const c_void) -> f32;
	// voxelSize /usr/include/opencv2/rgbd/kinfu.hpp:118
	pub fn cv_kinfu_Params_setPropVoxelSize_float(instance: *mut c_void, val: f32);
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/kinfu.hpp:124
	pub fn cv_kinfu_Params_getPropTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/kinfu.hpp:124
	pub fn cv_kinfu_Params_setPropTsdf_min_camera_movement_float(instance: *mut c_void, val: f32);
	// volumePose /usr/include/opencv2/rgbd/kinfu.hpp:127
	pub fn cv_kinfu_Params_getPropVolumePose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
	// volumePose /usr/include/opencv2/rgbd/kinfu.hpp:127
	pub fn cv_kinfu_Params_setPropVolumePose_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/kinfu.hpp:133
	pub fn cv_kinfu_Params_getPropTsdf_trunc_dist_const(instance: *const c_void) -> f32;
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/kinfu.hpp:133
	pub fn cv_kinfu_Params_setPropTsdf_trunc_dist_float(instance: *mut c_void, val: f32);
	// tsdf_max_weight /usr/include/opencv2/rgbd/kinfu.hpp:139
	pub fn cv_kinfu_Params_getPropTsdf_max_weight_const(instance: *const c_void) -> i32;
	// tsdf_max_weight /usr/include/opencv2/rgbd/kinfu.hpp:139
	pub fn cv_kinfu_Params_setPropTsdf_max_weight_int(instance: *mut c_void, val: i32);
	// raycast_step_factor /usr/include/opencv2/rgbd/kinfu.hpp:145
	pub fn cv_kinfu_Params_getPropRaycast_step_factor_const(instance: *const c_void) -> f32;
	// raycast_step_factor /usr/include/opencv2/rgbd/kinfu.hpp:145
	pub fn cv_kinfu_Params_setPropRaycast_step_factor_float(instance: *mut c_void, val: f32);
	// lightPose /usr/include/opencv2/rgbd/kinfu.hpp:152
	pub fn cv_kinfu_Params_getPropLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
	// lightPose /usr/include/opencv2/rgbd/kinfu.hpp:152
	pub fn cv_kinfu_Params_setPropLightPose_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
	// icpDistThresh /usr/include/opencv2/rgbd/kinfu.hpp:155
	pub fn cv_kinfu_Params_getPropIcpDistThresh_const(instance: *const c_void) -> f32;
	// icpDistThresh /usr/include/opencv2/rgbd/kinfu.hpp:155
	pub fn cv_kinfu_Params_setPropIcpDistThresh_float(instance: *mut c_void, val: f32);
	// icpAngleThresh /usr/include/opencv2/rgbd/kinfu.hpp:157
	pub fn cv_kinfu_Params_getPropIcpAngleThresh_const(instance: *const c_void) -> f32;
	// icpAngleThresh /usr/include/opencv2/rgbd/kinfu.hpp:157
	pub fn cv_kinfu_Params_setPropIcpAngleThresh_float(instance: *mut c_void, val: f32);
	// icpIterations /usr/include/opencv2/rgbd/kinfu.hpp:159
	pub fn cv_kinfu_Params_getPropIcpIterations_const(instance: *const c_void) -> *mut c_void;
	// icpIterations /usr/include/opencv2/rgbd/kinfu.hpp:159
	pub fn cv_kinfu_Params_setPropIcpIterations_vector_int_(instance: *mut c_void, val: *mut c_void);
	// truncateThreshold /usr/include/opencv2/rgbd/kinfu.hpp:165
	pub fn cv_kinfu_Params_getPropTruncateThreshold_const(instance: *const c_void) -> f32;
	// truncateThreshold /usr/include/opencv2/rgbd/kinfu.hpp:165
	pub fn cv_kinfu_Params_setPropTruncateThreshold_float(instance: *mut c_void, val: f32);
	// Params() /usr/include/opencv2/rgbd/kinfu.hpp:22
	pub fn cv_kinfu_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// Params(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/kinfu.hpp:30
	pub fn cv_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot: *const core::Matx33f, volume_initial_pose_transl: *const core::Vec3f, ocvrs_return: *mut Result<*mut c_void>);
	// Params(cv::Matx44f) /usr/include/opencv2/rgbd/kinfu.hpp:40
	pub fn cv_kinfu_Params_Params_Matx44f(volume_initial_pose: *const core::Matx44f, ocvrs_return: *mut Result<*mut c_void>);
	// setInitialVolumePose(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/kinfu.hpp:51
	pub fn cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(instance: *mut c_void, r: *const core::Matx33f, t: *const core::Vec3f, ocvrs_return: *mut Result_void);
	// setInitialVolumePose(cv::Matx44f) /usr/include/opencv2/rgbd/kinfu.hpp:58
	pub fn cv_kinfu_Params_setInitialVolumePose_Matx44f(instance: *mut c_void, homogen_tf: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// defaultParams() /usr/include/opencv2/rgbd/kinfu.hpp:64
	pub fn cv_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
	// coarseParams() /usr/include/opencv2/rgbd/kinfu.hpp:70
	pub fn cv_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/kinfu.hpp:75
	pub fn cv_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
	// coloredTSDFParams(bool) /usr/include/opencv2/rgbd/kinfu.hpp:80
	pub fn cv_kinfu_Params_coloredTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:49
	pub fn cv_kinfu_Volume_getPropVoxelSize_const(instance: *const c_void) -> f32;
	// voxelSizeInv /usr/include/opencv2/rgbd/volume.hpp:50
	pub fn cv_kinfu_Volume_getPropVoxelSizeInv_const(instance: *const c_void) -> f32;
	// pose /usr/include/opencv2/rgbd/volume.hpp:51
	pub fn cv_kinfu_Volume_getPropPose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:52
	pub fn cv_kinfu_Volume_getPropRaycastStepFactor_const(instance: *const c_void) -> f32;
	// integrate(cv::InputArray, float, const cv::Matx44f &, const kinfu::Intr &, const int) /usr/include/opencv2/rgbd/volume.hpp:31
	pub fn cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(instance: *mut c_void, _depth: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_id: i32, ocvrs_return: *mut Result_void);
	// integrate(cv::InputArray, cv::InputArray, float, const cv::Matx44f &, const kinfu::Intr &, const cv::kinfu::Intr &, const int) /usr/include/opencv2/rgbd/volume.hpp:33
	pub fn cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(instance: *mut c_void, _depth: *const c_void, _rgb: *const c_void, depth_factor: f32, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, rgb_intrinsics: *const crate::rgbd::Kinfu_Intr, frame_id: i32, ocvrs_return: *mut Result_void);
	// raycast(const cv::Matx44f &, const kinfu::Intr &, const cv::Size &, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:36
	pub fn cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_size: *const core::Size, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// raycast(const cv::Matx44f &, const kinfu::Intr &, const cv::Size &, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:38
	pub fn cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, camera_pose: *const core::Matx44f, intrinsics: *const crate::rgbd::Kinfu_Intr, frame_size: *const core::Size, points: *const c_void, normals: *const c_void, colors: *const c_void, ocvrs_return: *mut Result_void);
	// fetchNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:40
	pub fn cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, _normals: *const c_void, ocvrs_return: *mut Result_void);
	// fetchPointsNormals(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:41
	pub fn cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// fetchPointsNormalsColors(cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:42
	pub fn cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/rgbd/volume.hpp:46
	pub fn cv_kinfu_Volume_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// type /usr/include/opencv2/rgbd/volume.hpp:67
	pub fn cv_kinfu_VolumeParams_getPropType_const(instance: *const c_void, ocvrs_return: *mut crate::rgbd::Kinfu_VolumeType);
	// type /usr/include/opencv2/rgbd/volume.hpp:67
	pub fn cv_kinfu_VolumeParams_setPropType_VolumeType(instance: *mut c_void, val: crate::rgbd::Kinfu_VolumeType);
	// resolution /usr/include/opencv2/rgbd/volume.hpp:74
	pub fn cv_kinfu_VolumeParams_getPropResolution_const(instance: *const c_void, ocvrs_return: *mut core::Vec3i);
	// resolution /usr/include/opencv2/rgbd/volume.hpp:74
	pub fn cv_kinfu_VolumeParams_setPropResolution_Vec3i(instance: *mut c_void, val: *const core::Vec3i);
	// unitResolution /usr/include/opencv2/rgbd/volume.hpp:80
	pub fn cv_kinfu_VolumeParams_getPropUnitResolution_const(instance: *const c_void) -> i32;
	// unitResolution /usr/include/opencv2/rgbd/volume.hpp:80
	pub fn cv_kinfu_VolumeParams_setPropUnitResolution_int(instance: *mut c_void, val: i32);
	// pose /usr/include/opencv2/rgbd/volume.hpp:83
	pub fn cv_kinfu_VolumeParams_getPropPose_const(instance: *const c_void, ocvrs_return: *mut core::Affine3f);
	// pose /usr/include/opencv2/rgbd/volume.hpp:83
	pub fn cv_kinfu_VolumeParams_setPropPose_Affine3f(instance: *mut c_void, val: *const core::Affine3f);
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:86
	pub fn cv_kinfu_VolumeParams_getPropVoxelSize_const(instance: *const c_void) -> f32;
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:86
	pub fn cv_kinfu_VolumeParams_setPropVoxelSize_float(instance: *mut c_void, val: f32);
	// tsdfTruncDist /usr/include/opencv2/rgbd/volume.hpp:91
	pub fn cv_kinfu_VolumeParams_getPropTsdfTruncDist_const(instance: *const c_void) -> f32;
	// tsdfTruncDist /usr/include/opencv2/rgbd/volume.hpp:91
	pub fn cv_kinfu_VolumeParams_setPropTsdfTruncDist_float(instance: *mut c_void, val: f32);
	// maxWeight /usr/include/opencv2/rgbd/volume.hpp:97
	pub fn cv_kinfu_VolumeParams_getPropMaxWeight_const(instance: *const c_void) -> i32;
	// maxWeight /usr/include/opencv2/rgbd/volume.hpp:97
	pub fn cv_kinfu_VolumeParams_setPropMaxWeight_int(instance: *mut c_void, val: i32);
	// depthTruncThreshold /usr/include/opencv2/rgbd/volume.hpp:102
	pub fn cv_kinfu_VolumeParams_getPropDepthTruncThreshold_const(instance: *const c_void) -> f32;
	// depthTruncThreshold /usr/include/opencv2/rgbd/volume.hpp:102
	pub fn cv_kinfu_VolumeParams_setPropDepthTruncThreshold_float(instance: *mut c_void, val: f32);
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:107
	pub fn cv_kinfu_VolumeParams_getPropRaycastStepFactor_const(instance: *const c_void) -> f32;
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:107
	pub fn cv_kinfu_VolumeParams_setPropRaycastStepFactor_float(instance: *mut c_void, val: f32);
	// defaultParams(cv::kinfu::VolumeType) /usr/include/opencv2/rgbd/volume.hpp:112
	pub fn cv_kinfu_VolumeParams_defaultParams_VolumeType(_volume_type: crate::rgbd::Kinfu_VolumeType, ocvrs_return: *mut Result<*mut c_void>);
	// coarseParams(cv::kinfu::VolumeType) /usr/include/opencv2/rgbd/volume.hpp:117
	pub fn cv_kinfu_VolumeParams_coarseParams_VolumeType(_volume_type: crate::rgbd::Kinfu_VolumeType, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:29
	pub fn cv_kinfu_detail_PoseGraph_create(ocvrs_return: *mut Result<*mut c_void>);
	// addNode(size_t, const cv::Affine3d &, bool) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:33
	pub fn cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(instance: *mut c_void, _node_id: size_t, _pose: *const core::Affine3d, fixed: bool, ocvrs_return: *mut Result_void);
	// isNodeExist(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:34
	pub fn cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<bool>);
	// setNodeFixed(size_t, bool) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:35
	pub fn cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(instance: *mut c_void, node_id: size_t, fixed: bool, ocvrs_return: *mut Result<bool>);
	// isNodeFixed(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:36
	pub fn cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<bool>);
	// getNodePose(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:37
	pub fn cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(instance: *const c_void, node_id: size_t, ocvrs_return: *mut Result<core::Affine3d>);
	// getNodesIds() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:38
	pub fn cv_kinfu_detail_PoseGraph_getNodesIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getNumNodes() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:39
	pub fn cv_kinfu_detail_PoseGraph_getNumNodes_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// addEdge(size_t, size_t, const cv::Affine3f &, const cv::Matx66f &) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:42
	pub fn cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(instance: *mut c_void, _source_node_id: size_t, _target_node_id: size_t, _transformation: *const core::Affine3f, _information: *const core::Matx66f, ocvrs_return: *mut Result_void);
	// getEdgeStart(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:44
	pub fn cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
	// getEdgeEnd(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:45
	pub fn cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
	// getNumEdges() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:46
	pub fn cv_kinfu_detail_PoseGraph_getNumEdges_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// isValid() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:49
	pub fn cv_kinfu_detail_PoseGraph_isValid_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// optimize(const cv::TermCriteria &) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:53
	pub fn cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(instance: *mut c_void, tc: *const core::TermCriteria, ocvrs_return: *mut Result<i32>);
	// calcEnergy() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:56
	pub fn cv_kinfu_detail_PoseGraph_calcEnergy_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// create(const Ptr<cv::large_kinfu::Params> &) /usr/include/opencv2/rgbd/large_kinfu.hpp:123
	pub fn cv_large_kinfu_LargeKinfu_create_const_Ptr_Params_R(_params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:126
	pub fn cv_large_kinfu_LargeKinfu_getParams_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:128
	pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(instance: *const c_void, image: *const c_void, ocvrs_return: *mut Result_void);
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/large_kinfu.hpp:129
	pub fn cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(instance: *const c_void, image: *const c_void, camera_pose: *const core::Matx44f, ocvrs_return: *mut Result_void);
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:131
	pub fn cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:133
	pub fn cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(instance: *const c_void, points: *const c_void, ocvrs_return: *mut Result_void);
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:135
	pub fn cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, points: *const c_void, normals: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/rgbd/large_kinfu.hpp:137
	pub fn cv_large_kinfu_LargeKinfu_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getPose() /usr/include/opencv2/rgbd/large_kinfu.hpp:139
	pub fn cv_large_kinfu_LargeKinfu_getPose_const(instance: *const c_void, ocvrs_return: *mut Result<core::Affine3f>);
	// update(cv::InputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:141
	pub fn cv_large_kinfu_LargeKinfu_update_const__InputArrayR(instance: *mut c_void, depth: *const c_void, ocvrs_return: *mut Result<bool>);
	// frameSize /usr/include/opencv2/rgbd/large_kinfu.hpp:39
	pub fn cv_large_kinfu_Params_getPropFrameSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// frameSize /usr/include/opencv2/rgbd/large_kinfu.hpp:39
	pub fn cv_large_kinfu_Params_setPropFrameSize_Size(instance: *mut c_void, val: *const core::Size);
	// intr /usr/include/opencv2/rgbd/large_kinfu.hpp:42
	pub fn cv_large_kinfu_Params_getPropIntr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// intr /usr/include/opencv2/rgbd/large_kinfu.hpp:42
	pub fn cv_large_kinfu_Params_setPropIntr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/large_kinfu.hpp:45
	pub fn cv_large_kinfu_Params_getPropRgb_intr_const(instance: *const c_void, ocvrs_return: *mut core::Matx33f);
	// rgb_intr /usr/include/opencv2/rgbd/large_kinfu.hpp:45
	pub fn cv_large_kinfu_Params_setPropRgb_intr_Matx33f(instance: *mut c_void, val: *const core::Matx33f);
	// depthFactor /usr/include/opencv2/rgbd/large_kinfu.hpp:53
	pub fn cv_large_kinfu_Params_getPropDepthFactor_const(instance: *const c_void) -> f32;
	// depthFactor /usr/include/opencv2/rgbd/large_kinfu.hpp:53
	pub fn cv_large_kinfu_Params_setPropDepthFactor_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/large_kinfu.hpp:56
	pub fn cv_large_kinfu_Params_getPropBilateral_sigma_depth_const(instance: *const c_void) -> f32;
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/large_kinfu.hpp:56
	pub fn cv_large_kinfu_Params_setPropBilateral_sigma_depth_float(instance: *mut c_void, val: f32);
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/large_kinfu.hpp:58
	pub fn cv_large_kinfu_Params_getPropBilateral_sigma_spatial_const(instance: *const c_void) -> f32;
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/large_kinfu.hpp:58
	pub fn cv_large_kinfu_Params_setPropBilateral_sigma_spatial_float(instance: *mut c_void, val: f32);
	// bilateral_kernel_size /usr/include/opencv2/rgbd/large_kinfu.hpp:60
	pub fn cv_large_kinfu_Params_getPropBilateral_kernel_size_const(instance: *const c_void) -> i32;
	// bilateral_kernel_size /usr/include/opencv2/rgbd/large_kinfu.hpp:60
	pub fn cv_large_kinfu_Params_setPropBilateral_kernel_size_int(instance: *mut c_void, val: i32);
	// pyramidLevels /usr/include/opencv2/rgbd/large_kinfu.hpp:63
	pub fn cv_large_kinfu_Params_getPropPyramidLevels_const(instance: *const c_void) -> i32;
	// pyramidLevels /usr/include/opencv2/rgbd/large_kinfu.hpp:63
	pub fn cv_large_kinfu_Params_setPropPyramidLevels_int(instance: *mut c_void, val: i32);
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/large_kinfu.hpp:68
	pub fn cv_large_kinfu_Params_getPropTsdf_min_camera_movement_const(instance: *const c_void) -> f32;
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/large_kinfu.hpp:68
	pub fn cv_large_kinfu_Params_setPropTsdf_min_camera_movement_float(instance: *mut c_void, val: f32);
	// lightPose /usr/include/opencv2/rgbd/large_kinfu.hpp:71
	pub fn cv_large_kinfu_Params_getPropLightPose_const(instance: *const c_void, ocvrs_return: *mut core::Vec3f);
	// lightPose /usr/include/opencv2/rgbd/large_kinfu.hpp:71
	pub fn cv_large_kinfu_Params_setPropLightPose_Vec3f(instance: *mut c_void, val: *const core::Vec3f);
	// icpDistThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:74
	pub fn cv_large_kinfu_Params_getPropIcpDistThresh_const(instance: *const c_void) -> f32;
	// icpDistThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:74
	pub fn cv_large_kinfu_Params_setPropIcpDistThresh_float(instance: *mut c_void, val: f32);
	// icpAngleThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:76
	pub fn cv_large_kinfu_Params_getPropIcpAngleThresh_const(instance: *const c_void) -> f32;
	// icpAngleThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:76
	pub fn cv_large_kinfu_Params_setPropIcpAngleThresh_float(instance: *mut c_void, val: f32);
	// icpIterations /usr/include/opencv2/rgbd/large_kinfu.hpp:78
	pub fn cv_large_kinfu_Params_getPropIcpIterations_const(instance: *const c_void) -> *mut c_void;
	// icpIterations /usr/include/opencv2/rgbd/large_kinfu.hpp:78
	pub fn cv_large_kinfu_Params_setPropIcpIterations_vector_int_(instance: *mut c_void, val: *mut c_void);
	// truncateThreshold /usr/include/opencv2/rgbd/large_kinfu.hpp:83
	pub fn cv_large_kinfu_Params_getPropTruncateThreshold_const(instance: *const c_void) -> f32;
	// truncateThreshold /usr/include/opencv2/rgbd/large_kinfu.hpp:83
	pub fn cv_large_kinfu_Params_setPropTruncateThreshold_float(instance: *mut c_void, val: f32);
	// volumeParams /usr/include/opencv2/rgbd/large_kinfu.hpp:87
	pub fn cv_large_kinfu_Params_getPropVolumeParams_const(instance: *const c_void) -> *mut c_void;
	// volumeParams /usr/include/opencv2/rgbd/large_kinfu.hpp:87
	pub fn cv_large_kinfu_Params_setPropVolumeParams_VolumeParams(instance: *mut c_void, val: *mut c_void);
	// defaultParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:25
	pub fn cv_large_kinfu_Params_defaultParams(ocvrs_return: *mut Result<*mut c_void>);
	// coarseParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:31
	pub fn cv_large_kinfu_Params_coarseParams(ocvrs_return: *mut Result<*mut c_void>);
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/large_kinfu.hpp:36
	pub fn cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse: bool, ocvrs_return: *mut Result<*mut c_void>);
	// weak_threshold /usr/include/opencv2/rgbd/linemod.hpp:191
	pub fn cv_linemod_ColorGradient_getPropWeak_threshold_const(instance: *const c_void) -> f32;
	// weak_threshold /usr/include/opencv2/rgbd/linemod.hpp:191
	pub fn cv_linemod_ColorGradient_setPropWeak_threshold_float(instance: *mut c_void, val: f32);
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:192
	pub fn cv_linemod_ColorGradient_getPropNum_features_const(instance: *const c_void) -> size_t;
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:192
	pub fn cv_linemod_ColorGradient_setPropNum_features_size_t(instance: *mut c_void, val: size_t);
	// strong_threshold /usr/include/opencv2/rgbd/linemod.hpp:193
	pub fn cv_linemod_ColorGradient_getPropStrong_threshold_const(instance: *const c_void) -> f32;
	// strong_threshold /usr/include/opencv2/rgbd/linemod.hpp:193
	pub fn cv_linemod_ColorGradient_setPropStrong_threshold_float(instance: *mut c_void, val: f32);
	// ColorGradient() /usr/include/opencv2/rgbd/linemod.hpp:172
	pub fn cv_linemod_ColorGradient_ColorGradient(ocvrs_return: *mut Result<*mut c_void>);
	// ColorGradient(float, size_t, float) /usr/include/opencv2/rgbd/linemod.hpp:182
	pub fn cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create(float, size_t, float) /usr/include/opencv2/rgbd/linemod.hpp:184
	pub fn cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold: f32, num_features: size_t, strong_threshold: f32, ocvrs_return: *mut Result<*mut c_void>);
	// name() /usr/include/opencv2/rgbd/linemod.hpp:186
	pub fn cv_linemod_ColorGradient_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:188
	pub fn cv_linemod_ColorGradient_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:189
	pub fn cv_linemod_ColorGradient_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// distance_threshold /usr/include/opencv2/rgbd/linemod.hpp:232
	pub fn cv_linemod_DepthNormal_getPropDistance_threshold_const(instance: *const c_void) -> i32;
	// distance_threshold /usr/include/opencv2/rgbd/linemod.hpp:232
	pub fn cv_linemod_DepthNormal_setPropDistance_threshold_int(instance: *mut c_void, val: i32);
	// difference_threshold /usr/include/opencv2/rgbd/linemod.hpp:233
	pub fn cv_linemod_DepthNormal_getPropDifference_threshold_const(instance: *const c_void) -> i32;
	// difference_threshold /usr/include/opencv2/rgbd/linemod.hpp:233
	pub fn cv_linemod_DepthNormal_setPropDifference_threshold_int(instance: *mut c_void, val: i32);
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:234
	pub fn cv_linemod_DepthNormal_getPropNum_features_const(instance: *const c_void) -> size_t;
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:234
	pub fn cv_linemod_DepthNormal_setPropNum_features_size_t(instance: *mut c_void, val: size_t);
	// extract_threshold /usr/include/opencv2/rgbd/linemod.hpp:235
	pub fn cv_linemod_DepthNormal_getPropExtract_threshold_const(instance: *const c_void) -> i32;
	// extract_threshold /usr/include/opencv2/rgbd/linemod.hpp:235
	pub fn cv_linemod_DepthNormal_setPropExtract_threshold_int(instance: *mut c_void, val: i32);
	// DepthNormal() /usr/include/opencv2/rgbd/linemod.hpp:209
	pub fn cv_linemod_DepthNormal_DepthNormal(ocvrs_return: *mut Result<*mut c_void>);
	// DepthNormal(int, int, size_t, int) /usr/include/opencv2/rgbd/linemod.hpp:221
	pub fn cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, size_t, int) /usr/include/opencv2/rgbd/linemod.hpp:224
	pub fn cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32, ocvrs_return: *mut Result<*mut c_void>);
	// name() /usr/include/opencv2/rgbd/linemod.hpp:227
	pub fn cv_linemod_DepthNormal_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:229
	pub fn cv_linemod_DepthNormal_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:230
	pub fn cv_linemod_DepthNormal_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// Detector() /usr/include/opencv2/rgbd/linemod.hpp:304
	pub fn cv_linemod_Detector_Detector(ocvrs_return: *mut Result<*mut c_void>);
	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &) /usr/include/opencv2/rgbd/linemod.hpp:313
	pub fn cv_linemod_Detector_Detector_const_vector_Ptr_Modality__R_const_vector_int_R(modalities: *const c_void, t_pyramid: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, cv::OutputArrayOfArrays, const std::vector<Mat> &) /usr/include/opencv2/rgbd/linemod.hpp:330
	pub fn cv_linemod_Detector_match_const_const_vector_Mat_R_float_vector_Match_R_const_vector_String_R_const__OutputArrayR_const_vector_Mat_R(instance: *const c_void, sources: *const c_void, threshold: f32, matches: *mut c_void, class_ids: *const c_void, quantized_images: *const c_void, masks: *const c_void, ocvrs_return: *mut Result_void);
	// addTemplate(const std::vector<Mat> &, const cv::String &, const cv::Mat &, cv::Rect *) /usr/include/opencv2/rgbd/linemod.hpp:345
	pub fn cv_linemod_Detector_addTemplate_const_vector_Mat_R_const_StringR_const_MatR_RectX(instance: *mut c_void, sources: *const c_void, class_id: *const c_char, object_mask: *const c_void, bounding_box: *mut core::Rect, ocvrs_return: *mut Result<i32>);
	// addSyntheticTemplate(const std::vector<Template> &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:351
	pub fn cv_linemod_Detector_addSyntheticTemplate_const_vector_Template_R_const_StringR(instance: *mut c_void, templates: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
	// getModalities() /usr/include/opencv2/rgbd/linemod.hpp:359
	pub fn cv_linemod_Detector_getModalities_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getT(int) /usr/include/opencv2/rgbd/linemod.hpp:364
	pub fn cv_linemod_Detector_getT_const_int(instance: *const c_void, pyramid_level: i32, ocvrs_return: *mut Result<i32>);
	// pyramidLevels() /usr/include/opencv2/rgbd/linemod.hpp:369
	pub fn cv_linemod_Detector_pyramidLevels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getTemplates(const cv::String &, int) /usr/include/opencv2/rgbd/linemod.hpp:377
	pub fn cv_linemod_Detector_getTemplates_const_const_StringR_int(instance: *const c_void, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// numTemplates() /usr/include/opencv2/rgbd/linemod.hpp:379
	pub fn cv_linemod_Detector_numTemplates_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// numTemplates(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:380
	pub fn cv_linemod_Detector_numTemplates_const_const_StringR(instance: *const c_void, class_id: *const c_char, ocvrs_return: *mut Result<i32>);
	// numClasses() /usr/include/opencv2/rgbd/linemod.hpp:381
	pub fn cv_linemod_Detector_numClasses_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// classIds() /usr/include/opencv2/rgbd/linemod.hpp:383
	pub fn cv_linemod_Detector_classIds_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:385
	pub fn cv_linemod_Detector_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:386
	pub fn cv_linemod_Detector_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// readClass(const cv::FileNode &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:388
	pub fn cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(instance: *mut c_void, fn_: *const c_void, class_id_override: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// writeClass(const cv::String &, cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:389
	pub fn cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(instance: *const c_void, class_id: *const c_char, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// readClasses(const std::vector<String> &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:391
	pub fn cv_linemod_Detector_readClasses_const_vector_String_R_const_StringR(instance: *mut c_void, class_ids: *const c_void, format: *const c_char, ocvrs_return: *mut Result_void);
	// writeClasses(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:393
	pub fn cv_linemod_Detector_writeClasses_const_const_StringR(instance: *const c_void, format: *const c_char, ocvrs_return: *mut Result_void);
	// Feature() /usr/include/opencv2/rgbd/linemod.hpp:32
	pub fn cv_linemod_Feature_Feature(ocvrs_return: *mut Result<crate::rgbd::Linemod_Feature>);
	// Feature(int, int, int) /usr/include/opencv2/rgbd/linemod.hpp:33
	pub fn cv_linemod_Feature_Feature_int_int_int(x: i32, y: i32, label: i32, ocvrs_return: *mut Result<crate::rgbd::Linemod_Feature>);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:35
	pub fn cv_linemod_Feature_read_const_FileNodeR(instance: *const crate::rgbd::Linemod_Feature, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:36
	pub fn cv_linemod_Feature_write_const_FileStorageR(instance: *const crate::rgbd::Linemod_Feature, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// x /usr/include/opencv2/rgbd/linemod.hpp:282
	pub fn cv_linemod_Match_getPropX_const(instance: *const c_void) -> i32;
	// x /usr/include/opencv2/rgbd/linemod.hpp:282
	pub fn cv_linemod_Match_setPropX_int(instance: *mut c_void, val: i32);
	// y /usr/include/opencv2/rgbd/linemod.hpp:283
	pub fn cv_linemod_Match_getPropY_const(instance: *const c_void) -> i32;
	// y /usr/include/opencv2/rgbd/linemod.hpp:283
	pub fn cv_linemod_Match_setPropY_int(instance: *mut c_void, val: i32);
	// similarity /usr/include/opencv2/rgbd/linemod.hpp:284
	pub fn cv_linemod_Match_getPropSimilarity_const(instance: *const c_void) -> f32;
	// similarity /usr/include/opencv2/rgbd/linemod.hpp:284
	pub fn cv_linemod_Match_setPropSimilarity_float(instance: *mut c_void, val: f32);
	// class_id /usr/include/opencv2/rgbd/linemod.hpp:285
	pub fn cv_linemod_Match_getPropClass_id_const(instance: *const c_void) -> *mut c_void;
	// class_id /usr/include/opencv2/rgbd/linemod.hpp:285
	pub fn cv_linemod_Match_setPropClass_id_String(instance: *mut c_void, val: *mut c_char);
	// template_id /usr/include/opencv2/rgbd/linemod.hpp:286
	pub fn cv_linemod_Match_getPropTemplate_id_const(instance: *const c_void) -> i32;
	// template_id /usr/include/opencv2/rgbd/linemod.hpp:286
	pub fn cv_linemod_Match_setPropTemplate_id_int(instance: *mut c_void, val: i32);
	// Match() /usr/include/opencv2/rgbd/linemod.hpp:261
	pub fn cv_linemod_Match_Match(ocvrs_return: *mut Result<*mut c_void>);
	// Match(int, int, float, const cv::String &, int) /usr/include/opencv2/rgbd/linemod.hpp:265
	pub fn cv_linemod_Match_Match_int_int_float_const_StringR_int(x: i32, y: i32, similarity: f32, class_id: *const c_char, template_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::linemod::Match &) /usr/include/opencv2/rgbd/linemod.hpp:277
	pub fn cv_linemod_Match_operatorEQ_const_const_MatchR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
	// process(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:132
	pub fn cv_linemod_Modality_process_const_const_MatR_const_MatR(instance: *const c_void, src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// name() /usr/include/opencv2/rgbd/linemod.hpp:138
	pub fn cv_linemod_Modality_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:140
	pub fn cv_linemod_Modality_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:141
	pub fn cv_linemod_Modality_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// create(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:150
	pub fn cv_linemod_Modality_create_const_StringR(modality_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:155
	pub fn cv_linemod_Modality_create_const_FileNodeR(fn_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// quantize(cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:67
	pub fn cv_linemod_QuantizedPyramid_quantize_const_MatR(instance: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// extractTemplate(cv::linemod::Template &) /usr/include/opencv2/rgbd/linemod.hpp:74
	pub fn cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(instance: *const c_void, templ: *mut c_void, ocvrs_return: *mut Result<bool>);
	// pyrDown() /usr/include/opencv2/rgbd/linemod.hpp:81
	pub fn cv_linemod_QuantizedPyramid_pyrDown(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// width /usr/include/opencv2/rgbd/linemod.hpp:43
	pub fn cv_linemod_Template_getPropWidth_const(instance: *const c_void) -> i32;
	// width /usr/include/opencv2/rgbd/linemod.hpp:43
	pub fn cv_linemod_Template_setPropWidth_int(instance: *mut c_void, val: i32);
	// height /usr/include/opencv2/rgbd/linemod.hpp:44
	pub fn cv_linemod_Template_getPropHeight_const(instance: *const c_void) -> i32;
	// height /usr/include/opencv2/rgbd/linemod.hpp:44
	pub fn cv_linemod_Template_setPropHeight_int(instance: *mut c_void, val: i32);
	// pyramid_level /usr/include/opencv2/rgbd/linemod.hpp:45
	pub fn cv_linemod_Template_getPropPyramid_level_const(instance: *const c_void) -> i32;
	// pyramid_level /usr/include/opencv2/rgbd/linemod.hpp:45
	pub fn cv_linemod_Template_setPropPyramid_level_int(instance: *mut c_void, val: i32);
	// features /usr/include/opencv2/rgbd/linemod.hpp:46
	pub fn cv_linemod_Template_getPropFeatures_const(instance: *const c_void) -> *mut c_void;
	// features /usr/include/opencv2/rgbd/linemod.hpp:46
	pub fn cv_linemod_Template_setPropFeatures_vector_Feature_(instance: *mut c_void, val: *mut c_void);
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:48
	pub fn cv_linemod_Template_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:49
	pub fn cv_linemod_Template_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// DepthCleaner() /usr/include/opencv2/rgbd/depth.hpp:198
	pub fn cv_rgbd_DepthCleaner_DepthCleaner(ocvrs_return: *mut Result<*mut c_void>);
	// DepthCleaner(int, int, int) /usr/include/opencv2/rgbd/depth.hpp:212
	pub fn cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int) /usr/include/opencv2/rgbd/depth.hpp:216
	pub fn cv_rgbd_DepthCleaner_create_int_int_int(depth: i32, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// initialize() /usr/include/opencv2/rgbd/depth.hpp:229
	pub fn cv_rgbd_DepthCleaner_initialize_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/rgbd/depth.hpp:231
	pub fn cv_rgbd_DepthCleaner_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWindowSize(int) /usr/include/opencv2/rgbd/depth.hpp:235
	pub fn cv_rgbd_DepthCleaner_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getDepth() /usr/include/opencv2/rgbd/depth.hpp:239
	pub fn cv_rgbd_DepthCleaner_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDepth(int) /usr/include/opencv2/rgbd/depth.hpp:243
	pub fn cv_rgbd_DepthCleaner_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:247
	pub fn cv_rgbd_DepthCleaner_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:251
	pub fn cv_rgbd_DepthCleaner_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// FastICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:1042
	pub fn cv_rgbd_FastICPOdometry_FastICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
	// FastICPOdometry(const cv::Mat &, float, float, float, float, int, const std::vector<int> &) /usr/include/opencv2/rgbd/depth.hpp:1054
	pub fn cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vector_int_R(camera_matrix: *const c_void, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, float, float, float, float, int, const std::vector<int> &) /usr/include/opencv2/rgbd/depth.hpp:1062
	pub fn cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vector_int_R(camera_matrix: *const c_void, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:1070
	pub fn cv_rgbd_FastICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:1072
	pub fn cv_rgbd_FastICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:1076
	pub fn cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMaxDistDiff() /usr/include/opencv2/rgbd/depth.hpp:1080
	pub fn cv_rgbd_FastICPOdometry_getMaxDistDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDistDiff(float) /usr/include/opencv2/rgbd/depth.hpp:1084
	pub fn cv_rgbd_FastICPOdometry_setMaxDistDiff_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result_void);
	// getAngleThreshold() /usr/include/opencv2/rgbd/depth.hpp:1088
	pub fn cv_rgbd_FastICPOdometry_getAngleThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setAngleThreshold(float) /usr/include/opencv2/rgbd/depth.hpp:1092
	pub fn cv_rgbd_FastICPOdometry_setAngleThreshold_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result_void);
	// getSigmaDepth() /usr/include/opencv2/rgbd/depth.hpp:1096
	pub fn cv_rgbd_FastICPOdometry_getSigmaDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSigmaDepth(float) /usr/include/opencv2/rgbd/depth.hpp:1100
	pub fn cv_rgbd_FastICPOdometry_setSigmaDepth_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result_void);
	// getSigmaSpatial() /usr/include/opencv2/rgbd/depth.hpp:1104
	pub fn cv_rgbd_FastICPOdometry_getSigmaSpatial_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setSigmaSpatial(float) /usr/include/opencv2/rgbd/depth.hpp:1108
	pub fn cv_rgbd_FastICPOdometry_setSigmaSpatial_float(instance: *mut c_void, f: f32, ocvrs_return: *mut Result_void);
	// getKernelSize() /usr/include/opencv2/rgbd/depth.hpp:1112
	pub fn cv_rgbd_FastICPOdometry_getKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setKernelSize(int) /usr/include/opencv2/rgbd/depth.hpp:1116
	pub fn cv_rgbd_FastICPOdometry_setKernelSize_int(instance: *mut c_void, f: i32, ocvrs_return: *mut Result_void);
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:1120
	pub fn cv_rgbd_FastICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:1124
	pub fn cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:1128
	pub fn cv_rgbd_FastICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:1132
	pub fn cv_rgbd_FastICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// ICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:762
	pub fn cv_rgbd_ICPOdometry_ICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
	// ICPOdometry(const cv::Mat &, float, float, float, float, const std::vector<int> &, int) /usr/include/opencv2/rgbd/depth.hpp:773
	pub fn cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, float, float, float, float, const std::vector<int> &, int) /usr/include/opencv2/rgbd/depth.hpp:777
	pub fn cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:781
	pub fn cv_rgbd_ICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:783
	pub fn cv_rgbd_ICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:787
	pub fn cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:791
	pub fn cv_rgbd_ICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:795
	pub fn cv_rgbd_ICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:799
	pub fn cv_rgbd_ICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:803
	pub fn cv_rgbd_ICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:807
	pub fn cv_rgbd_ICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:811
	pub fn cv_rgbd_ICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:815
	pub fn cv_rgbd_ICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:819
	pub fn cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:823
	pub fn cv_rgbd_ICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:827
	pub fn cv_rgbd_ICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:831
	pub fn cv_rgbd_ICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:835
	pub fn cv_rgbd_ICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:839
	pub fn cv_rgbd_ICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:843
	pub fn cv_rgbd_ICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:847
	pub fn cv_rgbd_ICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:851
	pub fn cv_rgbd_ICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getNormalsComputer() /usr/include/opencv2/rgbd/depth.hpp:855
	pub fn cv_rgbd_ICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DEFAULT_MIN_DEPTH() /usr/include/opencv2/rgbd/depth.hpp:535
	pub fn cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return: *mut Result<f32>);
	// DEFAULT_MAX_DEPTH() /usr/include/opencv2/rgbd/depth.hpp:540
	pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return: *mut Result<f32>);
	// DEFAULT_MAX_DEPTH_DIFF() /usr/include/opencv2/rgbd/depth.hpp:545
	pub fn cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return: *mut Result<f32>);
	// DEFAULT_MAX_POINTS_PART() /usr/include/opencv2/rgbd/depth.hpp:550
	pub fn cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return: *mut Result<f32>);
	// DEFAULT_MAX_TRANSLATION() /usr/include/opencv2/rgbd/depth.hpp:555
	pub fn cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return: *mut Result<f32>);
	// DEFAULT_MAX_ROTATION() /usr/include/opencv2/rgbd/depth.hpp:560
	pub fn cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return: *mut Result<f32>);
	// compute(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, cv::OutputArray, const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:584
	pub fn cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(instance: *const c_void, src_image: *const c_void, src_depth: *const c_void, src_mask: *const c_void, dst_image: *const c_void, dst_depth: *const c_void, dst_mask: *const c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
	// compute(Ptr<cv::rgbd::OdometryFrame> &, Ptr<cv::rgbd::OdometryFrame> &, cv::OutputArray, const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:591
	pub fn cv_rgbd_Odometry_compute_const_Ptr_OdometryFrame_R_Ptr_OdometryFrame_R_const__OutputArrayR_const_MatR(instance: *const c_void, src_frame: *mut c_void, dst_frame: *mut c_void, rt: *const c_void, init_rt: *const c_void, ocvrs_return: *mut Result<bool>);
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:599
	pub fn cv_rgbd_Odometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
	// create(const cv::String &) /usr/include/opencv2/rgbd/depth.hpp:601
	pub fn cv_rgbd_Odometry_create_const_StringR(odometry_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:604
	pub fn cv_rgbd_Odometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:606
	pub fn cv_rgbd_Odometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:608
	pub fn cv_rgbd_Odometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:610
	pub fn cv_rgbd_Odometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// pyramidImage /usr/include/opencv2/rgbd/depth.hpp:508
	pub fn cv_rgbd_OdometryFrame_getPropPyramidImage_const(instance: *const c_void) -> *mut c_void;
	// pyramidImage /usr/include/opencv2/rgbd/depth.hpp:508
	pub fn cv_rgbd_OdometryFrame_setPropPyramidImage_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidDepth /usr/include/opencv2/rgbd/depth.hpp:509
	pub fn cv_rgbd_OdometryFrame_getPropPyramidDepth_const(instance: *const c_void) -> *mut c_void;
	// pyramidDepth /usr/include/opencv2/rgbd/depth.hpp:509
	pub fn cv_rgbd_OdometryFrame_setPropPyramidDepth_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidMask /usr/include/opencv2/rgbd/depth.hpp:510
	pub fn cv_rgbd_OdometryFrame_getPropPyramidMask_const(instance: *const c_void) -> *mut c_void;
	// pyramidMask /usr/include/opencv2/rgbd/depth.hpp:510
	pub fn cv_rgbd_OdometryFrame_setPropPyramidMask_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidCloud /usr/include/opencv2/rgbd/depth.hpp:512
	pub fn cv_rgbd_OdometryFrame_getPropPyramidCloud_const(instance: *const c_void) -> *mut c_void;
	// pyramidCloud /usr/include/opencv2/rgbd/depth.hpp:512
	pub fn cv_rgbd_OdometryFrame_setPropPyramidCloud_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramid_dI_dx /usr/include/opencv2/rgbd/depth.hpp:514
	pub fn cv_rgbd_OdometryFrame_getPropPyramid_dI_dx_const(instance: *const c_void) -> *mut c_void;
	// pyramid_dI_dx /usr/include/opencv2/rgbd/depth.hpp:514
	pub fn cv_rgbd_OdometryFrame_setPropPyramid_dI_dx_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramid_dI_dy /usr/include/opencv2/rgbd/depth.hpp:515
	pub fn cv_rgbd_OdometryFrame_getPropPyramid_dI_dy_const(instance: *const c_void) -> *mut c_void;
	// pyramid_dI_dy /usr/include/opencv2/rgbd/depth.hpp:515
	pub fn cv_rgbd_OdometryFrame_setPropPyramid_dI_dy_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidTexturedMask /usr/include/opencv2/rgbd/depth.hpp:516
	pub fn cv_rgbd_OdometryFrame_getPropPyramidTexturedMask_const(instance: *const c_void) -> *mut c_void;
	// pyramidTexturedMask /usr/include/opencv2/rgbd/depth.hpp:516
	pub fn cv_rgbd_OdometryFrame_setPropPyramidTexturedMask_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidNormals /usr/include/opencv2/rgbd/depth.hpp:518
	pub fn cv_rgbd_OdometryFrame_getPropPyramidNormals_const(instance: *const c_void) -> *mut c_void;
	// pyramidNormals /usr/include/opencv2/rgbd/depth.hpp:518
	pub fn cv_rgbd_OdometryFrame_setPropPyramidNormals_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// pyramidNormalsMask /usr/include/opencv2/rgbd/depth.hpp:519
	pub fn cv_rgbd_OdometryFrame_getPropPyramidNormalsMask_const(instance: *const c_void) -> *mut c_void;
	// pyramidNormalsMask /usr/include/opencv2/rgbd/depth.hpp:519
	pub fn cv_rgbd_OdometryFrame_setPropPyramidNormalsMask_vector_Mat_(instance: *mut c_void, val: *mut c_void);
	// OdometryFrame() /usr/include/opencv2/rgbd/depth.hpp:497
	pub fn cv_rgbd_OdometryFrame_OdometryFrame(ocvrs_return: *mut Result<*mut c_void>);
	// OdometryFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:498
	pub fn cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:500
	pub fn cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// release() /usr/include/opencv2/rgbd/depth.hpp:503
	pub fn cv_rgbd_OdometryFrame_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// releasePyramids() /usr/include/opencv2/rgbd/depth.hpp:506
	pub fn cv_rgbd_OdometryFrame_releasePyramids(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// ID /usr/include/opencv2/rgbd/depth.hpp:472
	pub fn cv_rgbd_RgbdFrame_getPropID_const(instance: *const c_void) -> i32;
	// ID /usr/include/opencv2/rgbd/depth.hpp:472
	pub fn cv_rgbd_RgbdFrame_setPropID_int(instance: *mut c_void, val: i32);
	// image /usr/include/opencv2/rgbd/depth.hpp:473
	pub fn cv_rgbd_RgbdFrame_getPropImage_const(instance: *const c_void) -> *mut c_void;
	// image /usr/include/opencv2/rgbd/depth.hpp:473
	pub fn cv_rgbd_RgbdFrame_setPropImage_Mat(instance: *mut c_void, val: *mut c_void);
	// depth /usr/include/opencv2/rgbd/depth.hpp:474
	pub fn cv_rgbd_RgbdFrame_getPropDepth_const(instance: *const c_void) -> *mut c_void;
	// depth /usr/include/opencv2/rgbd/depth.hpp:474
	pub fn cv_rgbd_RgbdFrame_setPropDepth_Mat(instance: *mut c_void, val: *mut c_void);
	// mask /usr/include/opencv2/rgbd/depth.hpp:475
	pub fn cv_rgbd_RgbdFrame_getPropMask_const(instance: *const c_void) -> *mut c_void;
	// mask /usr/include/opencv2/rgbd/depth.hpp:475
	pub fn cv_rgbd_RgbdFrame_setPropMask_Mat(instance: *mut c_void, val: *mut c_void);
	// normals /usr/include/opencv2/rgbd/depth.hpp:476
	pub fn cv_rgbd_RgbdFrame_getPropNormals_const(instance: *const c_void) -> *mut c_void;
	// normals /usr/include/opencv2/rgbd/depth.hpp:476
	pub fn cv_rgbd_RgbdFrame_setPropNormals_Mat(instance: *mut c_void, val: *mut c_void);
	// RgbdFrame() /usr/include/opencv2/rgbd/depth.hpp:463
	pub fn cv_rgbd_RgbdFrame_RgbdFrame(ocvrs_return: *mut Result<*mut c_void>);
	// RgbdFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:464
	pub fn cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:467
	pub fn cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image: *const c_void, depth: *const c_void, mask: *const c_void, normals: *const c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// release() /usr/include/opencv2/rgbd/depth.hpp:470
	pub fn cv_rgbd_RgbdFrame_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// RgbdICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:890
	pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(ocvrs_return: *mut Result<*mut c_void>);
	// RgbdICPOdometry(const cv::Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int) /usr/include/opencv2/rgbd/depth.hpp:903
	pub fn cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int) /usr/include/opencv2/rgbd/depth.hpp:909
	pub fn cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:915
	pub fn cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:917
	pub fn cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:921
	pub fn cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:925
	pub fn cv_rgbd_RgbdICPOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:929
	pub fn cv_rgbd_RgbdICPOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:933
	pub fn cv_rgbd_RgbdICPOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:937
	pub fn cv_rgbd_RgbdICPOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:941
	pub fn cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:945
	pub fn cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:949
	pub fn cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:953
	pub fn cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:957
	pub fn cv_rgbd_RgbdICPOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:961
	pub fn cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMinGradientMagnitudes() /usr/include/opencv2/rgbd/depth.hpp:965
	pub fn cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMinGradientMagnitudes(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:969
	pub fn cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:973
	pub fn cv_rgbd_RgbdICPOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:977
	pub fn cv_rgbd_RgbdICPOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:981
	pub fn cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:985
	pub fn cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:989
	pub fn cv_rgbd_RgbdICPOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:993
	pub fn cv_rgbd_RgbdICPOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getNormalsComputer() /usr/include/opencv2/rgbd/depth.hpp:997
	pub fn cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RgbdNormals() /usr/include/opencv2/rgbd/depth.hpp:83
	pub fn cv_rgbd_RgbdNormals_RgbdNormals(ocvrs_return: *mut Result<*mut c_void>);
	// RgbdNormals(int, int, int, cv::InputArray, int, int) /usr/include/opencv2/rgbd/depth.hpp:103
	pub fn cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, cv::InputArray, int, int) /usr/include/opencv2/rgbd/depth.hpp:108
	pub fn cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows: i32, cols: i32, depth: i32, k: *const c_void, window_size: i32, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// initialize() /usr/include/opencv2/rgbd/depth.hpp:122
	pub fn cv_rgbd_RgbdNormals_initialize_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// getRows() /usr/include/opencv2/rgbd/depth.hpp:124
	pub fn cv_rgbd_RgbdNormals_getRows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRows(int) /usr/include/opencv2/rgbd/depth.hpp:128
	pub fn cv_rgbd_RgbdNormals_setRows_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getCols() /usr/include/opencv2/rgbd/depth.hpp:132
	pub fn cv_rgbd_RgbdNormals_getCols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setCols(int) /usr/include/opencv2/rgbd/depth.hpp:136
	pub fn cv_rgbd_RgbdNormals_setCols_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/rgbd/depth.hpp:140
	pub fn cv_rgbd_RgbdNormals_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWindowSize(int) /usr/include/opencv2/rgbd/depth.hpp:144
	pub fn cv_rgbd_RgbdNormals_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getDepth() /usr/include/opencv2/rgbd/depth.hpp:148
	pub fn cv_rgbd_RgbdNormals_getDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setDepth(int) /usr/include/opencv2/rgbd/depth.hpp:152
	pub fn cv_rgbd_RgbdNormals_setDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getK() /usr/include/opencv2/rgbd/depth.hpp:156
	pub fn cv_rgbd_RgbdNormals_getK_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setK(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:160
	pub fn cv_rgbd_RgbdNormals_setK_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:164
	pub fn cv_rgbd_RgbdNormals_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:168
	pub fn cv_rgbd_RgbdNormals_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// RgbdOdometry() /usr/include/opencv2/rgbd/depth.hpp:627
	pub fn cv_rgbd_RgbdOdometry_RgbdOdometry(ocvrs_return: *mut Result<*mut c_void>);
	// RgbdOdometry(const cv::Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int) /usr/include/opencv2/rgbd/depth.hpp:640
	pub fn cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int) /usr/include/opencv2/rgbd/depth.hpp:645
	pub fn cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(camera_matrix: *const c_void, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: *const c_void, min_gradient_magnitudes: *const c_void, max_points_part: f32, transform_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:650
	pub fn cv_rgbd_RgbdOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(instance: *const c_void, frame: *mut c_void, cache_type: i32, ocvrs_return: *mut Result<core::Size>);
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:652
	pub fn cv_rgbd_RgbdOdometry_getCameraMatrix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:656
	pub fn cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:660
	pub fn cv_rgbd_RgbdOdometry_getMinDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:664
	pub fn cv_rgbd_RgbdOdometry_setMinDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:668
	pub fn cv_rgbd_RgbdOdometry_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:672
	pub fn cv_rgbd_RgbdOdometry_setMaxDepth_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:676
	pub fn cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:680
	pub fn cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:684
	pub fn cv_rgbd_RgbdOdometry_getIterationCounts_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:688
	pub fn cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMinGradientMagnitudes() /usr/include/opencv2/rgbd/depth.hpp:692
	pub fn cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMinGradientMagnitudes(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:696
	pub fn cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:700
	pub fn cv_rgbd_RgbdOdometry_getMaxPointsPart_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:704
	pub fn cv_rgbd_RgbdOdometry_setMaxPointsPart_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:708
	pub fn cv_rgbd_RgbdOdometry_getTransformType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:712
	pub fn cv_rgbd_RgbdOdometry_setTransformType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:716
	pub fn cv_rgbd_RgbdOdometry_getMaxTranslation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:720
	pub fn cv_rgbd_RgbdOdometry_setMaxTranslation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:724
	pub fn cv_rgbd_RgbdOdometry_getMaxRotation_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:728
	pub fn cv_rgbd_RgbdOdometry_setMaxRotation_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// RgbdPlane(int) /usr/include/opencv2/rgbd/depth.hpp:337
	pub fn cv_rgbd_RgbdPlane_RgbdPlane_int(method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// RgbdPlane(int, int, int, double, double, double, double) /usr/include/opencv2/rgbd/depth.hpp:358
	pub fn cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, double, double, double, double) /usr/include/opencv2/rgbd/depth.hpp:364
	pub fn cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, ocvrs_return: *mut Result<*mut c_void>);
	// getBlockSize() /usr/include/opencv2/rgbd/depth.hpp:389
	pub fn cv_rgbd_RgbdPlane_getBlockSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setBlockSize(int) /usr/include/opencv2/rgbd/depth.hpp:393
	pub fn cv_rgbd_RgbdPlane_setBlockSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMinSize() /usr/include/opencv2/rgbd/depth.hpp:397
	pub fn cv_rgbd_RgbdPlane_getMinSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMinSize(int) /usr/include/opencv2/rgbd/depth.hpp:401
	pub fn cv_rgbd_RgbdPlane_setMinSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:405
	pub fn cv_rgbd_RgbdPlane_getMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:409
	pub fn cv_rgbd_RgbdPlane_setMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/rgbd/depth.hpp:413
	pub fn cv_rgbd_RgbdPlane_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setThreshold(double) /usr/include/opencv2/rgbd/depth.hpp:417
	pub fn cv_rgbd_RgbdPlane_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getSensorErrorA() /usr/include/opencv2/rgbd/depth.hpp:421
	pub fn cv_rgbd_RgbdPlane_getSensorErrorA_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSensorErrorA(double) /usr/include/opencv2/rgbd/depth.hpp:425
	pub fn cv_rgbd_RgbdPlane_setSensorErrorA_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getSensorErrorB() /usr/include/opencv2/rgbd/depth.hpp:429
	pub fn cv_rgbd_RgbdPlane_getSensorErrorB_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSensorErrorB(double) /usr/include/opencv2/rgbd/depth.hpp:433
	pub fn cv_rgbd_RgbdPlane_setSensorErrorB_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getSensorErrorC() /usr/include/opencv2/rgbd/depth.hpp:437
	pub fn cv_rgbd_RgbdPlane_getSensorErrorC_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSensorErrorC(double) /usr/include/opencv2/rgbd/depth.hpp:441
	pub fn cv_rgbd_RgbdPlane_setSensorErrorC_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
}
