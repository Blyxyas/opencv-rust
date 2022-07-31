extern "C" {
	// MotionSaliencyBinWangApr2014() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:172
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:175
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_create(ocvrs_return: *mut Result<*mut c_void>);
	// computeSaliency(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:180
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, saliency_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// setImagesize(int, int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:193
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(instance: *mut c_void, w: i32, h: i32, ocvrs_return: *mut Result_void);
	// init() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:197
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_init(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getImageWidth() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:199
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setImageWidth(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:203
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getImageHeight() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:207
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setImageHeight(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:211
	pub fn cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// ObjectnessBING() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:295
	pub fn cv_saliency_ObjectnessBING_ObjectnessBING(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:298
	pub fn cv_saliency_ObjectnessBING_create(ocvrs_return: *mut Result<*mut c_void>);
	// computeSaliency(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:303
	pub fn cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, saliency_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// read() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:311
	pub fn cv_saliency_ObjectnessBING_read(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// write() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:312
	pub fn cv_saliency_ObjectnessBING_write_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// getobjectnessValues() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:320
	pub fn cv_saliency_ObjectnessBING_getobjectnessValues(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setTrainingPath(const cv::String &) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:326
	pub fn cv_saliency_ObjectnessBING_setTrainingPath_const_StringR(instance: *mut c_void, training_path: *const c_char, ocvrs_return: *mut Result_void);
	// setBBResDir(const cv::String &) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:335
	pub fn cv_saliency_ObjectnessBING_setBBResDir_const_StringR(instance: *mut c_void, results_dir: *const c_char, ocvrs_return: *mut Result_void);
	// getBase() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:337
	pub fn cv_saliency_ObjectnessBING_getBase_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setBase(double) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:341
	pub fn cv_saliency_ObjectnessBING_setBase_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getNSS() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:345
	pub fn cv_saliency_ObjectnessBING_getNSS_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNSS(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:349
	pub fn cv_saliency_ObjectnessBING_setNSS_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getW() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:353
	pub fn cv_saliency_ObjectnessBING_getW_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setW(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:357
	pub fn cv_saliency_ObjectnessBING_setW_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// computeSaliency(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencyBaseClasses.hpp:76
	pub fn cv_saliency_Saliency_computeSaliency_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, saliency_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// computeBinaryMap(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencyBaseClasses.hpp:104
	pub fn cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, _saliency_map: *const c_void, _binary_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// StaticSaliencyFineGrained() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:126
	pub fn cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:128
	pub fn cv_saliency_StaticSaliencyFineGrained_create(ocvrs_return: *mut Result<*mut c_void>);
	// computeSaliency(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:133
	pub fn cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, saliency_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// StaticSaliencySpectralResidual() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:73
	pub fn cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual(ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:76
	pub fn cv_saliency_StaticSaliencySpectralResidual_create(ocvrs_return: *mut Result<*mut c_void>);
	// computeSaliency(cv::InputArray, cv::OutputArray) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:81
	pub fn cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, saliency_map: *const c_void, ocvrs_return: *mut Result<bool>);
	// read(const cv::FileNode &) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:89
	pub fn cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:90
	pub fn cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// getImageWidth() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:92
	pub fn cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setImageWidth(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:96
	pub fn cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getImageHeight() /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:100
	pub fn cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setImageHeight(int) /usr/include/opencv2/saliency/saliencySpecializedClasses.hpp:104
	pub fn cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
}
