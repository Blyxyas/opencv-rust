#include "ocvrs_common.hpp"
#include <opencv2/rgbd.hpp>
#include "rgbd_types.hpp"

extern "C" {
	// makeVolume(cv::kinfu::VolumeType, float, cv::Matx44f, float, float, int, float, cv::Vec3i) /usr/include/opencv2/rgbd/volume.hpp:122
	void cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(cv::kinfu::VolumeType _volumeType, float _voxelSize, cv::Matx44f* _pose, float _raycastStepFactor, float _truncDist, int _maxWeight, float _truncateThreshold, cv::Vec3i* _resolution, Result<cv::Ptr<cv::kinfu::Volume>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Volume> ret = cv::kinfu::makeVolume(_volumeType, _voxelSize, *_pose, _raycastStepFactor, _truncDist, _maxWeight, _truncateThreshold, *_resolution);
			Ok(new cv::Ptr<cv::kinfu::Volume>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::Volume>*>))
	}
	
	// colormap(const cv::Mat &, cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:245
	void cv_linemod_colormap_const_MatR_MatR(const cv::Mat* quantized, cv::Mat* dst, Result_void* ocvrs_return) {
		try {
			cv::linemod::colormap(*quantized, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawFeatures(cv::InputOutputArray, const std::vector<Template> &, const cv::Point2i &, int) /usr/include/opencv2/rgbd/linemod.hpp:254
	void cv_linemod_drawFeatures_const__InputOutputArrayR_const_vector_Template_R_const_Point2iR_int(const cv::_InputOutputArray* img, const std::vector<cv::linemod::Template>* templates, const cv::Point2i* tl, int size, Result_void* ocvrs_return) {
		try {
			cv::linemod::drawFeatures(*img, *templates, *tl, size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultLINE() /usr/include/opencv2/rgbd/linemod.hpp:420
	void cv_linemod_getDefaultLINE(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINE();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Detector>*>))
	}
	
	// getDefaultLINEMOD() /usr/include/opencv2/rgbd/linemod.hpp:428
	void cv_linemod_getDefaultLINEMOD(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINEMOD();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Detector>*>))
	}
	
	// depthTo3dSparse(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/depth.hpp:299
	void cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* in_K, const cv::_InputArray* in_points, const cv::_OutputArray* points3d, Result_void* ocvrs_return) {
		try {
			cv::rgbd::depthTo3dSparse(*depth, *in_K, *in_points, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// depthTo3d(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/rgbd/depth.hpp:312
	void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isValidDepth(const double &) /usr/include/opencv2/rgbd/depth.hpp:33
	void cv_rgbd_isValidDepth_const_doubleR(const double* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isValidDepth(const float &) /usr/include/opencv2/rgbd/depth.hpp:27
	void cv_rgbd_isValidDepth_const_floatR(const float* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isValidDepth(const int &) /usr/include/opencv2/rgbd/depth.hpp:52
	void cv_rgbd_isValidDepth_const_intR(const int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isValidDepth(const short &) /usr/include/opencv2/rgbd/depth.hpp:39
	void cv_rgbd_isValidDepth_const_shortR(const short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isValidDepth(const unsigned int &) /usr/include/opencv2/rgbd/depth.hpp:58
	void cv_rgbd_isValidDepth_const_unsigned_intR(const unsigned int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isValidDepth(const unsigned short &) /usr/include/opencv2/rgbd/depth.hpp:45
	void cv_rgbd_isValidDepth_const_unsigned_shortR(const unsigned short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// registerDepth(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, const cv::Size &, cv::OutputArray, bool) /usr/include/opencv2/rgbd/depth.hpp:287
	void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, bool depthDilation, Result_void* ocvrs_return) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth, depthDilation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rescaleDepth(cv::InputArray, int, cv::OutputArray, double) /usr/include/opencv2/rgbd/depth.hpp:325
	void cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(const cv::_InputArray* in, int depth, const cv::_OutputArray* out, double depth_factor, Result_void* ocvrs_return) {
		try {
			cv::rgbd::rescaleDepth(*in, depth, *out, depth_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// warpFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/depth.hpp:1179
	void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, const cv::_OutputArray* warpedDepth, const cv::_OutputArray* warpedMask, Result_void* ocvrs_return) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage, *warpedDepth, *warpedMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const Ptr<cv::colored_kinfu::Params> &) /usr/include/opencv2/rgbd/colored_kinfu.hpp:198
	void cv_colored_kinfu_ColoredKinFu_create_const_Ptr_Params_R(const cv::Ptr<cv::colored_kinfu::Params>* _params, Result<cv::Ptr<cv::colored_kinfu::ColoredKinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::ColoredKinFu> ret = cv::colored_kinfu::ColoredKinFu::create(*_params);
			Ok(new cv::Ptr<cv::colored_kinfu::ColoredKinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::colored_kinfu::ColoredKinFu>*>))
	}
	
	// getParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:202
	void cv_colored_kinfu_ColoredKinFu_getParams_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::colored_kinfu::Params ret = instance->getParams();
			Ok(new const cv::colored_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::colored_kinfu::Params*>))
	}
	
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:212
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, Result_void* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/colored_kinfu.hpp:224
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, Result_void* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCloud(cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:235
	void cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, Result_void* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:243
	void cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, Result_void* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:249
	void cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/rgbd/colored_kinfu.hpp:255
	void cv_colored_kinfu_ColoredKinFu_reset(cv::colored_kinfu::ColoredKinFu* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPose() /usr/include/opencv2/rgbd/colored_kinfu.hpp:258
	void cv_colored_kinfu_ColoredKinFu_getPose_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3f>))
	}
	
	// update(cv::InputArray, cv::InputArray) /usr/include/opencv2/rgbd/colored_kinfu.hpp:266
	void cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(cv::colored_kinfu::ColoredKinFu* instance, const cv::_InputArray* depth, const cv::_InputArray* rgb, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth, *rgb);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:83
	void cv_colored_kinfu_Params_getPropFrameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}
	
	// frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:83
	void cv_colored_kinfu_Params_setPropFrameSize_Size(cv::colored_kinfu::Params* instance, cv::Size* val) {
			instance->frameSize = *val;
	}
	
	// rgb_frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:86
	void cv_colored_kinfu_Params_getPropRgb_frameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->rgb_frameSize;
			*ocvrs_return = ret;
	}
	
	// rgb_frameSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:86
	void cv_colored_kinfu_Params_setPropRgb_frameSize_Size(cv::colored_kinfu::Params* instance, cv::Size* val) {
			instance->rgb_frameSize = *val;
	}
	
	// volumeType /usr/include/opencv2/rgbd/colored_kinfu.hpp:88
	void cv_colored_kinfu_Params_getPropVolumeType_const(const cv::colored_kinfu::Params* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->volumeType;
			*ocvrs_return = ret;
	}
	
	// volumeType /usr/include/opencv2/rgbd/colored_kinfu.hpp:88
	void cv_colored_kinfu_Params_setPropVolumeType_VolumeType(cv::colored_kinfu::Params* instance, cv::kinfu::VolumeType val) {
			instance->volumeType = val;
	}
	
	// intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:91
	void cv_colored_kinfu_Params_getPropIntr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}
	
	// intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:91
	void cv_colored_kinfu_Params_setPropIntr_Matx33f(cv::colored_kinfu::Params* instance, cv::Matx33f* val) {
			instance->intr = *val;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:94
	void cv_colored_kinfu_Params_getPropRgb_intr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/colored_kinfu.hpp:94
	void cv_colored_kinfu_Params_setPropRgb_intr_Matx33f(cv::colored_kinfu::Params* instance, cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/colored_kinfu.hpp:103
	float cv_colored_kinfu_Params_getPropDepthFactor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/colored_kinfu.hpp:103
	void cv_colored_kinfu_Params_setPropDepthFactor_float(cv::colored_kinfu::Params* instance, float val) {
			instance->depthFactor = val;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/colored_kinfu.hpp:106
	float cv_colored_kinfu_Params_getPropBilateral_sigma_depth_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/colored_kinfu.hpp:106
	void cv_colored_kinfu_Params_setPropBilateral_sigma_depth_float(cv::colored_kinfu::Params* instance, float val) {
			instance->bilateral_sigma_depth = val;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/colored_kinfu.hpp:108
	float cv_colored_kinfu_Params_getPropBilateral_sigma_spatial_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/colored_kinfu.hpp:108
	void cv_colored_kinfu_Params_setPropBilateral_sigma_spatial_float(cv::colored_kinfu::Params* instance, float val) {
			instance->bilateral_sigma_spatial = val;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/colored_kinfu.hpp:110
	int cv_colored_kinfu_Params_getPropBilateral_kernel_size_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/colored_kinfu.hpp:110
	void cv_colored_kinfu_Params_setPropBilateral_kernel_size_int(cv::colored_kinfu::Params* instance, int val) {
			instance->bilateral_kernel_size = val;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/colored_kinfu.hpp:113
	int cv_colored_kinfu_Params_getPropPyramidLevels_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/colored_kinfu.hpp:113
	void cv_colored_kinfu_Params_setPropPyramidLevels_int(cv::colored_kinfu::Params* instance, int val) {
			instance->pyramidLevels = val;
	}
	
	// volumeDims /usr/include/opencv2/rgbd/colored_kinfu.hpp:119
	void cv_colored_kinfu_Params_getPropVolumeDims_const(const cv::colored_kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}
	
	// volumeDims /usr/include/opencv2/rgbd/colored_kinfu.hpp:119
	void cv_colored_kinfu_Params_setPropVolumeDims_Vec3i(cv::colored_kinfu::Params* instance, cv::Vec3i* val) {
			instance->volumeDims = *val;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:121
	float cv_colored_kinfu_Params_getPropVoxelSize_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/colored_kinfu.hpp:121
	void cv_colored_kinfu_Params_setPropVoxelSize_float(cv::colored_kinfu::Params* instance, float val) {
			instance->voxelSize = val;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/colored_kinfu.hpp:127
	float cv_colored_kinfu_Params_getPropTsdf_min_camera_movement_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/colored_kinfu.hpp:127
	void cv_colored_kinfu_Params_setPropTsdf_min_camera_movement_float(cv::colored_kinfu::Params* instance, float val) {
			instance->tsdf_min_camera_movement = val;
	}
	
	// volumePose /usr/include/opencv2/rgbd/colored_kinfu.hpp:130
	void cv_colored_kinfu_Params_getPropVolumePose_const(const cv::colored_kinfu::Params* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->volumePose;
			*ocvrs_return = ret;
	}
	
	// volumePose /usr/include/opencv2/rgbd/colored_kinfu.hpp:130
	void cv_colored_kinfu_Params_setPropVolumePose_Affine3f(cv::colored_kinfu::Params* instance, cv::Affine3f* val) {
			instance->volumePose = *val;
	}
	
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/colored_kinfu.hpp:136
	float cv_colored_kinfu_Params_getPropTsdf_trunc_dist_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}
	
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/colored_kinfu.hpp:136
	void cv_colored_kinfu_Params_setPropTsdf_trunc_dist_float(cv::colored_kinfu::Params* instance, float val) {
			instance->tsdf_trunc_dist = val;
	}
	
	// tsdf_max_weight /usr/include/opencv2/rgbd/colored_kinfu.hpp:142
	int cv_colored_kinfu_Params_getPropTsdf_max_weight_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}
	
	// tsdf_max_weight /usr/include/opencv2/rgbd/colored_kinfu.hpp:142
	void cv_colored_kinfu_Params_setPropTsdf_max_weight_int(cv::colored_kinfu::Params* instance, int val) {
			instance->tsdf_max_weight = val;
	}
	
	// raycast_step_factor /usr/include/opencv2/rgbd/colored_kinfu.hpp:148
	float cv_colored_kinfu_Params_getPropRaycast_step_factor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}
	
	// raycast_step_factor /usr/include/opencv2/rgbd/colored_kinfu.hpp:148
	void cv_colored_kinfu_Params_setPropRaycast_step_factor_float(cv::colored_kinfu::Params* instance, float val) {
			instance->raycast_step_factor = val;
	}
	
	// lightPose /usr/include/opencv2/rgbd/colored_kinfu.hpp:155
	void cv_colored_kinfu_Params_getPropLightPose_const(const cv::colored_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}
	
	// lightPose /usr/include/opencv2/rgbd/colored_kinfu.hpp:155
	void cv_colored_kinfu_Params_setPropLightPose_Vec3f(cv::colored_kinfu::Params* instance, cv::Vec3f* val) {
			instance->lightPose = *val;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:158
	float cv_colored_kinfu_Params_getPropIcpDistThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:158
	void cv_colored_kinfu_Params_setPropIcpDistThresh_float(cv::colored_kinfu::Params* instance, float val) {
			instance->icpDistThresh = val;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:160
	float cv_colored_kinfu_Params_getPropIcpAngleThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/colored_kinfu.hpp:160
	void cv_colored_kinfu_Params_setPropIcpAngleThresh_float(cv::colored_kinfu::Params* instance, float val) {
			instance->icpAngleThresh = val;
	}
	
	// icpIterations /usr/include/opencv2/rgbd/colored_kinfu.hpp:162
	std::vector<int>* cv_colored_kinfu_Params_getPropIcpIterations_const(const cv::colored_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}
	
	// icpIterations /usr/include/opencv2/rgbd/colored_kinfu.hpp:162
	void cv_colored_kinfu_Params_setPropIcpIterations_vector_int_(cv::colored_kinfu::Params* instance, std::vector<int>* val) {
			instance->icpIterations = *val;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/colored_kinfu.hpp:168
	float cv_colored_kinfu_Params_getPropTruncateThreshold_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/colored_kinfu.hpp:168
	void cv_colored_kinfu_Params_setPropTruncateThreshold_float(cv::colored_kinfu::Params* instance, float val) {
			instance->truncateThreshold = val;
	}
	
	void cv_ColoredKinfu_Params_delete(cv::colored_kinfu::Params* instance) {
		delete instance;
	}
	// Params() /usr/include/opencv2/rgbd/colored_kinfu.hpp:22
	void cv_colored_kinfu_Params_Params(Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::colored_kinfu::Params*>))
	}
	
	// Params(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:30
	void cv_colored_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::colored_kinfu::Params*>))
	}
	
	// Params(cv::Matx44f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:40
	void cv_colored_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::colored_kinfu::Params*>))
	}
	
	// setInitialVolumePose(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:51
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::colored_kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, Result_void* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInitialVolumePose(cv::Matx44f) /usr/include/opencv2/rgbd/colored_kinfu.hpp:58
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(cv::colored_kinfu::Params* instance, cv::Matx44f* homogen_tf, Result_void* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// defaultParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:64
	void cv_colored_kinfu_Params_defaultParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::colored_kinfu::Params>*>))
	}
	
	// coarseParams() /usr/include/opencv2/rgbd/colored_kinfu.hpp:70
	void cv_colored_kinfu_Params_coarseParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::colored_kinfu::Params>*>))
	}
	
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/colored_kinfu.hpp:75
	void cv_colored_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::colored_kinfu::Params>*>))
	}
	
	// coloredTSDFParams(bool) /usr/include/opencv2/rgbd/colored_kinfu.hpp:80
	void cv_colored_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::colored_kinfu::Params>*>))
	}
	
	// create(const Ptr<kinfu::Params> &) /usr/include/opencv2/rgbd/dynafu.hpp:49
	void cv_dynafu_DynaFu_create_const_Ptr_Params_R(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::dynafu::DynaFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dynafu::DynaFu> ret = cv::dynafu::DynaFu::create(*_params);
			Ok(new cv::Ptr<cv::dynafu::DynaFu>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dynafu::DynaFu>*>))
	}
	
	// getParams() /usr/include/opencv2/rgbd/dynafu.hpp:53
	void cv_dynafu_DynaFu_getParams_const(const cv::dynafu::DynaFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Params*>))
	}
	
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/dynafu.hpp:65
	void cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, Result_void* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:75
	void cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:83
	void cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, Result_void* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:89
	void cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/rgbd/dynafu.hpp:95
	void cv_dynafu_DynaFu_reset(cv::dynafu::DynaFu* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPose() /usr/include/opencv2/rgbd/dynafu.hpp:98
	void cv_dynafu_DynaFu_getPose_const(const cv::dynafu::DynaFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3f>))
	}
	
	// update(cv::InputArray) /usr/include/opencv2/rgbd/dynafu.hpp:108
	void cv_dynafu_DynaFu_update_const__InputArrayR(cv::dynafu::DynaFu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getNodesPos() /usr/include/opencv2/rgbd/dynafu.hpp:110
	void cv_dynafu_DynaFu_getNodesPos_const(const cv::dynafu::DynaFu* instance, Result<std::vector<cv::Point3f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point3f> ret = instance->getNodesPos();
			Ok(new std::vector<cv::Point3f>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point3f>*>))
	}
	
	// marchCubes(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/dynafu.hpp:112
	void cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* vertices, const cv::_OutputArray* edges, Result_void* ocvrs_return) {
		try {
			instance->marchCubes(*vertices, *edges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// renderSurface(cv::OutputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/rgbd/dynafu.hpp:114
	void cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(cv::dynafu::DynaFu* instance, const cv::_OutputArray* depthImage, const cv::_OutputArray* vertImage, const cv::_OutputArray* normImage, bool warp, Result_void* ocvrs_return) {
		try {
			instance->renderSurface(*depthImage, *vertImage, *normImage, warp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Intr() /usr/include/opencv2/rgbd/intrinsics.hpp:61
	void cv_kinfu_Intr_Intr(Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr>))
	}
	
	// Intr(float, float, float, float) /usr/include/opencv2/rgbd/intrinsics.hpp:62
	void cv_kinfu_Intr_Intr_float_float_float_float(float _fx, float _fy, float _cx, float _cy, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret(_fx, _fy, _cx, _cy);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr>))
	}
	
	// Intr(cv::Matx33f) /usr/include/opencv2/rgbd/intrinsics.hpp:63
	void cv_kinfu_Intr_Intr_Matx33f(cv::Matx33f* m, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr>))
	}
	
	// scale(int) /usr/include/opencv2/rgbd/intrinsics.hpp:65
	void cv_kinfu_Intr_scale_const_int(const cv::kinfu::Intr instance, int pyr, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret = instance.scale(pyr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr>))
	}
	
	// makeReprojector() /usr/include/opencv2/rgbd/intrinsics.hpp:70
	void cv_kinfu_Intr_makeReprojector_const(const cv::kinfu::Intr instance, Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret = instance.makeReprojector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr::Reprojector>))
	}
	
	// makeProjector() /usr/include/opencv2/rgbd/intrinsics.hpp:71
	void cv_kinfu_Intr_makeProjector_const(const cv::kinfu::Intr instance, Result<cv::kinfu::Intr::Projector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Projector ret = instance.makeProjector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr::Projector>))
	}
	
	// getMat() /usr/include/opencv2/rgbd/intrinsics.hpp:73
	void cv_kinfu_Intr_getMat_const(const cv::kinfu::Intr instance, Result<cv::Matx33f>* ocvrs_return) {
		try {
			cv::Matx33f ret = instance.getMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx33f>))
	}
	
	// Projector(cv::kinfu::Intr) /usr/include/opencv2/rgbd/intrinsics.hpp:41
	void cv_kinfu_Intr_Projector_Projector_Intr(cv::kinfu::Intr* intr, Result<cv::kinfu::Intr::Projector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Projector ret(*intr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr::Projector>))
	}
	
	// Reprojector() /usr/include/opencv2/rgbd/intrinsics.hpp:21
	void cv_kinfu_Intr_Reprojector_Reprojector(Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr::Reprojector>))
	}
	
	// Reprojector(cv::kinfu::Intr) /usr/include/opencv2/rgbd/intrinsics.hpp:22
	void cv_kinfu_Intr_Reprojector_Reprojector_Intr(cv::kinfu::Intr* intr, Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret(*intr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Intr::Reprojector>))
	}
	
	// create(const Ptr<cv::kinfu::Params> &) /usr/include/opencv2/rgbd/kinfu.hpp:195
	void cv_kinfu_KinFu_create_const_Ptr_Params_R(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::kinfu::KinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::KinFu> ret = cv::kinfu::KinFu::create(*_params);
			Ok(new cv::Ptr<cv::kinfu::KinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::KinFu>*>))
	}
	
	// getParams() /usr/include/opencv2/rgbd/kinfu.hpp:199
	void cv_kinfu_KinFu_getParams_const(const cv::kinfu::KinFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Params*>))
	}
	
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:209
	void cv_kinfu_KinFu_render_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, Result_void* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/kinfu.hpp:221
	void cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, Result_void* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:231
	void cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:239
	void cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, Result_void* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/kinfu.hpp:245
	void cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/rgbd/kinfu.hpp:251
	void cv_kinfu_KinFu_reset(cv::kinfu::KinFu* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPose() /usr/include/opencv2/rgbd/kinfu.hpp:254
	void cv_kinfu_KinFu_getPose_const(const cv::kinfu::KinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3f>))
	}
	
	// update(cv::InputArray) /usr/include/opencv2/rgbd/kinfu.hpp:264
	void cv_kinfu_KinFu_update_const__InputArrayR(cv::kinfu::KinFu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// frameSize /usr/include/opencv2/rgbd/kinfu.hpp:83
	void cv_kinfu_Params_getPropFrameSize_const(const cv::kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}
	
	// frameSize /usr/include/opencv2/rgbd/kinfu.hpp:83
	void cv_kinfu_Params_setPropFrameSize_Size(cv::kinfu::Params* instance, cv::Size* val) {
			instance->frameSize = *val;
	}
	
	// volumeType /usr/include/opencv2/rgbd/kinfu.hpp:86
	void cv_kinfu_Params_getPropVolumeType_const(const cv::kinfu::Params* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->volumeType;
			*ocvrs_return = ret;
	}
	
	// volumeType /usr/include/opencv2/rgbd/kinfu.hpp:86
	void cv_kinfu_Params_setPropVolumeType_VolumeType(cv::kinfu::Params* instance, cv::kinfu::VolumeType val) {
			instance->volumeType = val;
	}
	
	// intr /usr/include/opencv2/rgbd/kinfu.hpp:89
	void cv_kinfu_Params_getPropIntr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}
	
	// intr /usr/include/opencv2/rgbd/kinfu.hpp:89
	void cv_kinfu_Params_setPropIntr_Matx33f(cv::kinfu::Params* instance, cv::Matx33f* val) {
			instance->intr = *val;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/kinfu.hpp:92
	void cv_kinfu_Params_getPropRgb_intr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/kinfu.hpp:92
	void cv_kinfu_Params_setPropRgb_intr_Matx33f(cv::kinfu::Params* instance, cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/kinfu.hpp:100
	float cv_kinfu_Params_getPropDepthFactor_const(const cv::kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/kinfu.hpp:100
	void cv_kinfu_Params_setPropDepthFactor_float(cv::kinfu::Params* instance, float val) {
			instance->depthFactor = val;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/kinfu.hpp:103
	float cv_kinfu_Params_getPropBilateral_sigma_depth_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/kinfu.hpp:103
	void cv_kinfu_Params_setPropBilateral_sigma_depth_float(cv::kinfu::Params* instance, float val) {
			instance->bilateral_sigma_depth = val;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/kinfu.hpp:105
	float cv_kinfu_Params_getPropBilateral_sigma_spatial_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/kinfu.hpp:105
	void cv_kinfu_Params_setPropBilateral_sigma_spatial_float(cv::kinfu::Params* instance, float val) {
			instance->bilateral_sigma_spatial = val;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/kinfu.hpp:107
	int cv_kinfu_Params_getPropBilateral_kernel_size_const(const cv::kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/kinfu.hpp:107
	void cv_kinfu_Params_setPropBilateral_kernel_size_int(cv::kinfu::Params* instance, int val) {
			instance->bilateral_kernel_size = val;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/kinfu.hpp:110
	int cv_kinfu_Params_getPropPyramidLevels_const(const cv::kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/kinfu.hpp:110
	void cv_kinfu_Params_setPropPyramidLevels_int(cv::kinfu::Params* instance, int val) {
			instance->pyramidLevels = val;
	}
	
	// volumeDims /usr/include/opencv2/rgbd/kinfu.hpp:116
	void cv_kinfu_Params_getPropVolumeDims_const(const cv::kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}
	
	// volumeDims /usr/include/opencv2/rgbd/kinfu.hpp:116
	void cv_kinfu_Params_setPropVolumeDims_Vec3i(cv::kinfu::Params* instance, cv::Vec3i* val) {
			instance->volumeDims = *val;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/kinfu.hpp:118
	float cv_kinfu_Params_getPropVoxelSize_const(const cv::kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/kinfu.hpp:118
	void cv_kinfu_Params_setPropVoxelSize_float(cv::kinfu::Params* instance, float val) {
			instance->voxelSize = val;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/kinfu.hpp:124
	float cv_kinfu_Params_getPropTsdf_min_camera_movement_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/kinfu.hpp:124
	void cv_kinfu_Params_setPropTsdf_min_camera_movement_float(cv::kinfu::Params* instance, float val) {
			instance->tsdf_min_camera_movement = val;
	}
	
	// volumePose /usr/include/opencv2/rgbd/kinfu.hpp:127
	void cv_kinfu_Params_getPropVolumePose_const(const cv::kinfu::Params* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->volumePose;
			*ocvrs_return = ret;
	}
	
	// volumePose /usr/include/opencv2/rgbd/kinfu.hpp:127
	void cv_kinfu_Params_setPropVolumePose_Affine3f(cv::kinfu::Params* instance, cv::Affine3f* val) {
			instance->volumePose = *val;
	}
	
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/kinfu.hpp:133
	float cv_kinfu_Params_getPropTsdf_trunc_dist_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}
	
	// tsdf_trunc_dist /usr/include/opencv2/rgbd/kinfu.hpp:133
	void cv_kinfu_Params_setPropTsdf_trunc_dist_float(cv::kinfu::Params* instance, float val) {
			instance->tsdf_trunc_dist = val;
	}
	
	// tsdf_max_weight /usr/include/opencv2/rgbd/kinfu.hpp:139
	int cv_kinfu_Params_getPropTsdf_max_weight_const(const cv::kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}
	
	// tsdf_max_weight /usr/include/opencv2/rgbd/kinfu.hpp:139
	void cv_kinfu_Params_setPropTsdf_max_weight_int(cv::kinfu::Params* instance, int val) {
			instance->tsdf_max_weight = val;
	}
	
	// raycast_step_factor /usr/include/opencv2/rgbd/kinfu.hpp:145
	float cv_kinfu_Params_getPropRaycast_step_factor_const(const cv::kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}
	
	// raycast_step_factor /usr/include/opencv2/rgbd/kinfu.hpp:145
	void cv_kinfu_Params_setPropRaycast_step_factor_float(cv::kinfu::Params* instance, float val) {
			instance->raycast_step_factor = val;
	}
	
	// lightPose /usr/include/opencv2/rgbd/kinfu.hpp:152
	void cv_kinfu_Params_getPropLightPose_const(const cv::kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}
	
	// lightPose /usr/include/opencv2/rgbd/kinfu.hpp:152
	void cv_kinfu_Params_setPropLightPose_Vec3f(cv::kinfu::Params* instance, cv::Vec3f* val) {
			instance->lightPose = *val;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/kinfu.hpp:155
	float cv_kinfu_Params_getPropIcpDistThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/kinfu.hpp:155
	void cv_kinfu_Params_setPropIcpDistThresh_float(cv::kinfu::Params* instance, float val) {
			instance->icpDistThresh = val;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/kinfu.hpp:157
	float cv_kinfu_Params_getPropIcpAngleThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/kinfu.hpp:157
	void cv_kinfu_Params_setPropIcpAngleThresh_float(cv::kinfu::Params* instance, float val) {
			instance->icpAngleThresh = val;
	}
	
	// icpIterations /usr/include/opencv2/rgbd/kinfu.hpp:159
	std::vector<int>* cv_kinfu_Params_getPropIcpIterations_const(const cv::kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}
	
	// icpIterations /usr/include/opencv2/rgbd/kinfu.hpp:159
	void cv_kinfu_Params_setPropIcpIterations_vector_int_(cv::kinfu::Params* instance, std::vector<int>* val) {
			instance->icpIterations = *val;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/kinfu.hpp:165
	float cv_kinfu_Params_getPropTruncateThreshold_const(const cv::kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/kinfu.hpp:165
	void cv_kinfu_Params_setPropTruncateThreshold_float(cv::kinfu::Params* instance, float val) {
			instance->truncateThreshold = val;
	}
	
	void cv_Kinfu_Params_delete(cv::kinfu::Params* instance) {
		delete instance;
	}
	// Params() /usr/include/opencv2/rgbd/kinfu.hpp:22
	void cv_kinfu_Params_Params(Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Params*>))
	}
	
	// Params(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/kinfu.hpp:30
	void cv_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Params*>))
	}
	
	// Params(cv::Matx44f) /usr/include/opencv2/rgbd/kinfu.hpp:40
	void cv_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::kinfu::Params*>))
	}
	
	// setInitialVolumePose(cv::Matx33f, cv::Vec3f) /usr/include/opencv2/rgbd/kinfu.hpp:51
	void cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, Result_void* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInitialVolumePose(cv::Matx44f) /usr/include/opencv2/rgbd/kinfu.hpp:58
	void cv_kinfu_Params_setInitialVolumePose_Matx44f(cv::kinfu::Params* instance, cv::Matx44f* homogen_tf, Result_void* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// defaultParams() /usr/include/opencv2/rgbd/kinfu.hpp:64
	void cv_kinfu_Params_defaultParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::Params>*>))
	}
	
	// coarseParams() /usr/include/opencv2/rgbd/kinfu.hpp:70
	void cv_kinfu_Params_coarseParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::Params>*>))
	}
	
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/kinfu.hpp:75
	void cv_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::Params>*>))
	}
	
	// coloredTSDFParams(bool) /usr/include/opencv2/rgbd/kinfu.hpp:80
	void cv_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::Params>*>))
	}
	
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:49
	const float cv_kinfu_Volume_getPropVoxelSize_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->voxelSize;
			return ret;
	}
	
	// voxelSizeInv /usr/include/opencv2/rgbd/volume.hpp:50
	const float cv_kinfu_Volume_getPropVoxelSizeInv_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->voxelSizeInv;
			return ret;
	}
	
	// pose /usr/include/opencv2/rgbd/volume.hpp:51
	void cv_kinfu_Volume_getPropPose_const(const cv::kinfu::Volume* instance, cv::Affine3f* ocvrs_return) {
			const cv::Affine3f ret = instance->pose;
			*ocvrs_return = ret;
	}
	
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:52
	const float cv_kinfu_Volume_getPropRaycastStepFactor_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->raycastStepFactor;
			return ret;
	}
	
	// integrate(cv::InputArray, float, const cv::Matx44f &, const kinfu::Intr &, const int) /usr/include/opencv2/rgbd/volume.hpp:31
	void cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const int frameId, Result_void* ocvrs_return) {
		try {
			instance->integrate(*_depth, depthFactor, *cameraPose, *intrinsics, frameId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// integrate(cv::InputArray, cv::InputArray, float, const cv::Matx44f &, const kinfu::Intr &, const cv::kinfu::Intr &, const int) /usr/include/opencv2/rgbd/volume.hpp:33
	void cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, const cv::_InputArray* _rgb, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::kinfu::Intr* rgb_intrinsics, const int frameId, Result_void* ocvrs_return) {
		try {
			instance->integrate(*_depth, *_rgb, depthFactor, *cameraPose, *intrinsics, *rgb_intrinsics, frameId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// raycast(const cv::Matx44f &, const kinfu::Intr &, const cv::Size &, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:36
	void cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::Size* frameSize, const cv::_OutputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *intrinsics, *frameSize, *points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// raycast(const cv::Matx44f &, const kinfu::Intr &, const cv::Size &, cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:38
	void cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::Size* frameSize, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, Result_void* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *intrinsics, *frameSize, *points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fetchNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:40
	void cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_InputArray* points, const cv::_OutputArray* _normals, Result_void* ocvrs_return) {
		try {
			instance->fetchNormals(*points, *_normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fetchPointsNormals(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:41
	void cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->fetchPointsNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fetchPointsNormalsColors(cv::OutputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/volume.hpp:42
	void cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_OutputArray* unnamed, const cv::_OutputArray* unnamed_1, const cv::_OutputArray* unnamed_2, Result_void* ocvrs_return) {
		try {
			instance->fetchPointsNormalsColors(*unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/rgbd/volume.hpp:46
	void cv_kinfu_Volume_reset(cv::kinfu::Volume* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// type /usr/include/opencv2/rgbd/volume.hpp:67
	void cv_kinfu_VolumeParams_getPropType_const(const cv::kinfu::VolumeParams* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->type;
			*ocvrs_return = ret;
	}
	
	// type /usr/include/opencv2/rgbd/volume.hpp:67
	void cv_kinfu_VolumeParams_setPropType_VolumeType(cv::kinfu::VolumeParams* instance, cv::kinfu::VolumeType val) {
			instance->type = val;
	}
	
	// resolution /usr/include/opencv2/rgbd/volume.hpp:74
	void cv_kinfu_VolumeParams_getPropResolution_const(const cv::kinfu::VolumeParams* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->resolution;
			*ocvrs_return = ret;
	}
	
	// resolution /usr/include/opencv2/rgbd/volume.hpp:74
	void cv_kinfu_VolumeParams_setPropResolution_Vec3i(cv::kinfu::VolumeParams* instance, cv::Vec3i* val) {
			instance->resolution = *val;
	}
	
	// unitResolution /usr/include/opencv2/rgbd/volume.hpp:80
	int cv_kinfu_VolumeParams_getPropUnitResolution_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->unitResolution;
			return ret;
	}
	
	// unitResolution /usr/include/opencv2/rgbd/volume.hpp:80
	void cv_kinfu_VolumeParams_setPropUnitResolution_int(cv::kinfu::VolumeParams* instance, int val) {
			instance->unitResolution = val;
	}
	
	// pose /usr/include/opencv2/rgbd/volume.hpp:83
	void cv_kinfu_VolumeParams_getPropPose_const(const cv::kinfu::VolumeParams* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->pose;
			*ocvrs_return = ret;
	}
	
	// pose /usr/include/opencv2/rgbd/volume.hpp:83
	void cv_kinfu_VolumeParams_setPropPose_Affine3f(cv::kinfu::VolumeParams* instance, cv::Affine3f* val) {
			instance->pose = *val;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:86
	float cv_kinfu_VolumeParams_getPropVoxelSize_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->voxelSize;
			return ret;
	}
	
	// voxelSize /usr/include/opencv2/rgbd/volume.hpp:86
	void cv_kinfu_VolumeParams_setPropVoxelSize_float(cv::kinfu::VolumeParams* instance, float val) {
			instance->voxelSize = val;
	}
	
	// tsdfTruncDist /usr/include/opencv2/rgbd/volume.hpp:91
	float cv_kinfu_VolumeParams_getPropTsdfTruncDist_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->tsdfTruncDist;
			return ret;
	}
	
	// tsdfTruncDist /usr/include/opencv2/rgbd/volume.hpp:91
	void cv_kinfu_VolumeParams_setPropTsdfTruncDist_float(cv::kinfu::VolumeParams* instance, float val) {
			instance->tsdfTruncDist = val;
	}
	
	// maxWeight /usr/include/opencv2/rgbd/volume.hpp:97
	int cv_kinfu_VolumeParams_getPropMaxWeight_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->maxWeight;
			return ret;
	}
	
	// maxWeight /usr/include/opencv2/rgbd/volume.hpp:97
	void cv_kinfu_VolumeParams_setPropMaxWeight_int(cv::kinfu::VolumeParams* instance, int val) {
			instance->maxWeight = val;
	}
	
	// depthTruncThreshold /usr/include/opencv2/rgbd/volume.hpp:102
	float cv_kinfu_VolumeParams_getPropDepthTruncThreshold_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->depthTruncThreshold;
			return ret;
	}
	
	// depthTruncThreshold /usr/include/opencv2/rgbd/volume.hpp:102
	void cv_kinfu_VolumeParams_setPropDepthTruncThreshold_float(cv::kinfu::VolumeParams* instance, float val) {
			instance->depthTruncThreshold = val;
	}
	
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:107
	float cv_kinfu_VolumeParams_getPropRaycastStepFactor_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->raycastStepFactor;
			return ret;
	}
	
	// raycastStepFactor /usr/include/opencv2/rgbd/volume.hpp:107
	void cv_kinfu_VolumeParams_setPropRaycastStepFactor_float(cv::kinfu::VolumeParams* instance, float val) {
			instance->raycastStepFactor = val;
	}
	
	void cv_Kinfu_VolumeParams_delete(cv::kinfu::VolumeParams* instance) {
		delete instance;
	}
	// defaultParams(cv::kinfu::VolumeType) /usr/include/opencv2/rgbd/volume.hpp:112
	void cv_kinfu_VolumeParams_defaultParams_VolumeType(cv::kinfu::VolumeType _volumeType, Result<cv::Ptr<cv::kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::VolumeParams> ret = cv::kinfu::VolumeParams::defaultParams(_volumeType);
			Ok(new cv::Ptr<cv::kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::VolumeParams>*>))
	}
	
	// coarseParams(cv::kinfu::VolumeType) /usr/include/opencv2/rgbd/volume.hpp:117
	void cv_kinfu_VolumeParams_coarseParams_VolumeType(cv::kinfu::VolumeType _volumeType, Result<cv::Ptr<cv::kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::VolumeParams> ret = cv::kinfu::VolumeParams::coarseParams(_volumeType);
			Ok(new cv::Ptr<cv::kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::VolumeParams>*>))
	}
	
	// create() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:29
	void cv_kinfu_detail_PoseGraph_create(Result<cv::Ptr<cv::kinfu::detail::PoseGraph>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::detail::PoseGraph> ret = cv::kinfu::detail::PoseGraph::create();
			Ok(new cv::Ptr<cv::kinfu::detail::PoseGraph>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::kinfu::detail::PoseGraph>*>))
	}
	
	// addNode(size_t, const cv::Affine3d &, bool) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:33
	void cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(cv::kinfu::detail::PoseGraph* instance, size_t _nodeId, const cv::Affine3d* _pose, bool fixed, Result_void* ocvrs_return) {
		try {
			instance->addNode(_nodeId, *_pose, fixed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isNodeExist(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:34
	void cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNodeExist(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setNodeFixed(size_t, bool) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:35
	void cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(cv::kinfu::detail::PoseGraph* instance, size_t nodeId, bool fixed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setNodeFixed(nodeId, fixed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isNodeFixed(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:36
	void cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNodeFixed(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getNodePose(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:37
	void cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getNodePose(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3d>))
	}
	
	// getNodesIds() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:38
	void cv_kinfu_detail_PoseGraph_getNodesIds_const(const cv::kinfu::detail::PoseGraph* instance, Result<std::vector<size_t>*>* ocvrs_return) {
		try {
			std::vector<size_t> ret = instance->getNodesIds();
			Ok(new std::vector<size_t>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<size_t>*>))
	}
	
	// getNumNodes() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:39
	void cv_kinfu_detail_PoseGraph_getNumNodes_const(const cv::kinfu::detail::PoseGraph* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getNumNodes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// addEdge(size_t, size_t, const cv::Affine3f &, const cv::Matx66f &) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:42
	void cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(cv::kinfu::detail::PoseGraph* instance, size_t _sourceNodeId, size_t _targetNodeId, const cv::Affine3f* _transformation, const cv::Matx66f* _information, Result_void* ocvrs_return) {
		try {
			instance->addEdge(_sourceNodeId, _targetNodeId, *_transformation, *_information);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEdgeStart(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:44
	void cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getEdgeStart(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// getEdgeEnd(size_t) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:45
	void cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getEdgeEnd(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// getNumEdges() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:46
	void cv_kinfu_detail_PoseGraph_getNumEdges_const(const cv::kinfu::detail::PoseGraph* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getNumEdges();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// isValid() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:49
	void cv_kinfu_detail_PoseGraph_isValid_const(const cv::kinfu::detail::PoseGraph* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isValid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// optimize(const cv::TermCriteria &) /usr/include/opencv2/rgbd/detail/pose_graph.hpp:53
	void cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(cv::kinfu::detail::PoseGraph* instance, const cv::TermCriteria* tc, Result<int>* ocvrs_return) {
		try {
			int ret = instance->optimize(*tc);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// calcEnergy() /usr/include/opencv2/rgbd/detail/pose_graph.hpp:56
	void cv_kinfu_detail_PoseGraph_calcEnergy_const(const cv::kinfu::detail::PoseGraph* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calcEnergy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// create(const Ptr<cv::large_kinfu::Params> &) /usr/include/opencv2/rgbd/large_kinfu.hpp:123
	void cv_large_kinfu_LargeKinfu_create_const_Ptr_Params_R(const cv::Ptr<cv::large_kinfu::Params>* _params, Result<cv::Ptr<cv::large_kinfu::LargeKinfu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::LargeKinfu> ret = cv::large_kinfu::LargeKinfu::create(*_params);
			Ok(new cv::Ptr<cv::large_kinfu::LargeKinfu>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::large_kinfu::LargeKinfu>*>))
	}
	
	// getParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:126
	void cv_large_kinfu_LargeKinfu_getParams_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::large_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::large_kinfu::Params ret = instance->getParams();
			Ok(new const cv::large_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::large_kinfu::Params*>))
	}
	
	// render(cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:128
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, Result_void* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// render(cv::OutputArray, const cv::Matx44f &) /usr/include/opencv2/rgbd/large_kinfu.hpp:129
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, Result_void* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCloud(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:131
	void cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPoints(cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:133
	void cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, Result_void* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormals(cv::InputArray, cv::OutputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:135
	void cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, Result_void* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reset() /usr/include/opencv2/rgbd/large_kinfu.hpp:137
	void cv_large_kinfu_LargeKinfu_reset(cv::large_kinfu::LargeKinfu* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPose() /usr/include/opencv2/rgbd/large_kinfu.hpp:139
	void cv_large_kinfu_LargeKinfu_getPose_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Affine3f>))
	}
	
	// update(cv::InputArray) /usr/include/opencv2/rgbd/large_kinfu.hpp:141
	void cv_large_kinfu_LargeKinfu_update_const__InputArrayR(cv::large_kinfu::LargeKinfu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// frameSize /usr/include/opencv2/rgbd/large_kinfu.hpp:39
	void cv_large_kinfu_Params_getPropFrameSize_const(const cv::large_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}
	
	// frameSize /usr/include/opencv2/rgbd/large_kinfu.hpp:39
	void cv_large_kinfu_Params_setPropFrameSize_Size(cv::large_kinfu::Params* instance, cv::Size* val) {
			instance->frameSize = *val;
	}
	
	// intr /usr/include/opencv2/rgbd/large_kinfu.hpp:42
	void cv_large_kinfu_Params_getPropIntr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}
	
	// intr /usr/include/opencv2/rgbd/large_kinfu.hpp:42
	void cv_large_kinfu_Params_setPropIntr_Matx33f(cv::large_kinfu::Params* instance, cv::Matx33f* val) {
			instance->intr = *val;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/large_kinfu.hpp:45
	void cv_large_kinfu_Params_getPropRgb_intr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}
	
	// rgb_intr /usr/include/opencv2/rgbd/large_kinfu.hpp:45
	void cv_large_kinfu_Params_setPropRgb_intr_Matx33f(cv::large_kinfu::Params* instance, cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/large_kinfu.hpp:53
	float cv_large_kinfu_Params_getPropDepthFactor_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}
	
	// depthFactor /usr/include/opencv2/rgbd/large_kinfu.hpp:53
	void cv_large_kinfu_Params_setPropDepthFactor_float(cv::large_kinfu::Params* instance, float val) {
			instance->depthFactor = val;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/large_kinfu.hpp:56
	float cv_large_kinfu_Params_getPropBilateral_sigma_depth_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}
	
	// bilateral_sigma_depth /usr/include/opencv2/rgbd/large_kinfu.hpp:56
	void cv_large_kinfu_Params_setPropBilateral_sigma_depth_float(cv::large_kinfu::Params* instance, float val) {
			instance->bilateral_sigma_depth = val;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/large_kinfu.hpp:58
	float cv_large_kinfu_Params_getPropBilateral_sigma_spatial_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}
	
	// bilateral_sigma_spatial /usr/include/opencv2/rgbd/large_kinfu.hpp:58
	void cv_large_kinfu_Params_setPropBilateral_sigma_spatial_float(cv::large_kinfu::Params* instance, float val) {
			instance->bilateral_sigma_spatial = val;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/large_kinfu.hpp:60
	int cv_large_kinfu_Params_getPropBilateral_kernel_size_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}
	
	// bilateral_kernel_size /usr/include/opencv2/rgbd/large_kinfu.hpp:60
	void cv_large_kinfu_Params_setPropBilateral_kernel_size_int(cv::large_kinfu::Params* instance, int val) {
			instance->bilateral_kernel_size = val;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/large_kinfu.hpp:63
	int cv_large_kinfu_Params_getPropPyramidLevels_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}
	
	// pyramidLevels /usr/include/opencv2/rgbd/large_kinfu.hpp:63
	void cv_large_kinfu_Params_setPropPyramidLevels_int(cv::large_kinfu::Params* instance, int val) {
			instance->pyramidLevels = val;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/large_kinfu.hpp:68
	float cv_large_kinfu_Params_getPropTsdf_min_camera_movement_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}
	
	// tsdf_min_camera_movement /usr/include/opencv2/rgbd/large_kinfu.hpp:68
	void cv_large_kinfu_Params_setPropTsdf_min_camera_movement_float(cv::large_kinfu::Params* instance, float val) {
			instance->tsdf_min_camera_movement = val;
	}
	
	// lightPose /usr/include/opencv2/rgbd/large_kinfu.hpp:71
	void cv_large_kinfu_Params_getPropLightPose_const(const cv::large_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}
	
	// lightPose /usr/include/opencv2/rgbd/large_kinfu.hpp:71
	void cv_large_kinfu_Params_setPropLightPose_Vec3f(cv::large_kinfu::Params* instance, cv::Vec3f* val) {
			instance->lightPose = *val;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:74
	float cv_large_kinfu_Params_getPropIcpDistThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}
	
	// icpDistThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:74
	void cv_large_kinfu_Params_setPropIcpDistThresh_float(cv::large_kinfu::Params* instance, float val) {
			instance->icpDistThresh = val;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:76
	float cv_large_kinfu_Params_getPropIcpAngleThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}
	
	// icpAngleThresh /usr/include/opencv2/rgbd/large_kinfu.hpp:76
	void cv_large_kinfu_Params_setPropIcpAngleThresh_float(cv::large_kinfu::Params* instance, float val) {
			instance->icpAngleThresh = val;
	}
	
	// icpIterations /usr/include/opencv2/rgbd/large_kinfu.hpp:78
	std::vector<int>* cv_large_kinfu_Params_getPropIcpIterations_const(const cv::large_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}
	
	// icpIterations /usr/include/opencv2/rgbd/large_kinfu.hpp:78
	void cv_large_kinfu_Params_setPropIcpIterations_vector_int_(cv::large_kinfu::Params* instance, std::vector<int>* val) {
			instance->icpIterations = *val;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/large_kinfu.hpp:83
	float cv_large_kinfu_Params_getPropTruncateThreshold_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}
	
	// truncateThreshold /usr/include/opencv2/rgbd/large_kinfu.hpp:83
	void cv_large_kinfu_Params_setPropTruncateThreshold_float(cv::large_kinfu::Params* instance, float val) {
			instance->truncateThreshold = val;
	}
	
	// volumeParams /usr/include/opencv2/rgbd/large_kinfu.hpp:87
	cv::kinfu::VolumeParams* cv_large_kinfu_Params_getPropVolumeParams_const(const cv::large_kinfu::Params* instance) {
			cv::kinfu::VolumeParams ret = instance->volumeParams;
			return new cv::kinfu::VolumeParams(ret);
	}
	
	// volumeParams /usr/include/opencv2/rgbd/large_kinfu.hpp:87
	void cv_large_kinfu_Params_setPropVolumeParams_VolumeParams(cv::large_kinfu::Params* instance, cv::kinfu::VolumeParams* val) {
			instance->volumeParams = *val;
	}
	
	void cv_Params_delete(cv::large_kinfu::Params* instance) {
		delete instance;
	}
	// defaultParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:25
	void cv_large_kinfu_Params_defaultParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::large_kinfu::Params>*>))
	}
	
	// coarseParams() /usr/include/opencv2/rgbd/large_kinfu.hpp:31
	void cv_large_kinfu_Params_coarseParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::large_kinfu::Params>*>))
	}
	
	// hashTSDFParams(bool) /usr/include/opencv2/rgbd/large_kinfu.hpp:36
	void cv_large_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::large_kinfu::Params>*>))
	}
	
	// weak_threshold /usr/include/opencv2/rgbd/linemod.hpp:191
	float cv_linemod_ColorGradient_getPropWeak_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->weak_threshold;
			return ret;
	}
	
	// weak_threshold /usr/include/opencv2/rgbd/linemod.hpp:191
	void cv_linemod_ColorGradient_setPropWeak_threshold_float(cv::linemod::ColorGradient* instance, float val) {
			instance->weak_threshold = val;
	}
	
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:192
	size_t cv_linemod_ColorGradient_getPropNum_features_const(const cv::linemod::ColorGradient* instance) {
			size_t ret = instance->num_features;
			return ret;
	}
	
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:192
	void cv_linemod_ColorGradient_setPropNum_features_size_t(cv::linemod::ColorGradient* instance, size_t val) {
			instance->num_features = val;
	}
	
	// strong_threshold /usr/include/opencv2/rgbd/linemod.hpp:193
	float cv_linemod_ColorGradient_getPropStrong_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->strong_threshold;
			return ret;
	}
	
	// strong_threshold /usr/include/opencv2/rgbd/linemod.hpp:193
	void cv_linemod_ColorGradient_setPropStrong_threshold_float(cv::linemod::ColorGradient* instance, float val) {
			instance->strong_threshold = val;
	}
	
	void cv_Linemod_ColorGradient_delete(cv::linemod::ColorGradient* instance) {
		delete instance;
	}
	// ColorGradient() /usr/include/opencv2/rgbd/linemod.hpp:172
	void cv_linemod_ColorGradient_ColorGradient(Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::ColorGradient*>))
	}
	
	// ColorGradient(float, size_t, float) /usr/include/opencv2/rgbd/linemod.hpp:182
	void cv_linemod_ColorGradient_ColorGradient_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient(weak_threshold, num_features, strong_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::ColorGradient*>))
	}
	
	// create(float, size_t, float) /usr/include/opencv2/rgbd/linemod.hpp:184
	void cv_linemod_ColorGradient_create_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::Ptr<cv::linemod::ColorGradient>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::ColorGradient> ret = cv::linemod::ColorGradient::create(weak_threshold, num_features, strong_threshold);
			Ok(new cv::Ptr<cv::linemod::ColorGradient>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::ColorGradient>*>))
	}
	
	// name() /usr/include/opencv2/rgbd/linemod.hpp:186
	void cv_linemod_ColorGradient_name_const(const cv::linemod::ColorGradient* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:188
	void cv_linemod_ColorGradient_read_const_FileNodeR(cv::linemod::ColorGradient* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:189
	void cv_linemod_ColorGradient_write_const_FileStorageR(const cv::linemod::ColorGradient* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// distance_threshold /usr/include/opencv2/rgbd/linemod.hpp:232
	int cv_linemod_DepthNormal_getPropDistance_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->distance_threshold;
			return ret;
	}
	
	// distance_threshold /usr/include/opencv2/rgbd/linemod.hpp:232
	void cv_linemod_DepthNormal_setPropDistance_threshold_int(cv::linemod::DepthNormal* instance, int val) {
			instance->distance_threshold = val;
	}
	
	// difference_threshold /usr/include/opencv2/rgbd/linemod.hpp:233
	int cv_linemod_DepthNormal_getPropDifference_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->difference_threshold;
			return ret;
	}
	
	// difference_threshold /usr/include/opencv2/rgbd/linemod.hpp:233
	void cv_linemod_DepthNormal_setPropDifference_threshold_int(cv::linemod::DepthNormal* instance, int val) {
			instance->difference_threshold = val;
	}
	
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:234
	size_t cv_linemod_DepthNormal_getPropNum_features_const(const cv::linemod::DepthNormal* instance) {
			size_t ret = instance->num_features;
			return ret;
	}
	
	// num_features /usr/include/opencv2/rgbd/linemod.hpp:234
	void cv_linemod_DepthNormal_setPropNum_features_size_t(cv::linemod::DepthNormal* instance, size_t val) {
			instance->num_features = val;
	}
	
	// extract_threshold /usr/include/opencv2/rgbd/linemod.hpp:235
	int cv_linemod_DepthNormal_getPropExtract_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->extract_threshold;
			return ret;
	}
	
	// extract_threshold /usr/include/opencv2/rgbd/linemod.hpp:235
	void cv_linemod_DepthNormal_setPropExtract_threshold_int(cv::linemod::DepthNormal* instance, int val) {
			instance->extract_threshold = val;
	}
	
	void cv_Linemod_DepthNormal_delete(cv::linemod::DepthNormal* instance) {
		delete instance;
	}
	// DepthNormal() /usr/include/opencv2/rgbd/linemod.hpp:209
	void cv_linemod_DepthNormal_DepthNormal(Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::DepthNormal*>))
	}
	
	// DepthNormal(int, int, size_t, int) /usr/include/opencv2/rgbd/linemod.hpp:221
	void cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::DepthNormal*>))
	}
	
	// create(int, int, size_t, int) /usr/include/opencv2/rgbd/linemod.hpp:224
	void cv_linemod_DepthNormal_create_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::Ptr<cv::linemod::DepthNormal>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::DepthNormal> ret = cv::linemod::DepthNormal::create(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(new cv::Ptr<cv::linemod::DepthNormal>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::DepthNormal>*>))
	}
	
	// name() /usr/include/opencv2/rgbd/linemod.hpp:227
	void cv_linemod_DepthNormal_name_const(const cv::linemod::DepthNormal* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:229
	void cv_linemod_DepthNormal_read_const_FileNodeR(cv::linemod::DepthNormal* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:230
	void cv_linemod_DepthNormal_write_const_FileStorageR(const cv::linemod::DepthNormal* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Linemod_Detector_delete(cv::linemod::Detector* instance) {
		delete instance;
	}
	// Detector() /usr/include/opencv2/rgbd/linemod.hpp:304
	void cv_linemod_Detector_Detector(Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Detector*>))
	}
	
	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &) /usr/include/opencv2/rgbd/linemod.hpp:313
	void cv_linemod_Detector_Detector_const_vector_Ptr_Modality__R_const_vector_int_R(const std::vector<cv::Ptr<cv::linemod::Modality>>* modalities, const std::vector<int>* T_pyramid, Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector(*modalities, *T_pyramid);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Detector*>))
	}
	
	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, cv::OutputArrayOfArrays, const std::vector<Mat> &) /usr/include/opencv2/rgbd/linemod.hpp:330
	void cv_linemod_Detector_match_const_const_vector_Mat_R_float_vector_Match_R_const_vector_String_R_const__OutputArrayR_const_vector_Mat_R(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, const std::vector<cv::String>* class_ids, const cv::_OutputArray* quantized_images, const std::vector<cv::Mat>* masks, Result_void* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches, *class_ids, *quantized_images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addTemplate(const std::vector<Mat> &, const cv::String &, const cv::Mat &, cv::Rect *) /usr/include/opencv2/rgbd/linemod.hpp:345
	void cv_linemod_Detector_addTemplate_const_vector_Mat_R_const_StringR_const_MatR_RectX(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, cv::Rect* bounding_box, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, std::string(class_id), *object_mask, bounding_box);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// addSyntheticTemplate(const std::vector<Template> &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:351
	void cv_linemod_Detector_addSyntheticTemplate_const_vector_Template_R_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::linemod::Template>* templates, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addSyntheticTemplate(*templates, std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getModalities() /usr/include/opencv2/rgbd/linemod.hpp:359
	void cv_linemod_Detector_getModalities_const(const cv::linemod::Detector* instance, Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>* ocvrs_return) {
		try {
			const std::vector<cv::Ptr<cv::linemod::Modality>> ret = instance->getModalities();
			Ok(new const std::vector<cv::Ptr<cv::linemod::Modality>>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>))
	}
	
	// getT(int) /usr/include/opencv2/rgbd/linemod.hpp:364
	void cv_linemod_Detector_getT_const_int(const cv::linemod::Detector* instance, int pyramid_level, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getT(pyramid_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// pyramidLevels() /usr/include/opencv2/rgbd/linemod.hpp:369
	void cv_linemod_Detector_pyramidLevels_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pyramidLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getTemplates(const cv::String &, int) /usr/include/opencv2/rgbd/linemod.hpp:377
	void cv_linemod_Detector_getTemplates_const_const_StringR_int(const cv::linemod::Detector* instance, const char* class_id, int template_id, Result<std::vector<cv::linemod::Template>*>* ocvrs_return) {
		try {
			const std::vector<cv::linemod::Template> ret = instance->getTemplates(std::string(class_id), template_id);
			Ok(new const std::vector<cv::linemod::Template>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::linemod::Template>*>))
	}
	
	// numTemplates() /usr/include/opencv2/rgbd/linemod.hpp:379
	void cv_linemod_Detector_numTemplates_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// numTemplates(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:380
	void cv_linemod_Detector_numTemplates_const_const_StringR(const cv::linemod::Detector* instance, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates(std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// numClasses() /usr/include/opencv2/rgbd/linemod.hpp:381
	void cv_linemod_Detector_numClasses_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numClasses();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// classIds() /usr/include/opencv2/rgbd/linemod.hpp:383
	void cv_linemod_Detector_classIds_const(const cv::linemod::Detector* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->classIds();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:385
	void cv_linemod_Detector_read_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:386
	void cv_linemod_Detector_write_const_FileStorageR(const cv::linemod::Detector* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// readClass(const cv::FileNode &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:388
	void cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(cv::linemod::Detector* instance, const cv::FileNode* fn, const char* class_id_override, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn, std::string(class_id_override));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// writeClass(const cv::String &, cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:389
	void cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(const cv::linemod::Detector* instance, const char* class_id, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->writeClass(std::string(class_id), *fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// readClasses(const std::vector<String> &, const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:391
	void cv_linemod_Detector_readClasses_const_vector_String_R_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, const char* format, Result_void* ocvrs_return) {
		try {
			instance->readClasses(*class_ids, std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeClasses(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:393
	void cv_linemod_Detector_writeClasses_const_const_StringR(const cv::linemod::Detector* instance, const char* format, Result_void* ocvrs_return) {
		try {
			instance->writeClasses(std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Feature() /usr/include/opencv2/rgbd/linemod.hpp:32
	void cv_linemod_Feature_Feature(Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Feature>))
	}
	
	// Feature(int, int, int) /usr/include/opencv2/rgbd/linemod.hpp:33
	void cv_linemod_Feature_Feature_int_int_int(int x, int y, int label, Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret(x, y, label);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Feature>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:35
	void cv_linemod_Feature_read_const_FileNodeR(cv::linemod::Feature instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance.read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:36
	void cv_linemod_Feature_write_const_FileStorageR(const cv::linemod::Feature instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance.write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// x /usr/include/opencv2/rgbd/linemod.hpp:282
	int cv_linemod_Match_getPropX_const(const cv::linemod::Match* instance) {
			int ret = instance->x;
			return ret;
	}
	
	// x /usr/include/opencv2/rgbd/linemod.hpp:282
	void cv_linemod_Match_setPropX_int(cv::linemod::Match* instance, int val) {
			instance->x = val;
	}
	
	// y /usr/include/opencv2/rgbd/linemod.hpp:283
	int cv_linemod_Match_getPropY_const(const cv::linemod::Match* instance) {
			int ret = instance->y;
			return ret;
	}
	
	// y /usr/include/opencv2/rgbd/linemod.hpp:283
	void cv_linemod_Match_setPropY_int(cv::linemod::Match* instance, int val) {
			instance->y = val;
	}
	
	// similarity /usr/include/opencv2/rgbd/linemod.hpp:284
	float cv_linemod_Match_getPropSimilarity_const(const cv::linemod::Match* instance) {
			float ret = instance->similarity;
			return ret;
	}
	
	// similarity /usr/include/opencv2/rgbd/linemod.hpp:284
	void cv_linemod_Match_setPropSimilarity_float(cv::linemod::Match* instance, float val) {
			instance->similarity = val;
	}
	
	// class_id /usr/include/opencv2/rgbd/linemod.hpp:285
	void* cv_linemod_Match_getPropClass_id_const(const cv::linemod::Match* instance) {
			cv::String ret = instance->class_id;
			return ocvrs_create_string(ret.c_str());
	}
	
	// class_id /usr/include/opencv2/rgbd/linemod.hpp:285
	void cv_linemod_Match_setPropClass_id_String(cv::linemod::Match* instance, char* val) {
			instance->class_id = std::string(val);
	}
	
	// template_id /usr/include/opencv2/rgbd/linemod.hpp:286
	int cv_linemod_Match_getPropTemplate_id_const(const cv::linemod::Match* instance) {
			int ret = instance->template_id;
			return ret;
	}
	
	// template_id /usr/include/opencv2/rgbd/linemod.hpp:286
	void cv_linemod_Match_setPropTemplate_id_int(cv::linemod::Match* instance, int val) {
			instance->template_id = val;
	}
	
	void cv_Linemod_Match_delete(cv::linemod::Match* instance) {
		delete instance;
	}
	// Match() /usr/include/opencv2/rgbd/linemod.hpp:261
	void cv_linemod_Match_Match(Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Match*>))
	}
	
	// Match(int, int, float, const cv::String &, int) /usr/include/opencv2/rgbd/linemod.hpp:265
	void cv_linemod_Match_Match_int_int_float_const_StringR_int(int x, int y, float similarity, const char* class_id, int template_id, Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match(x, y, similarity, std::string(class_id), template_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::linemod::Match*>))
	}
	
	// operator==(const cv::linemod::Match &) /usr/include/opencv2/rgbd/linemod.hpp:277
	void cv_linemod_Match_operatorEQ_const_const_MatchR(const cv::linemod::Match* instance, const cv::linemod::Match* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// process(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:132
	void cv_linemod_Modality_process_const_const_MatR_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, const cv::Mat* mask, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src, *mask);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>))
	}
	
	// name() /usr/include/opencv2/rgbd/linemod.hpp:138
	void cv_linemod_Modality_name_const(const cv::linemod::Modality* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:140
	void cv_linemod_Modality_read_const_FileNodeR(cv::linemod::Modality* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:141
	void cv_linemod_Modality_write_const_FileStorageR(const cv::linemod::Modality* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const cv::String &) /usr/include/opencv2/rgbd/linemod.hpp:150
	void cv_linemod_Modality_create_const_StringR(const char* modality_type, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(std::string(modality_type));
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Modality>*>))
	}
	
	// create(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:155
	void cv_linemod_Modality_create_const_FileNodeR(const cv::FileNode* fn, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(*fn);
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::linemod::Modality>*>))
	}
	
	// quantize(cv::Mat &) /usr/include/opencv2/rgbd/linemod.hpp:67
	void cv_linemod_QuantizedPyramid_quantize_const_MatR(const cv::linemod::QuantizedPyramid* instance, cv::Mat* dst, Result_void* ocvrs_return) {
		try {
			instance->quantize(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// extractTemplate(cv::linemod::Template &) /usr/include/opencv2/rgbd/linemod.hpp:74
	void cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(const cv::linemod::QuantizedPyramid* instance, cv::linemod::Template* templ, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->extractTemplate(*templ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// pyrDown() /usr/include/opencv2/rgbd/linemod.hpp:81
	void cv_linemod_QuantizedPyramid_pyrDown(cv::linemod::QuantizedPyramid* instance, Result_void* ocvrs_return) {
		try {
			instance->pyrDown();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// width /usr/include/opencv2/rgbd/linemod.hpp:43
	int cv_linemod_Template_getPropWidth_const(const cv::linemod::Template* instance) {
			int ret = instance->width;
			return ret;
	}
	
	// width /usr/include/opencv2/rgbd/linemod.hpp:43
	void cv_linemod_Template_setPropWidth_int(cv::linemod::Template* instance, int val) {
			instance->width = val;
	}
	
	// height /usr/include/opencv2/rgbd/linemod.hpp:44
	int cv_linemod_Template_getPropHeight_const(const cv::linemod::Template* instance) {
			int ret = instance->height;
			return ret;
	}
	
	// height /usr/include/opencv2/rgbd/linemod.hpp:44
	void cv_linemod_Template_setPropHeight_int(cv::linemod::Template* instance, int val) {
			instance->height = val;
	}
	
	// pyramid_level /usr/include/opencv2/rgbd/linemod.hpp:45
	int cv_linemod_Template_getPropPyramid_level_const(const cv::linemod::Template* instance) {
			int ret = instance->pyramid_level;
			return ret;
	}
	
	// pyramid_level /usr/include/opencv2/rgbd/linemod.hpp:45
	void cv_linemod_Template_setPropPyramid_level_int(cv::linemod::Template* instance, int val) {
			instance->pyramid_level = val;
	}
	
	// features /usr/include/opencv2/rgbd/linemod.hpp:46
	std::vector<cv::linemod::Feature>* cv_linemod_Template_getPropFeatures_const(const cv::linemod::Template* instance) {
			std::vector<cv::linemod::Feature> ret = instance->features;
			return new std::vector<cv::linemod::Feature>(ret);
	}
	
	// features /usr/include/opencv2/rgbd/linemod.hpp:46
	void cv_linemod_Template_setPropFeatures_vector_Feature_(cv::linemod::Template* instance, std::vector<cv::linemod::Feature>* val) {
			instance->features = *val;
	}
	
	void cv_Linemod_Template_delete(cv::linemod::Template* instance) {
		delete instance;
	}
	// read(const cv::FileNode &) /usr/include/opencv2/rgbd/linemod.hpp:48
	void cv_linemod_Template_read_const_FileNodeR(cv::linemod::Template* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/rgbd/linemod.hpp:49
	void cv_linemod_Template_write_const_FileStorageR(const cv::linemod::Template* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_DepthCleaner_to_Algorithm(cv::rgbd::DepthCleaner* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_DepthCleaner_delete(cv::rgbd::DepthCleaner* instance) {
		delete instance;
	}
	// DepthCleaner() /usr/include/opencv2/rgbd/depth.hpp:198
	void cv_rgbd_DepthCleaner_DepthCleaner(Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::DepthCleaner*>))
	}
	
	// DepthCleaner(int, int, int) /usr/include/opencv2/rgbd/depth.hpp:212
	void cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(int depth, int window_size, int method, Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::DepthCleaner*>))
	}
	
	// create(int, int, int) /usr/include/opencv2/rgbd/depth.hpp:216
	void cv_rgbd_DepthCleaner_create_int_int_int(int depth, int window_size, int method, Result<cv::Ptr<cv::rgbd::DepthCleaner>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::DepthCleaner>*>))
	}
	
	// initialize() /usr/include/opencv2/rgbd/depth.hpp:229
	void cv_rgbd_DepthCleaner_initialize_const(const cv::rgbd::DepthCleaner* instance, Result_void* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/rgbd/depth.hpp:231
	void cv_rgbd_DepthCleaner_getWindowSize_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWindowSize(int) /usr/include/opencv2/rgbd/depth.hpp:235
	void cv_rgbd_DepthCleaner_setWindowSize_int(cv::rgbd::DepthCleaner* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDepth() /usr/include/opencv2/rgbd/depth.hpp:239
	void cv_rgbd_DepthCleaner_getDepth_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDepth(int) /usr/include/opencv2/rgbd/depth.hpp:243
	void cv_rgbd_DepthCleaner_setDepth_int(cv::rgbd::DepthCleaner* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:247
	void cv_rgbd_DepthCleaner_getMethod_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:251
	void cv_rgbd_DepthCleaner_setMethod_int(cv::rgbd::DepthCleaner* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_FastICPOdometry_to_Algorithm(cv::rgbd::FastICPOdometry* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_FastICPOdometry_delete(cv::rgbd::FastICPOdometry* instance) {
		delete instance;
	}
	// FastICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:1042
	void cv_rgbd_FastICPOdometry_FastICPOdometry(Result<cv::rgbd::FastICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::FastICPOdometry* ret = new cv::rgbd::FastICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::FastICPOdometry*>))
	}
	
	// FastICPOdometry(const cv::Mat &, float, float, float, float, int, const std::vector<int> &) /usr/include/opencv2/rgbd/depth.hpp:1054
	void cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vector_int_R(const cv::Mat* cameraMatrix, float maxDistDiff, float angleThreshold, float sigmaDepth, float sigmaSpatial, int kernelSize, const std::vector<int>* iterCounts, Result<cv::rgbd::FastICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::FastICPOdometry* ret = new cv::rgbd::FastICPOdometry(*cameraMatrix, maxDistDiff, angleThreshold, sigmaDepth, sigmaSpatial, kernelSize, *iterCounts);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::FastICPOdometry*>))
	}
	
	// create(const cv::Mat &, float, float, float, float, int, const std::vector<int> &) /usr/include/opencv2/rgbd/depth.hpp:1062
	void cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vector_int_R(const cv::Mat* cameraMatrix, float maxDistDiff, float angleThreshold, float sigmaDepth, float sigmaSpatial, int kernelSize, const std::vector<int>* iterCounts, Result<cv::Ptr<cv::rgbd::FastICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::FastICPOdometry> ret = cv::rgbd::FastICPOdometry::create(*cameraMatrix, maxDistDiff, angleThreshold, sigmaDepth, sigmaSpatial, kernelSize, *iterCounts);
			Ok(new cv::Ptr<cv::rgbd::FastICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::FastICPOdometry>*>))
	}
	
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:1070
	void cv_rgbd_FastICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::FastICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:1072
	void cv_rgbd_FastICPOdometry_getCameraMatrix_const(const cv::rgbd::FastICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:1076
	void cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::FastICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDistDiff() /usr/include/opencv2/rgbd/depth.hpp:1080
	void cv_rgbd_FastICPOdometry_getMaxDistDiff_const(const cv::rgbd::FastICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDistDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDistDiff(float) /usr/include/opencv2/rgbd/depth.hpp:1084
	void cv_rgbd_FastICPOdometry_setMaxDistDiff_float(cv::rgbd::FastICPOdometry* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDistDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAngleThreshold() /usr/include/opencv2/rgbd/depth.hpp:1088
	void cv_rgbd_FastICPOdometry_getAngleThreshold_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAngleThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setAngleThreshold(float) /usr/include/opencv2/rgbd/depth.hpp:1092
	void cv_rgbd_FastICPOdometry_setAngleThreshold_float(cv::rgbd::FastICPOdometry* instance, float f, Result_void* ocvrs_return) {
		try {
			instance->setAngleThreshold(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigmaDepth() /usr/include/opencv2/rgbd/depth.hpp:1096
	void cv_rgbd_FastICPOdometry_getSigmaDepth_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSigmaDepth(float) /usr/include/opencv2/rgbd/depth.hpp:1100
	void cv_rgbd_FastICPOdometry_setSigmaDepth_float(cv::rgbd::FastICPOdometry* instance, float f, Result_void* ocvrs_return) {
		try {
			instance->setSigmaDepth(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigmaSpatial() /usr/include/opencv2/rgbd/depth.hpp:1104
	void cv_rgbd_FastICPOdometry_getSigmaSpatial_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaSpatial();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSigmaSpatial(float) /usr/include/opencv2/rgbd/depth.hpp:1108
	void cv_rgbd_FastICPOdometry_setSigmaSpatial_float(cv::rgbd::FastICPOdometry* instance, float f, Result_void* ocvrs_return) {
		try {
			instance->setSigmaSpatial(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getKernelSize() /usr/include/opencv2/rgbd/depth.hpp:1112
	void cv_rgbd_FastICPOdometry_getKernelSize_const(const cv::rgbd::FastICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setKernelSize(int) /usr/include/opencv2/rgbd/depth.hpp:1116
	void cv_rgbd_FastICPOdometry_setKernelSize_int(cv::rgbd::FastICPOdometry* instance, int f, Result_void* ocvrs_return) {
		try {
			instance->setKernelSize(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:1120
	void cv_rgbd_FastICPOdometry_getIterationCounts_const(const cv::rgbd::FastICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:1124
	void cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(cv::rgbd::FastICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:1128
	void cv_rgbd_FastICPOdometry_getTransformType_const(const cv::rgbd::FastICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:1132
	void cv_rgbd_FastICPOdometry_setTransformType_int(cv::rgbd::FastICPOdometry* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_ICPOdometry_to_Algorithm(cv::rgbd::ICPOdometry* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ICPOdometry_delete(cv::rgbd::ICPOdometry* instance) {
		delete instance;
	}
	// ICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:762
	void cv_rgbd_ICPOdometry_ICPOdometry(Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::ICPOdometry*>))
	}
	
	// ICPOdometry(const cv::Mat &, float, float, float, float, const std::vector<int> &, int) /usr/include/opencv2/rgbd/depth.hpp:773
	void cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::ICPOdometry*>))
	}
	
	// create(const cv::Mat &, float, float, float, float, const std::vector<int> &, int) /usr/include/opencv2/rgbd/depth.hpp:777
	void cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::Ptr<cv::rgbd::ICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::ICPOdometry>*>))
	}
	
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:781
	void cv_rgbd_ICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::ICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:783
	void cv_rgbd_ICPOdometry_getCameraMatrix_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:787
	void cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:791
	void cv_rgbd_ICPOdometry_getMinDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:795
	void cv_rgbd_ICPOdometry_setMinDepth_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:799
	void cv_rgbd_ICPOdometry_getMaxDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:803
	void cv_rgbd_ICPOdometry_setMaxDepth_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:807
	void cv_rgbd_ICPOdometry_getMaxDepthDiff_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:811
	void cv_rgbd_ICPOdometry_setMaxDepthDiff_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:815
	void cv_rgbd_ICPOdometry_getIterationCounts_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:819
	void cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:823
	void cv_rgbd_ICPOdometry_getMaxPointsPart_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:827
	void cv_rgbd_ICPOdometry_setMaxPointsPart_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:831
	void cv_rgbd_ICPOdometry_getTransformType_const(const cv::rgbd::ICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:835
	void cv_rgbd_ICPOdometry_setTransformType_int(cv::rgbd::ICPOdometry* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:839
	void cv_rgbd_ICPOdometry_getMaxTranslation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:843
	void cv_rgbd_ICPOdometry_setMaxTranslation_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:847
	void cv_rgbd_ICPOdometry_getMaxRotation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:851
	void cv_rgbd_ICPOdometry_setMaxRotation_double(cv::rgbd::ICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormalsComputer() /usr/include/opencv2/rgbd/depth.hpp:855
	void cv_rgbd_ICPOdometry_getNormalsComputer_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	// DEFAULT_MIN_DEPTH() /usr/include/opencv2/rgbd/depth.hpp:535
	void cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MIN_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// DEFAULT_MAX_DEPTH() /usr/include/opencv2/rgbd/depth.hpp:540
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// DEFAULT_MAX_DEPTH_DIFF() /usr/include/opencv2/rgbd/depth.hpp:545
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// DEFAULT_MAX_POINTS_PART() /usr/include/opencv2/rgbd/depth.hpp:550
	void cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// DEFAULT_MAX_TRANSLATION() /usr/include/opencv2/rgbd/depth.hpp:555
	void cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// DEFAULT_MAX_ROTATION() /usr/include/opencv2/rgbd/depth.hpp:560
	void cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_ROTATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// compute(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, cv::OutputArray, const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:584
	void cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// compute(Ptr<cv::rgbd::OdometryFrame> &, Ptr<cv::rgbd::OdometryFrame> &, cv::OutputArray, const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:591
	void cv_rgbd_Odometry_compute_const_Ptr_OdometryFrame_R_Ptr_OdometryFrame_R_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:599
	void cv_rgbd_Odometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// create(const cv::String &) /usr/include/opencv2/rgbd/depth.hpp:601
	void cv_rgbd_Odometry_create_const_StringR(const char* odometryType, Result<cv::Ptr<cv::rgbd::Odometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::Odometry> ret = cv::rgbd::Odometry::create(std::string(odometryType));
			Ok(new cv::Ptr<cv::rgbd::Odometry>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::Odometry>*>))
	}
	
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:604
	void cv_rgbd_Odometry_getCameraMatrix_const(const cv::rgbd::Odometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:606
	void cv_rgbd_Odometry_setCameraMatrix_const_MatR(cv::rgbd::Odometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:608
	void cv_rgbd_Odometry_getTransformType_const(const cv::rgbd::Odometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:610
	void cv_rgbd_Odometry_setTransformType_int(cv::rgbd::Odometry* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// pyramidImage /usr/include/opencv2/rgbd/depth.hpp:508
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidImage_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidImage;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidImage /usr/include/opencv2/rgbd/depth.hpp:508
	void cv_rgbd_OdometryFrame_setPropPyramidImage_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidImage = *val;
	}
	
	// pyramidDepth /usr/include/opencv2/rgbd/depth.hpp:509
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidDepth_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidDepth;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidDepth /usr/include/opencv2/rgbd/depth.hpp:509
	void cv_rgbd_OdometryFrame_setPropPyramidDepth_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidDepth = *val;
	}
	
	// pyramidMask /usr/include/opencv2/rgbd/depth.hpp:510
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidMask;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidMask /usr/include/opencv2/rgbd/depth.hpp:510
	void cv_rgbd_OdometryFrame_setPropPyramidMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidMask = *val;
	}
	
	// pyramidCloud /usr/include/opencv2/rgbd/depth.hpp:512
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidCloud_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidCloud;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidCloud /usr/include/opencv2/rgbd/depth.hpp:512
	void cv_rgbd_OdometryFrame_setPropPyramidCloud_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidCloud = *val;
	}
	
	// pyramid_dI_dx /usr/include/opencv2/rgbd/depth.hpp:514
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramid_dI_dx_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dx;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramid_dI_dx /usr/include/opencv2/rgbd/depth.hpp:514
	void cv_rgbd_OdometryFrame_setPropPyramid_dI_dx_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dx = *val;
	}
	
	// pyramid_dI_dy /usr/include/opencv2/rgbd/depth.hpp:515
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramid_dI_dy_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dy;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramid_dI_dy /usr/include/opencv2/rgbd/depth.hpp:515
	void cv_rgbd_OdometryFrame_setPropPyramid_dI_dy_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dy = *val;
	}
	
	// pyramidTexturedMask /usr/include/opencv2/rgbd/depth.hpp:516
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidTexturedMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidTexturedMask;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidTexturedMask /usr/include/opencv2/rgbd/depth.hpp:516
	void cv_rgbd_OdometryFrame_setPropPyramidTexturedMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidTexturedMask = *val;
	}
	
	// pyramidNormals /usr/include/opencv2/rgbd/depth.hpp:518
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidNormals_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormals;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidNormals /usr/include/opencv2/rgbd/depth.hpp:518
	void cv_rgbd_OdometryFrame_setPropPyramidNormals_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidNormals = *val;
	}
	
	// pyramidNormalsMask /usr/include/opencv2/rgbd/depth.hpp:519
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_getPropPyramidNormalsMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormalsMask;
			return new std::vector<cv::Mat>(ret);
	}
	
	// pyramidNormalsMask /usr/include/opencv2/rgbd/depth.hpp:519
	void cv_rgbd_OdometryFrame_setPropPyramidNormalsMask_vector_Mat_(cv::rgbd::OdometryFrame* instance, std::vector<cv::Mat>* val) {
			instance->pyramidNormalsMask = *val;
	}
	
	cv::rgbd::RgbdFrame* cv_OdometryFrame_to_RgbdFrame(cv::rgbd::OdometryFrame* instance) {
		return dynamic_cast<cv::rgbd::RgbdFrame*>(instance);
	}
	
	void cv_OdometryFrame_delete(cv::rgbd::OdometryFrame* instance) {
		delete instance;
	}
	// OdometryFrame() /usr/include/opencv2/rgbd/depth.hpp:497
	void cv_rgbd_OdometryFrame_OdometryFrame(Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::OdometryFrame*>))
	}
	
	// OdometryFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:498
	void cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::OdometryFrame*>))
	}
	
	// create(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:500
	void cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::OdometryFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::OdometryFrame>*>))
	}
	
	// release() /usr/include/opencv2/rgbd/depth.hpp:503
	void cv_rgbd_OdometryFrame_release(cv::rgbd::OdometryFrame* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// releasePyramids() /usr/include/opencv2/rgbd/depth.hpp:506
	void cv_rgbd_OdometryFrame_releasePyramids(cv::rgbd::OdometryFrame* instance, Result_void* ocvrs_return) {
		try {
			instance->releasePyramids();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ID /usr/include/opencv2/rgbd/depth.hpp:472
	int cv_rgbd_RgbdFrame_getPropID_const(const cv::rgbd::RgbdFrame* instance) {
			int ret = instance->ID;
			return ret;
	}
	
	// ID /usr/include/opencv2/rgbd/depth.hpp:472
	void cv_rgbd_RgbdFrame_setPropID_int(cv::rgbd::RgbdFrame* instance, int val) {
			instance->ID = val;
	}
	
	// image /usr/include/opencv2/rgbd/depth.hpp:473
	cv::Mat* cv_rgbd_RgbdFrame_getPropImage_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->image;
			return new cv::Mat(ret);
	}
	
	// image /usr/include/opencv2/rgbd/depth.hpp:473
	void cv_rgbd_RgbdFrame_setPropImage_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
			instance->image = *val;
	}
	
	// depth /usr/include/opencv2/rgbd/depth.hpp:474
	cv::Mat* cv_rgbd_RgbdFrame_getPropDepth_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->depth;
			return new cv::Mat(ret);
	}
	
	// depth /usr/include/opencv2/rgbd/depth.hpp:474
	void cv_rgbd_RgbdFrame_setPropDepth_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
			instance->depth = *val;
	}
	
	// mask /usr/include/opencv2/rgbd/depth.hpp:475
	cv::Mat* cv_rgbd_RgbdFrame_getPropMask_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->mask;
			return new cv::Mat(ret);
	}
	
	// mask /usr/include/opencv2/rgbd/depth.hpp:475
	void cv_rgbd_RgbdFrame_setPropMask_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
			instance->mask = *val;
	}
	
	// normals /usr/include/opencv2/rgbd/depth.hpp:476
	cv::Mat* cv_rgbd_RgbdFrame_getPropNormals_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->normals;
			return new cv::Mat(ret);
	}
	
	// normals /usr/include/opencv2/rgbd/depth.hpp:476
	void cv_rgbd_RgbdFrame_setPropNormals_Mat(cv::rgbd::RgbdFrame* instance, cv::Mat* val) {
			instance->normals = *val;
	}
	
	cv::rgbd::OdometryFrame* cv_RgbdFrame_to_OdometryFrame(cv::rgbd::RgbdFrame* instance) {
		return dynamic_cast<cv::rgbd::OdometryFrame*>(instance);
	}
	
	void cv_RgbdFrame_delete(cv::rgbd::RgbdFrame* instance) {
		delete instance;
	}
	// RgbdFrame() /usr/include/opencv2/rgbd/depth.hpp:463
	void cv_rgbd_RgbdFrame_RgbdFrame(Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdFrame*>))
	}
	
	// RgbdFrame(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:464
	void cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdFrame*>))
	}
	
	// create(const cv::Mat &, const cv::Mat &, const cv::Mat &, const cv::Mat &, int) /usr/include/opencv2/rgbd/depth.hpp:467
	void cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::RgbdFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdFrame>*>))
	}
	
	// release() /usr/include/opencv2/rgbd/depth.hpp:470
	void cv_rgbd_RgbdFrame_release(cv::rgbd::RgbdFrame* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_RgbdICPOdometry_to_Algorithm(cv::rgbd::RgbdICPOdometry* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_RgbdICPOdometry_delete(cv::rgbd::RgbdICPOdometry* instance) {
		delete instance;
	}
	// RgbdICPOdometry() /usr/include/opencv2/rgbd/depth.hpp:890
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdICPOdometry*>))
	}
	
	// RgbdICPOdometry(const cv::Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int) /usr/include/opencv2/rgbd/depth.hpp:903
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdICPOdometry*>))
	}
	
	// create(const cv::Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int) /usr/include/opencv2/rgbd/depth.hpp:909
	void cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vector_int_R_const_vector_float_R_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>))
	}
	
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:915
	void cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::RgbdICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:917
	void cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:921
	void cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:925
	void cv_rgbd_RgbdICPOdometry_getMinDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:929
	void cv_rgbd_RgbdICPOdometry_setMinDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:933
	void cv_rgbd_RgbdICPOdometry_getMaxDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:937
	void cv_rgbd_RgbdICPOdometry_setMaxDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:941
	void cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:945
	void cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:949
	void cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:953
	void cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:957
	void cv_rgbd_RgbdICPOdometry_getIterationCounts_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:961
	void cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinGradientMagnitudes() /usr/include/opencv2/rgbd/depth.hpp:965
	void cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setMinGradientMagnitudes(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:969
	void cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:973
	void cv_rgbd_RgbdICPOdometry_getTransformType_const(const cv::rgbd::RgbdICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:977
	void cv_rgbd_RgbdICPOdometry_setTransformType_int(cv::rgbd::RgbdICPOdometry* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:981
	void cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:985
	void cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:989
	void cv_rgbd_RgbdICPOdometry_getMaxRotation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:993
	void cv_rgbd_RgbdICPOdometry_setMaxRotation_double(cv::rgbd::RgbdICPOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormalsComputer() /usr/include/opencv2/rgbd/depth.hpp:997
	void cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	cv::Algorithm* cv_RgbdNormals_to_Algorithm(cv::rgbd::RgbdNormals* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_RgbdNormals_delete(cv::rgbd::RgbdNormals* instance) {
		delete instance;
	}
	// RgbdNormals() /usr/include/opencv2/rgbd/depth.hpp:83
	void cv_rgbd_RgbdNormals_RgbdNormals(Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdNormals*>))
	}
	
	// RgbdNormals(int, int, int, cv::InputArray, int, int) /usr/include/opencv2/rgbd/depth.hpp:103
	void cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdNormals*>))
	}
	
	// create(int, int, int, cv::InputArray, int, int) /usr/include/opencv2/rgbd/depth.hpp:108
	void cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdNormals>*>))
	}
	
	// initialize() /usr/include/opencv2/rgbd/depth.hpp:122
	void cv_rgbd_RgbdNormals_initialize_const(const cv::rgbd::RgbdNormals* instance, Result_void* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRows() /usr/include/opencv2/rgbd/depth.hpp:124
	void cv_rgbd_RgbdNormals_getRows_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRows(int) /usr/include/opencv2/rgbd/depth.hpp:128
	void cv_rgbd_RgbdNormals_setRows_int(cv::rgbd::RgbdNormals* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setRows(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCols() /usr/include/opencv2/rgbd/depth.hpp:132
	void cv_rgbd_RgbdNormals_getCols_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCols(int) /usr/include/opencv2/rgbd/depth.hpp:136
	void cv_rgbd_RgbdNormals_setCols_int(cv::rgbd::RgbdNormals* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setCols(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getWindowSize() /usr/include/opencv2/rgbd/depth.hpp:140
	void cv_rgbd_RgbdNormals_getWindowSize_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setWindowSize(int) /usr/include/opencv2/rgbd/depth.hpp:144
	void cv_rgbd_RgbdNormals_setWindowSize_int(cv::rgbd::RgbdNormals* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDepth() /usr/include/opencv2/rgbd/depth.hpp:148
	void cv_rgbd_RgbdNormals_getDepth_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDepth(int) /usr/include/opencv2/rgbd/depth.hpp:152
	void cv_rgbd_RgbdNormals_setDepth_int(cv::rgbd::RgbdNormals* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getK() /usr/include/opencv2/rgbd/depth.hpp:156
	void cv_rgbd_RgbdNormals_getK_const(const cv::rgbd::RgbdNormals* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getK();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setK(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:160
	void cv_rgbd_RgbdNormals_setK_const_MatR(cv::rgbd::RgbdNormals* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setK(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:164
	void cv_rgbd_RgbdNormals_getMethod_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:168
	void cv_rgbd_RgbdNormals_setMethod_int(cv::rgbd::RgbdNormals* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_RgbdOdometry_to_Algorithm(cv::rgbd::RgbdOdometry* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_RgbdOdometry_delete(cv::rgbd::RgbdOdometry* instance) {
		delete instance;
	}
	// RgbdOdometry() /usr/include/opencv2/rgbd/depth.hpp:627
	void cv_rgbd_RgbdOdometry_RgbdOdometry(Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdOdometry*>))
	}
	
	// RgbdOdometry(const cv::Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int) /usr/include/opencv2/rgbd/depth.hpp:640
	void cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdOdometry*>))
	}
	
	// create(const cv::Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int) /usr/include/opencv2/rgbd/depth.hpp:645
	void cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vector_int_R_const_vector_float_R_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>))
	}
	
	// prepareFrameCache(Ptr<cv::rgbd::OdometryFrame> &, int) /usr/include/opencv2/rgbd/depth.hpp:650
	void cv_rgbd_RgbdOdometry_prepareFrameCache_const_Ptr_OdometryFrame_R_int(const cv::rgbd::RgbdOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getCameraMatrix() /usr/include/opencv2/rgbd/depth.hpp:652
	void cv_rgbd_RgbdOdometry_getCameraMatrix_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setCameraMatrix(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:656
	void cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinDepth() /usr/include/opencv2/rgbd/depth.hpp:660
	void cv_rgbd_RgbdOdometry_getMinDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMinDepth(double) /usr/include/opencv2/rgbd/depth.hpp:664
	void cv_rgbd_RgbdOdometry_setMinDepth_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepth() /usr/include/opencv2/rgbd/depth.hpp:668
	void cv_rgbd_RgbdOdometry_getMaxDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepth(double) /usr/include/opencv2/rgbd/depth.hpp:672
	void cv_rgbd_RgbdOdometry_setMaxDepth_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxDepthDiff() /usr/include/opencv2/rgbd/depth.hpp:676
	void cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxDepthDiff(double) /usr/include/opencv2/rgbd/depth.hpp:680
	void cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterationCounts() /usr/include/opencv2/rgbd/depth.hpp:684
	void cv_rgbd_RgbdOdometry_getIterationCounts_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setIterationCounts(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:688
	void cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinGradientMagnitudes() /usr/include/opencv2/rgbd/depth.hpp:692
	void cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setMinGradientMagnitudes(const cv::Mat &) /usr/include/opencv2/rgbd/depth.hpp:696
	void cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxPointsPart() /usr/include/opencv2/rgbd/depth.hpp:700
	void cv_rgbd_RgbdOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxPointsPart(double) /usr/include/opencv2/rgbd/depth.hpp:704
	void cv_rgbd_RgbdOdometry_setMaxPointsPart_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformType() /usr/include/opencv2/rgbd/depth.hpp:708
	void cv_rgbd_RgbdOdometry_getTransformType_const(const cv::rgbd::RgbdOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTransformType(int) /usr/include/opencv2/rgbd/depth.hpp:712
	void cv_rgbd_RgbdOdometry_setTransformType_int(cv::rgbd::RgbdOdometry* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxTranslation() /usr/include/opencv2/rgbd/depth.hpp:716
	void cv_rgbd_RgbdOdometry_getMaxTranslation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxTranslation(double) /usr/include/opencv2/rgbd/depth.hpp:720
	void cv_rgbd_RgbdOdometry_setMaxTranslation_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxRotation() /usr/include/opencv2/rgbd/depth.hpp:724
	void cv_rgbd_RgbdOdometry_getMaxRotation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setMaxRotation(double) /usr/include/opencv2/rgbd/depth.hpp:728
	void cv_rgbd_RgbdOdometry_setMaxRotation_double(cv::rgbd::RgbdOdometry* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_RgbdPlane_to_Algorithm(cv::rgbd::RgbdPlane* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_RgbdPlane_delete(cv::rgbd::RgbdPlane* instance) {
		delete instance;
	}
	// RgbdPlane(int) /usr/include/opencv2/rgbd/depth.hpp:337
	void cv_rgbd_RgbdPlane_RgbdPlane_int(int method, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdPlane*>))
	}
	
	// RgbdPlane(int, int, int, double, double, double, double) /usr/include/opencv2/rgbd/depth.hpp:358
	void cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::rgbd::RgbdPlane*>))
	}
	
	// create(int, int, int, double, double, double, double) /usr/include/opencv2/rgbd/depth.hpp:364
	void cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::Ptr<cv::rgbd::RgbdPlane>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::rgbd::RgbdPlane>*>))
	}
	
	// getBlockSize() /usr/include/opencv2/rgbd/depth.hpp:389
	void cv_rgbd_RgbdPlane_getBlockSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setBlockSize(int) /usr/include/opencv2/rgbd/depth.hpp:393
	void cv_rgbd_RgbdPlane_setBlockSize_int(cv::rgbd::RgbdPlane* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setBlockSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinSize() /usr/include/opencv2/rgbd/depth.hpp:397
	void cv_rgbd_RgbdPlane_getMinSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinSize(int) /usr/include/opencv2/rgbd/depth.hpp:401
	void cv_rgbd_RgbdPlane_setMinSize_int(cv::rgbd::RgbdPlane* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMethod() /usr/include/opencv2/rgbd/depth.hpp:405
	void cv_rgbd_RgbdPlane_getMethod_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMethod(int) /usr/include/opencv2/rgbd/depth.hpp:409
	void cv_rgbd_RgbdPlane_setMethod_int(cv::rgbd::RgbdPlane* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreshold() /usr/include/opencv2/rgbd/depth.hpp:413
	void cv_rgbd_RgbdPlane_getThreshold_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setThreshold(double) /usr/include/opencv2/rgbd/depth.hpp:417
	void cv_rgbd_RgbdPlane_setThreshold_double(cv::rgbd::RgbdPlane* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSensorErrorA() /usr/include/opencv2/rgbd/depth.hpp:421
	void cv_rgbd_RgbdPlane_getSensorErrorA_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSensorErrorA(double) /usr/include/opencv2/rgbd/depth.hpp:425
	void cv_rgbd_RgbdPlane_setSensorErrorA_double(cv::rgbd::RgbdPlane* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setSensorErrorA(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSensorErrorB() /usr/include/opencv2/rgbd/depth.hpp:429
	void cv_rgbd_RgbdPlane_getSensorErrorB_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSensorErrorB(double) /usr/include/opencv2/rgbd/depth.hpp:433
	void cv_rgbd_RgbdPlane_setSensorErrorB_double(cv::rgbd::RgbdPlane* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setSensorErrorB(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSensorErrorC() /usr/include/opencv2/rgbd/depth.hpp:437
	void cv_rgbd_RgbdPlane_getSensorErrorC_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSensorErrorC(double) /usr/include/opencv2/rgbd/depth.hpp:441
	void cv_rgbd_RgbdPlane_setSensorErrorC_double(cv::rgbd::RgbdPlane* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setSensorErrorC(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
