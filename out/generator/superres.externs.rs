extern "C" {
	// createFrameSource_Camera(int) /usr/include/opencv2/superres.hpp:80
	pub fn cv_superres_createFrameSource_Camera_int(device_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// createFrameSource_Empty() /usr/include/opencv2/superres.hpp:75
	pub fn cv_superres_createFrameSource_Empty(ocvrs_return: *mut Result<*mut c_void>);
	// createFrameSource_Video_CUDA(const cv::String &) /usr/include/opencv2/superres.hpp:78
	pub fn cv_superres_createFrameSource_Video_CUDA_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// createFrameSource_Video(const cv::String &) /usr/include/opencv2/superres.hpp:77
	pub fn cv_superres_createFrameSource_Video_const_StringR(file_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_Brox_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:177
	pub fn cv_superres_createOptFlow_Brox_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_DualTVL1() /usr/include/opencv2/superres/optical_flow.hpp:139
	pub fn cv_superres_createOptFlow_DualTVL1(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_DualTVL1_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:140
	pub fn cv_superres_createOptFlow_DualTVL1_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_Farneback() /usr/include/opencv2/superres/optical_flow.hpp:96
	pub fn cv_superres_createOptFlow_Farneback(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_Farneback_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:97
	pub fn cv_superres_createOptFlow_Farneback_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// createOptFlow_PyrLK_CUDA() /usr/include/opencv2/superres/optical_flow.hpp:196
	pub fn cv_superres_createOptFlow_PyrLK_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// createSuperResolution_BTVL1() /usr/include/opencv2/superres.hpp:199
	pub fn cv_superres_createSuperResolution_BTVL1(ocvrs_return: *mut Result<*mut c_void>);
	// createSuperResolution_BTVL1_CUDA() /usr/include/opencv2/superres.hpp:200
	pub fn cv_superres_createSuperResolution_BTVL1_CUDA(ocvrs_return: *mut Result<*mut c_void>);
	// getAlpha() /usr/include/opencv2/superres/optical_flow.hpp:148
	pub fn cv_superres_BroxOpticalFlow_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAlpha(double) /usr/include/opencv2/superres/optical_flow.hpp:150
	pub fn cv_superres_BroxOpticalFlow_setAlpha_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getGamma() /usr/include/opencv2/superres/optical_flow.hpp:153
	pub fn cv_superres_BroxOpticalFlow_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setGamma(double) /usr/include/opencv2/superres/optical_flow.hpp:155
	pub fn cv_superres_BroxOpticalFlow_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getScaleFactor() /usr/include/opencv2/superres/optical_flow.hpp:158
	pub fn cv_superres_BroxOpticalFlow_getScaleFactor_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setScaleFactor(double) /usr/include/opencv2/superres/optical_flow.hpp:160
	pub fn cv_superres_BroxOpticalFlow_setScaleFactor_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getInnerIterations() /usr/include/opencv2/superres/optical_flow.hpp:163
	pub fn cv_superres_BroxOpticalFlow_getInnerIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setInnerIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:165
	pub fn cv_superres_BroxOpticalFlow_setInnerIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getOuterIterations() /usr/include/opencv2/superres/optical_flow.hpp:168
	pub fn cv_superres_BroxOpticalFlow_getOuterIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setOuterIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:170
	pub fn cv_superres_BroxOpticalFlow_setOuterIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getSolverIterations() /usr/include/opencv2/superres/optical_flow.hpp:173
	pub fn cv_superres_BroxOpticalFlow_getSolverIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setSolverIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:175
	pub fn cv_superres_BroxOpticalFlow_setSolverIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// calc(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/superres/optical_flow.hpp:59
	pub fn cv_superres_DenseOpticalFlowExt_calc_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, frame0: *const c_void, frame1: *const c_void, flow1: *const c_void, flow2: *const c_void, ocvrs_return: *mut Result_void);
	// collectGarbage() /usr/include/opencv2/superres/optical_flow.hpp:60
	pub fn cv_superres_DenseOpticalFlowExt_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getTau() /usr/include/opencv2/superres/optical_flow.hpp:107
	pub fn cv_superres_DualTVL1OpticalFlow_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTau(double) /usr/include/opencv2/superres/optical_flow.hpp:109
	pub fn cv_superres_DualTVL1OpticalFlow_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/superres/optical_flow.hpp:111
	pub fn cv_superres_DualTVL1OpticalFlow_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setLambda(double) /usr/include/opencv2/superres/optical_flow.hpp:113
	pub fn cv_superres_DualTVL1OpticalFlow_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTheta() /usr/include/opencv2/superres/optical_flow.hpp:115
	pub fn cv_superres_DualTVL1OpticalFlow_getTheta_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTheta(double) /usr/include/opencv2/superres/optical_flow.hpp:117
	pub fn cv_superres_DualTVL1OpticalFlow_setTheta_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getScalesNumber() /usr/include/opencv2/superres/optical_flow.hpp:119
	pub fn cv_superres_DualTVL1OpticalFlow_getScalesNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setScalesNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:121
	pub fn cv_superres_DualTVL1OpticalFlow_setScalesNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWarpingsNumber() /usr/include/opencv2/superres/optical_flow.hpp:123
	pub fn cv_superres_DualTVL1OpticalFlow_getWarpingsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWarpingsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:125
	pub fn cv_superres_DualTVL1OpticalFlow_setWarpingsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getEpsilon() /usr/include/opencv2/superres/optical_flow.hpp:127
	pub fn cv_superres_DualTVL1OpticalFlow_getEpsilon_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setEpsilon(double) /usr/include/opencv2/superres/optical_flow.hpp:129
	pub fn cv_superres_DualTVL1OpticalFlow_setEpsilon_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:131
	pub fn cv_superres_DualTVL1OpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:133
	pub fn cv_superres_DualTVL1OpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getUseInitialFlow() /usr/include/opencv2/superres/optical_flow.hpp:135
	pub fn cv_superres_DualTVL1OpticalFlow_getUseInitialFlow_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseInitialFlow(bool) /usr/include/opencv2/superres/optical_flow.hpp:137
	pub fn cv_superres_DualTVL1OpticalFlow_setUseInitialFlow_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result_void);
	// getPyrScale() /usr/include/opencv2/superres/optical_flow.hpp:68
	pub fn cv_superres_FarnebackOpticalFlow_getPyrScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setPyrScale(double) /usr/include/opencv2/superres/optical_flow.hpp:70
	pub fn cv_superres_FarnebackOpticalFlow_setPyrScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getLevelsNumber() /usr/include/opencv2/superres/optical_flow.hpp:72
	pub fn cv_superres_FarnebackOpticalFlow_getLevelsNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setLevelsNumber(int) /usr/include/opencv2/superres/optical_flow.hpp:74
	pub fn cv_superres_FarnebackOpticalFlow_setLevelsNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:76
	pub fn cv_superres_FarnebackOpticalFlow_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:78
	pub fn cv_superres_FarnebackOpticalFlow_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:80
	pub fn cv_superres_FarnebackOpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:82
	pub fn cv_superres_FarnebackOpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getPolyN() /usr/include/opencv2/superres/optical_flow.hpp:84
	pub fn cv_superres_FarnebackOpticalFlow_getPolyN_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setPolyN(int) /usr/include/opencv2/superres/optical_flow.hpp:86
	pub fn cv_superres_FarnebackOpticalFlow_setPolyN_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getPolySigma() /usr/include/opencv2/superres/optical_flow.hpp:88
	pub fn cv_superres_FarnebackOpticalFlow_getPolySigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setPolySigma(double) /usr/include/opencv2/superres/optical_flow.hpp:90
	pub fn cv_superres_FarnebackOpticalFlow_setPolySigma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getFlags() /usr/include/opencv2/superres/optical_flow.hpp:92
	pub fn cv_superres_FarnebackOpticalFlow_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setFlags(int) /usr/include/opencv2/superres/optical_flow.hpp:94
	pub fn cv_superres_FarnebackOpticalFlow_setFlags_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:71
	pub fn cv_superres_FrameSource_nextFrame_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/superres.hpp:72
	pub fn cv_superres_FrameSource_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/superres/optical_flow.hpp:184
	pub fn cv_superres_PyrLKOpticalFlow_getWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setWindowSize(int) /usr/include/opencv2/superres/optical_flow.hpp:186
	pub fn cv_superres_PyrLKOpticalFlow_setWindowSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getMaxLevel() /usr/include/opencv2/superres/optical_flow.hpp:188
	pub fn cv_superres_PyrLKOpticalFlow_getMaxLevel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setMaxLevel(int) /usr/include/opencv2/superres/optical_flow.hpp:190
	pub fn cv_superres_PyrLKOpticalFlow_setMaxLevel_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/superres/optical_flow.hpp:192
	pub fn cv_superres_PyrLKOpticalFlow_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterations(int) /usr/include/opencv2/superres/optical_flow.hpp:194
	pub fn cv_superres_PyrLKOpticalFlow_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// setInput(const Ptr<cv::superres::FrameSource> &) /usr/include/opencv2/superres.hpp:94
	pub fn cv_superres_SuperResolution_setInput_const_Ptr_FrameSource_R(instance: *mut c_void, frame_source: *const c_void, ocvrs_return: *mut Result_void);
	// nextFrame(cv::OutputArray) /usr/include/opencv2/superres.hpp:100
	pub fn cv_superres_SuperResolution_nextFrame_const__OutputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result_void);
	// reset() /usr/include/opencv2/superres.hpp:101
	pub fn cv_superres_SuperResolution_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// collectGarbage() /usr/include/opencv2/superres.hpp:105
	pub fn cv_superres_SuperResolution_collectGarbage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getScale() /usr/include/opencv2/superres.hpp:109
	pub fn cv_superres_SuperResolution_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setScale(int) /usr/include/opencv2/superres.hpp:111
	pub fn cv_superres_SuperResolution_setScale_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getIterations() /usr/include/opencv2/superres.hpp:115
	pub fn cv_superres_SuperResolution_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setIterations(int) /usr/include/opencv2/superres.hpp:117
	pub fn cv_superres_SuperResolution_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getTau() /usr/include/opencv2/superres.hpp:121
	pub fn cv_superres_SuperResolution_getTau_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setTau(double) /usr/include/opencv2/superres.hpp:123
	pub fn cv_superres_SuperResolution_setTau_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getLambda() /usr/include/opencv2/superres.hpp:127
	pub fn cv_superres_SuperResolution_getLambda_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setLambda(double) /usr/include/opencv2/superres.hpp:129
	pub fn cv_superres_SuperResolution_setLambda_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getAlpha() /usr/include/opencv2/superres.hpp:133
	pub fn cv_superres_SuperResolution_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setAlpha(double) /usr/include/opencv2/superres.hpp:135
	pub fn cv_superres_SuperResolution_setAlpha_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getKernelSize() /usr/include/opencv2/superres.hpp:139
	pub fn cv_superres_SuperResolution_getKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setKernelSize(int) /usr/include/opencv2/superres.hpp:141
	pub fn cv_superres_SuperResolution_setKernelSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getBlurKernelSize() /usr/include/opencv2/superres.hpp:145
	pub fn cv_superres_SuperResolution_getBlurKernelSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setBlurKernelSize(int) /usr/include/opencv2/superres.hpp:147
	pub fn cv_superres_SuperResolution_setBlurKernelSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getBlurSigma() /usr/include/opencv2/superres.hpp:151
	pub fn cv_superres_SuperResolution_getBlurSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBlurSigma(double) /usr/include/opencv2/superres.hpp:153
	pub fn cv_superres_SuperResolution_setBlurSigma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getTemporalAreaRadius() /usr/include/opencv2/superres.hpp:157
	pub fn cv_superres_SuperResolution_getTemporalAreaRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setTemporalAreaRadius(int) /usr/include/opencv2/superres.hpp:159
	pub fn cv_superres_SuperResolution_setTemporalAreaRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getOpticalFlow() /usr/include/opencv2/superres.hpp:163
	pub fn cv_superres_SuperResolution_getOpticalFlow_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setOpticalFlow(const Ptr<cv::superres::DenseOpticalFlowExt> &) /usr/include/opencv2/superres.hpp:165
	pub fn cv_superres_SuperResolution_setOpticalFlow_const_Ptr_DenseOpticalFlowExt_R(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result_void);
}
