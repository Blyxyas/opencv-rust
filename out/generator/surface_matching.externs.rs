extern "C" {
	// ICP() /usr/include/opencv2/surface_matching/icp.hpp:90
	pub fn cv_ppf_match_3d_ICP_ICP(ocvrs_return: *mut Result<*mut c_void>);
	// ICP(const int, const float, const float, const int, const int, const int) /usr/include/opencv2/surface_matching/icp.hpp:117
	pub fn cv_ppf_match_3d_ICP_ICP_const_int_const_float_const_float_const_int_const_int_const_int(iterations: i32, tolerence: f32, rejection_scale: f32, num_levels: i32, sample_type: i32, num_max_corr: i32, ocvrs_return: *mut Result<*mut c_void>);
	// registerModelToScene(const cv::Mat &, const cv::Mat &, double &, cv::Matx44d &) /usr/include/opencv2/surface_matching/icp.hpp:139
	pub fn cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_doubleR_Matx44dR(instance: *mut c_void, src_pc: *const c_void, dst_pc: *const c_void, residual: *mut f64, pose: *mut core::Matx44d, ocvrs_return: *mut Result<i32>);
	// registerModelToScene(const cv::Mat &, const cv::Mat &, std::vector<Pose3DPtr> &) /usr/include/opencv2/surface_matching/icp.hpp:152
	pub fn cv_ppf_match_3d_ICP_registerModelToScene_const_MatR_const_MatR_vector_Pose3DPtr_R(instance: *mut c_void, src_pc: *const c_void, dst_pc: *const c_void, poses: *mut c_void, ocvrs_return: *mut Result<i32>);
	// PPF3DDetector() /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:104
	pub fn cv_ppf_match_3d_PPF3DDetector_PPF3DDetector(ocvrs_return: *mut Result<*mut c_void>);
	// PPF3DDetector(const double, const double, const double) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:112
	pub fn cv_ppf_match_3d_PPF3DDetector_PPF3DDetector_const_double_const_double_const_double(relative_sampling_step: f64, relative_distance_step: f64, num_angles: f64, ocvrs_return: *mut Result<*mut c_void>);
	// setSearchParams(const double, const double, const bool) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:122
	pub fn cv_ppf_match_3d_PPF3DDetector_setSearchParams_const_double_const_double_const_bool(instance: *mut c_void, position_threshold: f64, rotation_threshold: f64, use_weighted_clustering: bool, ocvrs_return: *mut Result_void);
	// trainModel(const cv::Mat &) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:131
	pub fn cv_ppf_match_3d_PPF3DDetector_trainModel_const_MatR(instance: *mut c_void, model: *const c_void, ocvrs_return: *mut Result_void);
	// match(const cv::Mat &, std::vector<Pose3DPtr> &, const double, const double) /usr/include/opencv2/surface_matching/ppf_match_3d.hpp:141
	pub fn cv_ppf_match_3d_PPF3DDetector_match_const_MatR_vector_Pose3DPtr_R_const_double_const_double(instance: *mut c_void, scene: *const c_void, results: *mut c_void, relative_scene_sample_step: f64, relative_scene_distance: f64, ocvrs_return: *mut Result_void);
	// alpha /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	pub fn cv_ppf_match_3d_Pose3D_getPropAlpha_const(instance: *const c_void) -> f64;
	// alpha /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	pub fn cv_ppf_match_3d_Pose3D_setPropAlpha_double(instance: *mut c_void, val: f64);
	// residual /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	pub fn cv_ppf_match_3d_Pose3D_getPropResidual_const(instance: *const c_void) -> f64;
	// residual /usr/include/opencv2/surface_matching/pose_3d.hpp:125
	pub fn cv_ppf_match_3d_Pose3D_setPropResidual_double(instance: *mut c_void, val: f64);
	// modelIndex /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	pub fn cv_ppf_match_3d_Pose3D_getPropModelIndex_const(instance: *const c_void) -> size_t;
	// modelIndex /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	pub fn cv_ppf_match_3d_Pose3D_setPropModelIndex_size_t(instance: *mut c_void, val: size_t);
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	pub fn cv_ppf_match_3d_Pose3D_getPropNumVotes_const(instance: *const c_void) -> size_t;
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:126
	pub fn cv_ppf_match_3d_Pose3D_setPropNumVotes_size_t(instance: *mut c_void, val: size_t);
	// pose /usr/include/opencv2/surface_matching/pose_3d.hpp:127
	pub fn cv_ppf_match_3d_Pose3D_getPropPose_const(instance: *const c_void, ocvrs_return: *mut core::Matx44d);
	// pose /usr/include/opencv2/surface_matching/pose_3d.hpp:127
	pub fn cv_ppf_match_3d_Pose3D_setPropPose_Matx44d(instance: *mut c_void, val: *const core::Matx44d);
	// angle /usr/include/opencv2/surface_matching/pose_3d.hpp:128
	pub fn cv_ppf_match_3d_Pose3D_getPropAngle_const(instance: *const c_void) -> f64;
	// angle /usr/include/opencv2/surface_matching/pose_3d.hpp:128
	pub fn cv_ppf_match_3d_Pose3D_setPropAngle_double(instance: *mut c_void, val: f64);
	// t /usr/include/opencv2/surface_matching/pose_3d.hpp:129
	pub fn cv_ppf_match_3d_Pose3D_getPropT_const(instance: *const c_void, ocvrs_return: *mut core::Vec3d);
	// t /usr/include/opencv2/surface_matching/pose_3d.hpp:129
	pub fn cv_ppf_match_3d_Pose3D_setPropT_Vec3d(instance: *mut c_void, val: *const core::Vec3d);
	// q /usr/include/opencv2/surface_matching/pose_3d.hpp:130
	pub fn cv_ppf_match_3d_Pose3D_getPropQ_const(instance: *const c_void, ocvrs_return: *mut core::Vec4d);
	// q /usr/include/opencv2/surface_matching/pose_3d.hpp:130
	pub fn cv_ppf_match_3d_Pose3D_setPropQ_Vec4d(instance: *mut c_void, val: *const core::Vec4d);
	// Pose3D() /usr/include/opencv2/surface_matching/pose_3d.hpp:73
	pub fn cv_ppf_match_3d_Pose3D_Pose3D(ocvrs_return: *mut Result<*mut c_void>);
	// Pose3D(double, size_t, size_t) /usr/include/opencv2/surface_matching/pose_3d.hpp:83
	pub fn cv_ppf_match_3d_Pose3D_Pose3D_double_size_t_size_t(alpha: f64, model_index: size_t, num_votes: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// updatePose(cv::Matx44d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:97
	pub fn cv_ppf_match_3d_Pose3D_updatePose_Matx44dR(instance: *mut c_void, new_pose: *mut core::Matx44d, ocvrs_return: *mut Result_void);
	// updatePose(cv::Matx33d &, cv::Vec3d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:102
	pub fn cv_ppf_match_3d_Pose3D_updatePose_Matx33dR_Vec3dR(instance: *mut c_void, new_r: *mut core::Matx33d, new_t: *mut core::Vec3d, ocvrs_return: *mut Result_void);
	// updatePoseQuat(cv::Vec4d &, cv::Vec3d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:107
	pub fn cv_ppf_match_3d_Pose3D_updatePoseQuat_Vec4dR_Vec3dR(instance: *mut c_void, q: *mut core::Vec4d, new_t: *mut core::Vec3d, ocvrs_return: *mut Result_void);
	// appendPose(cv::Matx44d &) /usr/include/opencv2/surface_matching/pose_3d.hpp:113
	pub fn cv_ppf_match_3d_Pose3D_appendPose_Matx44dR(instance: *mut c_void, incremental_pose: *mut core::Matx44d, ocvrs_return: *mut Result_void);
	// printPose() /usr/include/opencv2/surface_matching/pose_3d.hpp:114
	pub fn cv_ppf_match_3d_Pose3D_printPose(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// clone() /usr/include/opencv2/surface_matching/pose_3d.hpp:116
	pub fn cv_ppf_match_3d_Pose3D_clone(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// writePose(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:120
	pub fn cv_ppf_match_3d_Pose3D_writePose_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
	// readPose(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:121
	pub fn cv_ppf_match_3d_Pose3D_readPose_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
	// poseList /usr/include/opencv2/surface_matching/pose_3d.hpp:177
	pub fn cv_ppf_match_3d_PoseCluster3D_getPropPoseList_const(instance: *const c_void) -> *mut c_void;
	// poseList /usr/include/opencv2/surface_matching/pose_3d.hpp:177
	pub fn cv_ppf_match_3d_PoseCluster3D_setPropPoseList_vector_Pose3DPtr_(instance: *mut c_void, val: *mut c_void);
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:178
	pub fn cv_ppf_match_3d_PoseCluster3D_getPropNumVotes_const(instance: *const c_void) -> size_t;
	// numVotes /usr/include/opencv2/surface_matching/pose_3d.hpp:178
	pub fn cv_ppf_match_3d_PoseCluster3D_setPropNumVotes_size_t(instance: *mut c_void, val: size_t);
	// id /usr/include/opencv2/surface_matching/pose_3d.hpp:179
	pub fn cv_ppf_match_3d_PoseCluster3D_getPropId_const(instance: *const c_void) -> i32;
	// id /usr/include/opencv2/surface_matching/pose_3d.hpp:179
	pub fn cv_ppf_match_3d_PoseCluster3D_setPropId_int(instance: *mut c_void, val: i32);
	// PoseCluster3D() /usr/include/opencv2/surface_matching/pose_3d.hpp:141
	pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D(ocvrs_return: *mut Result<*mut c_void>);
	// PoseCluster3D(cv::ppf_match_3d::Pose3DPtr) /usr/include/opencv2/surface_matching/pose_3d.hpp:147
	pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr(new_pose: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PoseCluster3D(cv::ppf_match_3d::Pose3DPtr, int) /usr/include/opencv2/surface_matching/pose_3d.hpp:155
	pub fn cv_ppf_match_3d_PoseCluster3D_PoseCluster3D_Pose3DPtr_int(new_pose: *mut c_void, new_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// addPose(cv::ppf_match_3d::Pose3DPtr) /usr/include/opencv2/surface_matching/pose_3d.hpp:170
	pub fn cv_ppf_match_3d_PoseCluster3D_addPose_Pose3DPtr(instance: *mut c_void, new_pose: *mut c_void, ocvrs_return: *mut Result_void);
	// writePoseCluster(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:174
	pub fn cv_ppf_match_3d_PoseCluster3D_writePoseCluster_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
	// readPoseCluster(const std::string &) /usr/include/opencv2/surface_matching/pose_3d.hpp:175
	pub fn cv_ppf_match_3d_PoseCluster3D_readPoseCluster_const_stringR(instance: *mut c_void, file_name: *const c_char, ocvrs_return: *mut Result<i32>);
}
