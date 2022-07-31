extern "C" {
	// createFacemarkAAM() /usr/include/opencv2/face/facemark.hpp:83
	pub fn cv_face_createFacemarkAAM(ocvrs_return: *mut Result<*mut c_void>);
	// createFacemarkKazemi() /usr/include/opencv2/face/facemark.hpp:89
	pub fn cv_face_createFacemarkKazemi(ocvrs_return: *mut Result<*mut c_void>);
	// createFacemarkLBF() /usr/include/opencv2/face/facemark.hpp:86
	pub fn cv_face_createFacemarkLBF(ocvrs_return: *mut Result<*mut c_void>);
	// drawFacemarks(cv::InputOutputArray, cv::InputArray, cv::Scalar) /usr/include/opencv2/face/facemark_train.hpp:232
	pub fn cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(image: *const c_void, points: *const c_void, color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// getFacesHAAR(cv::InputArray, cv::OutputArray, const cv::String &) /usr/include/opencv2/face/facemark_train.hpp:76
	pub fn cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(image: *const c_void, faces: *const c_void, face_cascade_name: *const c_char, ocvrs_return: *mut Result<bool>);
	// getFaces(cv::InputArray, cv::OutputArray, cv::face::CParams *) /usr/include/opencv2/face/facemark_train.hpp:74
	pub fn cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(image: *const c_void, faces: *const c_void, params: *mut c_void, ocvrs_return: *mut Result<bool>);
	// loadDatasetList(cv::String, cv::String, std::vector<String> &, std::vector<String> &) /usr/include/opencv2/face/facemark_train.hpp:93
	pub fn cv_face_loadDatasetList_String_String_vector_String_R_vector_String_R(image_list: *mut c_char, annotation_list: *mut c_char, images: *mut c_void, annotations: *mut c_void, ocvrs_return: *mut Result<bool>);
	// loadFacePoints(cv::String, cv::OutputArray, float) /usr/include/opencv2/face/facemark_train.hpp:212
	pub fn cv_face_loadFacePoints_String_const__OutputArrayR_float(filename: *mut c_char, points: *const c_void, offset: f32, ocvrs_return: *mut Result<bool>);
	// loadTrainingData(cv::String, cv::String, std::vector<String> &, cv::OutputArray, float) /usr/include/opencv2/face/facemark_train.hpp:163
	pub fn cv_face_loadTrainingData_String_String_vector_String_R_const__OutputArrayR_float(image_list: *mut c_char, ground_truth: *mut c_char, images: *mut c_void, face_points: *const c_void, offset: f32, ocvrs_return: *mut Result<bool>);
	// loadTrainingData(cv::String, std::vector<String> &, cv::OutputArray, char, float) /usr/include/opencv2/face/facemark_train.hpp:123
	pub fn cv_face_loadTrainingData_String_vector_String_R_const__OutputArrayR_char_float(filename: *mut c_char, images: *mut c_void, face_points: *const c_void, delim: i8, offset: f32, ocvrs_return: *mut Result<bool>);
	// loadTrainingData(std::vector<String>, std::vector<std::vector<Point2f>> &, std::vector<String> &) /usr/include/opencv2/face/facemark_train.hpp:184
	pub fn cv_face_loadTrainingData_vector_String__vector_vector_Point2f__R_vector_String_R(filename: *mut c_void, trainlandmarks: *mut c_void, trainimages: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getNumBands() /usr/include/opencv2/face/bif.hpp:60
	pub fn cv_face_BIF_getNumBands_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getNumRotations() /usr/include/opencv2/face/bif.hpp:63
	pub fn cv_face_BIF_getNumRotations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// compute(cv::InputArray, cv::OutputArray) /usr/include/opencv2/face/bif.hpp:69
	pub fn cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, image: *const c_void, features: *const c_void, ocvrs_return: *mut Result_void);
	// create(int, int) /usr/include/opencv2/face/bif.hpp:77
	pub fn cv_face_BIF_create_int_int(num_bands: i32, num_rotations: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getNumComponents() /usr/include/opencv2/face/facerec.hpp:24
	pub fn cv_face_BasicFaceRecognizer_getNumComponents_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNumComponents(int) /usr/include/opencv2/face/facerec.hpp:26
	pub fn cv_face_BasicFaceRecognizer_setNumComponents_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/face/facerec.hpp:28
	pub fn cv_face_BasicFaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setThreshold(double) /usr/include/opencv2/face/facerec.hpp:30
	pub fn cv_face_BasicFaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getProjections() /usr/include/opencv2/face/facerec.hpp:31
	pub fn cv_face_BasicFaceRecognizer_getProjections_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLabels() /usr/include/opencv2/face/facerec.hpp:32
	pub fn cv_face_BasicFaceRecognizer_getLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getEigenValues() /usr/include/opencv2/face/facerec.hpp:33
	pub fn cv_face_BasicFaceRecognizer_getEigenValues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getEigenVectors() /usr/include/opencv2/face/facerec.hpp:34
	pub fn cv_face_BasicFaceRecognizer_getEigenVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMean() /usr/include/opencv2/face/facerec.hpp:35
	pub fn cv_face_BasicFaceRecognizer_getMean_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/face/facerec.hpp:37
	pub fn cv_face_BasicFaceRecognizer_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/face/facerec.hpp:38
	pub fn cv_face_BasicFaceRecognizer_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/face/facerec.hpp:39
	pub fn cv_face_BasicFaceRecognizer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// cascade /usr/include/opencv2/face/facemark_train.hpp:36
	pub fn cv_face_CParams_getPropCascade_const(instance: *const c_void) -> *mut c_void;
	// cascade /usr/include/opencv2/face/facemark_train.hpp:36
	pub fn cv_face_CParams_setPropCascade_String(instance: *mut c_void, val: *mut c_char);
	// scaleFactor /usr/include/opencv2/face/facemark_train.hpp:37
	pub fn cv_face_CParams_getPropScaleFactor_const(instance: *const c_void) -> f64;
	// scaleFactor /usr/include/opencv2/face/facemark_train.hpp:37
	pub fn cv_face_CParams_setPropScaleFactor_double(instance: *mut c_void, val: f64);
	// minNeighbors /usr/include/opencv2/face/facemark_train.hpp:38
	pub fn cv_face_CParams_getPropMinNeighbors_const(instance: *const c_void) -> i32;
	// minNeighbors /usr/include/opencv2/face/facemark_train.hpp:38
	pub fn cv_face_CParams_setPropMinNeighbors_int(instance: *mut c_void, val: i32);
	// minSize /usr/include/opencv2/face/facemark_train.hpp:39
	pub fn cv_face_CParams_getPropMinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// minSize /usr/include/opencv2/face/facemark_train.hpp:39
	pub fn cv_face_CParams_setPropMinSize_Size(instance: *mut c_void, val: *const core::Size);
	// maxSize /usr/include/opencv2/face/facemark_train.hpp:40
	pub fn cv_face_CParams_getPropMaxSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// maxSize /usr/include/opencv2/face/facemark_train.hpp:40
	pub fn cv_face_CParams_setPropMaxSize_Size(instance: *mut c_void, val: *const core::Size);
	// face_cascade /usr/include/opencv2/face/facemark_train.hpp:50
	pub fn cv_face_CParams_getPropFace_cascade_const(instance: *const c_void) -> *mut c_void;
	// face_cascade /usr/include/opencv2/face/facemark_train.hpp:50
	pub fn cv_face_CParams_setPropFace_cascade_CascadeClassifier(instance: *mut c_void, val: *mut c_void);
	// CParams(cv::String, double, int, cv::Size, cv::Size) /usr/include/opencv2/face/facemark_train.hpp:42
	pub fn cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model: *mut c_char, sf: f64, min_n: i32, min_sz: *const core::Size, max_sz: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, double) /usr/include/opencv2/face/facerec.hpp:86
	pub fn cv_face_EigenFaceRecognizer_create_int_double(num_components: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// train(cv::InputArrayOfArrays, cv::InputArray) /usr/include/opencv2/face.hpp:209
	pub fn cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result_void);
	// update(cv::InputArrayOfArrays, cv::InputArray) /usr/include/opencv2/face.hpp:258
	pub fn cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result_void);
	// predict(cv::InputArray) /usr/include/opencv2/face.hpp:261
	pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR(instance: *const c_void, src: *const c_void, ocvrs_return: *mut Result<i32>);
	// predict(cv::InputArray, int &, double &) /usr/include/opencv2/face.hpp:299
	pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(instance: *const c_void, src: *const c_void, label: *mut i32, confidence: *mut f64, ocvrs_return: *mut Result_void);
	// predict(cv::InputArray, Ptr<cv::face::PredictCollector>) /usr/include/opencv2/face.hpp:309
	pub fn cv_face_FaceRecognizer_predict_const_const__InputArrayR_Ptr_PredictCollector_(instance: *const c_void, src: *const c_void, collector: *mut c_void, ocvrs_return: *mut Result_void);
	// write(const cv::String &) /usr/include/opencv2/face.hpp:323
	pub fn cv_face_FaceRecognizer_write_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::String &) /usr/include/opencv2/face.hpp:332
	pub fn cv_face_FaceRecognizer_read_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/face.hpp:338
	pub fn cv_face_FaceRecognizer_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/face.hpp:341
	pub fn cv_face_FaceRecognizer_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/face.hpp:344
	pub fn cv_face_FaceRecognizer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setLabelInfo(int, const cv::String &) /usr/include/opencv2/face.hpp:350
	pub fn cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(instance: *mut c_void, label: i32, str_info: *const c_char, ocvrs_return: *mut Result_void);
	// getLabelInfo(int) /usr/include/opencv2/face.hpp:357
	pub fn cv_face_FaceRecognizer_getLabelInfo_const_int(instance: *const c_void, label: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getLabelsByString(const cv::String &) /usr/include/opencv2/face.hpp:364
	pub fn cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(instance: *const c_void, str: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getThreshold() /usr/include/opencv2/face.hpp:366
	pub fn cv_face_FaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setThreshold(double) /usr/include/opencv2/face.hpp:368
	pub fn cv_face_FaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// loadModel(cv::String) /usr/include/opencv2/face/facemark.hpp:59
	pub fn cv_face_Facemark_loadModel_String(instance: *mut c_void, model: *mut c_char, ocvrs_return: *mut Result_void);
	// fit(cv::InputArray, cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/face/facemark.hpp:76
	pub fn cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, landmarks: *const c_void, ocvrs_return: *mut Result<bool>);
	// fitConfig(cv::InputArray, cv::InputArray, cv::OutputArrayOfArrays, const std::vector<Config> &) /usr/include/opencv2/face/facemarkAAM.hpp:149
	pub fn cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vector_Config_R(instance: *mut c_void, image: *const c_void, roi: *const c_void, _landmarks: *const c_void, runtime_params: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(const FacemarkAAM::Params &) /usr/include/opencv2/face/facemarkAAM.hpp:153
	pub fn cv_face_FacemarkAAM_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// R /usr/include/opencv2/face/facemarkAAM.hpp:88
	pub fn cv_face_FacemarkAAM_Config_getPropR_const(instance: *const c_void) -> *mut c_void;
	// R /usr/include/opencv2/face/facemarkAAM.hpp:88
	pub fn cv_face_FacemarkAAM_Config_setPropR_Mat(instance: *mut c_void, val: *mut c_void);
	// t /usr/include/opencv2/face/facemarkAAM.hpp:89
	pub fn cv_face_FacemarkAAM_Config_getPropT_const(instance: *const c_void, ocvrs_return: *mut core::Point2f);
	// t /usr/include/opencv2/face/facemarkAAM.hpp:89
	pub fn cv_face_FacemarkAAM_Config_setPropT_Point2f(instance: *mut c_void, val: *const core::Point2f);
	// scale /usr/include/opencv2/face/facemarkAAM.hpp:90
	pub fn cv_face_FacemarkAAM_Config_getPropScale_const(instance: *const c_void) -> f32;
	// scale /usr/include/opencv2/face/facemarkAAM.hpp:90
	pub fn cv_face_FacemarkAAM_Config_setPropScale_float(instance: *mut c_void, val: f32);
	// model_scale_idx /usr/include/opencv2/face/facemarkAAM.hpp:91
	pub fn cv_face_FacemarkAAM_Config_getPropModel_scale_idx_const(instance: *const c_void) -> i32;
	// model_scale_idx /usr/include/opencv2/face/facemarkAAM.hpp:91
	pub fn cv_face_FacemarkAAM_Config_setPropModel_scale_idx_int(instance: *mut c_void, val: i32);
	// Config(cv::Mat, cv::Point2f, float, int) /usr/include/opencv2/face/facemarkAAM.hpp:82
	pub fn cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot: *mut c_void, trans: *const core::Point2f, scaling: f32, scale_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// s0 /usr/include/opencv2/face/facemarkAAM.hpp:100
	pub fn cv_face_FacemarkAAM_Data_getPropS0_const(instance: *const c_void) -> *mut c_void;
	// s0 /usr/include/opencv2/face/facemarkAAM.hpp:100
	pub fn cv_face_FacemarkAAM_Data_setPropS0_vector_Point2f_(instance: *mut c_void, val: *mut c_void);
	// scales /usr/include/opencv2/face/facemarkAAM.hpp:108
	pub fn cv_face_FacemarkAAM_Model_getPropScales_const(instance: *const c_void) -> *mut c_void;
	// scales /usr/include/opencv2/face/facemarkAAM.hpp:108
	pub fn cv_face_FacemarkAAM_Model_setPropScales_vector_float_(instance: *mut c_void, val: *mut c_void);
	// triangles /usr/include/opencv2/face/facemarkAAM.hpp:112
	pub fn cv_face_FacemarkAAM_Model_getPropTriangles_const(instance: *const c_void) -> *mut c_void;
	// triangles /usr/include/opencv2/face/facemarkAAM.hpp:112
	pub fn cv_face_FacemarkAAM_Model_setPropTriangles_vector_Vec3i_(instance: *mut c_void, val: *mut c_void);
	// textures /usr/include/opencv2/face/facemarkAAM.hpp:137
	pub fn cv_face_FacemarkAAM_Model_getPropTextures_const(instance: *const c_void) -> *mut c_void;
	// textures /usr/include/opencv2/face/facemarkAAM.hpp:137
	pub fn cv_face_FacemarkAAM_Model_setPropTextures_vector_Texture_(instance: *mut c_void, val: *mut c_void);
	// s0 /usr/include/opencv2/face/facemarkAAM.hpp:141
	pub fn cv_face_FacemarkAAM_Model_getPropS0_const(instance: *const c_void) -> *mut c_void;
	// s0 /usr/include/opencv2/face/facemarkAAM.hpp:141
	pub fn cv_face_FacemarkAAM_Model_setPropS0_vector_Point2f_(instance: *mut c_void, val: *mut c_void);
	// S /usr/include/opencv2/face/facemarkAAM.hpp:143
	pub fn cv_face_FacemarkAAM_Model_getPropS_const(instance: *const c_void) -> *mut c_void;
	// S /usr/include/opencv2/face/facemarkAAM.hpp:143
	pub fn cv_face_FacemarkAAM_Model_setPropS_Mat(instance: *mut c_void, val: *mut c_void);
	// Q /usr/include/opencv2/face/facemarkAAM.hpp:143
	pub fn cv_face_FacemarkAAM_Model_getPropQ_const(instance: *const c_void) -> *mut c_void;
	// Q /usr/include/opencv2/face/facemarkAAM.hpp:143
	pub fn cv_face_FacemarkAAM_Model_setPropQ_Mat(instance: *mut c_void, val: *mut c_void);
	// max_m /usr/include/opencv2/face/facemarkAAM.hpp:116
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropMax_m_const(instance: *const c_void) -> i32;
	// max_m /usr/include/opencv2/face/facemarkAAM.hpp:116
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropMax_m_int(instance: *mut c_void, val: i32);
	// resolution /usr/include/opencv2/face/facemarkAAM.hpp:117
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropResolution_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
	// resolution /usr/include/opencv2/face/facemarkAAM.hpp:117
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropResolution_Rect(instance: *mut c_void, val: *const core::Rect);
	// A /usr/include/opencv2/face/facemarkAAM.hpp:119
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropA_const(instance: *const c_void) -> *mut c_void;
	// A /usr/include/opencv2/face/facemarkAAM.hpp:119
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropA_Mat(instance: *mut c_void, val: *mut c_void);
	// A0 /usr/include/opencv2/face/facemarkAAM.hpp:121
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropA0_const(instance: *const c_void) -> *mut c_void;
	// A0 /usr/include/opencv2/face/facemarkAAM.hpp:121
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropA0_Mat(instance: *mut c_void, val: *mut c_void);
	// AA /usr/include/opencv2/face/facemarkAAM.hpp:123
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropAA_const(instance: *const c_void) -> *mut c_void;
	// AA /usr/include/opencv2/face/facemarkAAM.hpp:123
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropAA_Mat(instance: *mut c_void, val: *mut c_void);
	// AA0 /usr/include/opencv2/face/facemarkAAM.hpp:125
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropAA0_const(instance: *const c_void) -> *mut c_void;
	// AA0 /usr/include/opencv2/face/facemarkAAM.hpp:125
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropAA0_Mat(instance: *mut c_void, val: *mut c_void);
	// textureIdx /usr/include/opencv2/face/facemarkAAM.hpp:128
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropTextureIdx_const(instance: *const c_void) -> *mut c_void;
	// textureIdx /usr/include/opencv2/face/facemarkAAM.hpp:128
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropTextureIdx_vector_vector_Point__(instance: *mut c_void, val: *mut c_void);
	// base_shape /usr/include/opencv2/face/facemarkAAM.hpp:130
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropBase_shape_const(instance: *const c_void) -> *mut c_void;
	// base_shape /usr/include/opencv2/face/facemarkAAM.hpp:130
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropBase_shape_vector_Point2f_(instance: *mut c_void, val: *mut c_void);
	// ind1 /usr/include/opencv2/face/facemarkAAM.hpp:132
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropInd1_const(instance: *const c_void) -> *mut c_void;
	// ind1 /usr/include/opencv2/face/facemarkAAM.hpp:132
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropInd1_vector_int_(instance: *mut c_void, val: *mut c_void);
	// ind2 /usr/include/opencv2/face/facemarkAAM.hpp:134
	pub fn cv_face_FacemarkAAM_Model_Texture_getPropInd2_const(instance: *const c_void) -> *mut c_void;
	// ind2 /usr/include/opencv2/face/facemarkAAM.hpp:134
	pub fn cv_face_FacemarkAAM_Model_Texture_setPropInd2_vector_int_(instance: *mut c_void, val: *mut c_void);
	// model_filename /usr/include/opencv2/face/facemarkAAM.hpp:67
	pub fn cv_face_FacemarkAAM_Params_getPropModel_filename_const(instance: *const c_void) -> *mut c_void;
	// model_filename /usr/include/opencv2/face/facemarkAAM.hpp:67
	pub fn cv_face_FacemarkAAM_Params_setPropModel_filename_string(instance: *mut c_void, val: *mut c_char);
	// m /usr/include/opencv2/face/facemarkAAM.hpp:68
	pub fn cv_face_FacemarkAAM_Params_getPropM_const(instance: *const c_void) -> i32;
	// m /usr/include/opencv2/face/facemarkAAM.hpp:68
	pub fn cv_face_FacemarkAAM_Params_setPropM_int(instance: *mut c_void, val: i32);
	// n /usr/include/opencv2/face/facemarkAAM.hpp:69
	pub fn cv_face_FacemarkAAM_Params_getPropN_const(instance: *const c_void) -> i32;
	// n /usr/include/opencv2/face/facemarkAAM.hpp:69
	pub fn cv_face_FacemarkAAM_Params_setPropN_int(instance: *mut c_void, val: i32);
	// n_iter /usr/include/opencv2/face/facemarkAAM.hpp:70
	pub fn cv_face_FacemarkAAM_Params_getPropN_iter_const(instance: *const c_void) -> i32;
	// n_iter /usr/include/opencv2/face/facemarkAAM.hpp:70
	pub fn cv_face_FacemarkAAM_Params_setPropN_iter_int(instance: *mut c_void, val: i32);
	// verbose /usr/include/opencv2/face/facemarkAAM.hpp:71
	pub fn cv_face_FacemarkAAM_Params_getPropVerbose_const(instance: *const c_void) -> bool;
	// verbose /usr/include/opencv2/face/facemarkAAM.hpp:71
	pub fn cv_face_FacemarkAAM_Params_setPropVerbose_bool(instance: *mut c_void, val: bool);
	// save_model /usr/include/opencv2/face/facemarkAAM.hpp:72
	pub fn cv_face_FacemarkAAM_Params_getPropSave_model_const(instance: *const c_void) -> bool;
	// save_model /usr/include/opencv2/face/facemarkAAM.hpp:72
	pub fn cv_face_FacemarkAAM_Params_setPropSave_model_bool(instance: *mut c_void, val: bool);
	// max_m /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_getPropMax_m_const(instance: *const c_void) -> i32;
	// max_m /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_setPropMax_m_int(instance: *mut c_void, val: i32);
	// max_n /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_getPropMax_n_const(instance: *const c_void) -> i32;
	// max_n /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_setPropMax_n_int(instance: *mut c_void, val: i32);
	// texture_max_m /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_getPropTexture_max_m_const(instance: *const c_void) -> i32;
	// texture_max_m /usr/include/opencv2/face/facemarkAAM.hpp:73
	pub fn cv_face_FacemarkAAM_Params_setPropTexture_max_m_int(instance: *mut c_void, val: i32);
	// scales /usr/include/opencv2/face/facemarkAAM.hpp:74
	pub fn cv_face_FacemarkAAM_Params_getPropScales_const(instance: *const c_void) -> *mut c_void;
	// scales /usr/include/opencv2/face/facemarkAAM.hpp:74
	pub fn cv_face_FacemarkAAM_Params_setPropScales_vector_float_(instance: *mut c_void, val: *mut c_void);
	// Params() /usr/include/opencv2/face/facemarkAAM.hpp:55
	pub fn cv_face_FacemarkAAM_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/face/facemarkAAM.hpp:60
	pub fn cv_face_FacemarkAAM_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/face/facemarkAAM.hpp:65
	pub fn cv_face_FacemarkAAM_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// create(const FacemarkKazemi::Params &) /usr/include/opencv2/face/face_alignment.hpp:39
	pub fn cv_face_FacemarkKazemi_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// training(std::vector<Mat> &, std::vector<std::vector<Point2f>> &, std::string, cv::Size, std::string) /usr/include/opencv2/face/face_alignment.hpp:51
	pub fn cv_face_FacemarkKazemi_training_vector_Mat_R_vector_vector_Point2f__R_string_Size_string(instance: *mut c_void, images: *mut c_void, landmarks: *mut c_void, configfile: *mut c_char, scale: *const core::Size, model_filename: *mut c_char, ocvrs_return: *mut Result<bool>);
	// setFaceDetector(bool (*)(cv::InputArray, cv::OutputArray, void *), void *) /usr/include/opencv2/face/face_alignment.hpp:54
	pub fn cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(instance: *mut c_void, f: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> bool>, user_data: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getFaces(cv::InputArray, cv::OutputArray) /usr/include/opencv2/face/face_alignment.hpp:56
	pub fn cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<bool>);
	// cascade_depth /usr/include/opencv2/face/face_alignment.hpp:21
	pub fn cv_face_FacemarkKazemi_Params_getPropCascade_depth_const(instance: *const c_void) -> u32;
	// cascade_depth /usr/include/opencv2/face/face_alignment.hpp:21
	pub fn cv_face_FacemarkKazemi_Params_setPropCascade_depth_unsigned_long(instance: *mut c_void, val: u32);
	// tree_depth /usr/include/opencv2/face/face_alignment.hpp:23
	pub fn cv_face_FacemarkKazemi_Params_getPropTree_depth_const(instance: *const c_void) -> u32;
	// tree_depth /usr/include/opencv2/face/face_alignment.hpp:23
	pub fn cv_face_FacemarkKazemi_Params_setPropTree_depth_unsigned_long(instance: *mut c_void, val: u32);
	// num_trees_per_cascade_level /usr/include/opencv2/face/face_alignment.hpp:25
	pub fn cv_face_FacemarkKazemi_Params_getPropNum_trees_per_cascade_level_const(instance: *const c_void) -> u32;
	// num_trees_per_cascade_level /usr/include/opencv2/face/face_alignment.hpp:25
	pub fn cv_face_FacemarkKazemi_Params_setPropNum_trees_per_cascade_level_unsigned_long(instance: *mut c_void, val: u32);
	// learning_rate /usr/include/opencv2/face/face_alignment.hpp:27
	pub fn cv_face_FacemarkKazemi_Params_getPropLearning_rate_const(instance: *const c_void) -> f32;
	// learning_rate /usr/include/opencv2/face/face_alignment.hpp:27
	pub fn cv_face_FacemarkKazemi_Params_setPropLearning_rate_float(instance: *mut c_void, val: f32);
	// oversampling_amount /usr/include/opencv2/face/face_alignment.hpp:29
	pub fn cv_face_FacemarkKazemi_Params_getPropOversampling_amount_const(instance: *const c_void) -> u32;
	// oversampling_amount /usr/include/opencv2/face/face_alignment.hpp:29
	pub fn cv_face_FacemarkKazemi_Params_setPropOversampling_amount_unsigned_long(instance: *mut c_void, val: u32);
	// num_test_coordinates /usr/include/opencv2/face/face_alignment.hpp:31
	pub fn cv_face_FacemarkKazemi_Params_getPropNum_test_coordinates_const(instance: *const c_void) -> u32;
	// num_test_coordinates /usr/include/opencv2/face/face_alignment.hpp:31
	pub fn cv_face_FacemarkKazemi_Params_setPropNum_test_coordinates_unsigned_long(instance: *mut c_void, val: u32);
	// lambda /usr/include/opencv2/face/face_alignment.hpp:33
	pub fn cv_face_FacemarkKazemi_Params_getPropLambda_const(instance: *const c_void) -> f32;
	// lambda /usr/include/opencv2/face/face_alignment.hpp:33
	pub fn cv_face_FacemarkKazemi_Params_setPropLambda_float(instance: *mut c_void, val: f32);
	// num_test_splits /usr/include/opencv2/face/face_alignment.hpp:35
	pub fn cv_face_FacemarkKazemi_Params_getPropNum_test_splits_const(instance: *const c_void) -> u32;
	// num_test_splits /usr/include/opencv2/face/face_alignment.hpp:35
	pub fn cv_face_FacemarkKazemi_Params_setPropNum_test_splits_unsigned_long(instance: *mut c_void, val: u32);
	// configfile /usr/include/opencv2/face/face_alignment.hpp:37
	pub fn cv_face_FacemarkKazemi_Params_getPropConfigfile_const(instance: *const c_void) -> *mut c_void;
	// configfile /usr/include/opencv2/face/face_alignment.hpp:37
	pub fn cv_face_FacemarkKazemi_Params_setPropConfigfile_String(instance: *mut c_void, val: *mut c_char);
	// Params() /usr/include/opencv2/face/face_alignment.hpp:19
	pub fn cv_face_FacemarkKazemi_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// create(const FacemarkLBF::Params &) /usr/include/opencv2/face/facemarkLBF.hpp:111
	pub fn cv_face_FacemarkLBF_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// shape_offset /usr/include/opencv2/face/facemarkLBF.hpp:58
	pub fn cv_face_FacemarkLBF_Params_getPropShape_offset_const(instance: *const c_void) -> f64;
	// shape_offset /usr/include/opencv2/face/facemarkLBF.hpp:58
	pub fn cv_face_FacemarkLBF_Params_setPropShape_offset_double(instance: *mut c_void, val: f64);
	// cascade_face /usr/include/opencv2/face/facemarkLBF.hpp:60
	pub fn cv_face_FacemarkLBF_Params_getPropCascade_face_const(instance: *const c_void) -> *mut c_void;
	// cascade_face /usr/include/opencv2/face/facemarkLBF.hpp:60
	pub fn cv_face_FacemarkLBF_Params_setPropCascade_face_String(instance: *mut c_void, val: *mut c_char);
	// verbose /usr/include/opencv2/face/facemarkLBF.hpp:62
	pub fn cv_face_FacemarkLBF_Params_getPropVerbose_const(instance: *const c_void) -> bool;
	// verbose /usr/include/opencv2/face/facemarkLBF.hpp:62
	pub fn cv_face_FacemarkLBF_Params_setPropVerbose_bool(instance: *mut c_void, val: bool);
	// n_landmarks /usr/include/opencv2/face/facemarkLBF.hpp:65
	pub fn cv_face_FacemarkLBF_Params_getPropN_landmarks_const(instance: *const c_void) -> i32;
	// n_landmarks /usr/include/opencv2/face/facemarkLBF.hpp:65
	pub fn cv_face_FacemarkLBF_Params_setPropN_landmarks_int(instance: *mut c_void, val: i32);
	// initShape_n /usr/include/opencv2/face/facemarkLBF.hpp:67
	pub fn cv_face_FacemarkLBF_Params_getPropInitShape_n_const(instance: *const c_void) -> i32;
	// initShape_n /usr/include/opencv2/face/facemarkLBF.hpp:67
	pub fn cv_face_FacemarkLBF_Params_setPropInitShape_n_int(instance: *mut c_void, val: i32);
	// stages_n /usr/include/opencv2/face/facemarkLBF.hpp:70
	pub fn cv_face_FacemarkLBF_Params_getPropStages_n_const(instance: *const c_void) -> i32;
	// stages_n /usr/include/opencv2/face/facemarkLBF.hpp:70
	pub fn cv_face_FacemarkLBF_Params_setPropStages_n_int(instance: *mut c_void, val: i32);
	// tree_n /usr/include/opencv2/face/facemarkLBF.hpp:72
	pub fn cv_face_FacemarkLBF_Params_getPropTree_n_const(instance: *const c_void) -> i32;
	// tree_n /usr/include/opencv2/face/facemarkLBF.hpp:72
	pub fn cv_face_FacemarkLBF_Params_setPropTree_n_int(instance: *mut c_void, val: i32);
	// tree_depth /usr/include/opencv2/face/facemarkLBF.hpp:74
	pub fn cv_face_FacemarkLBF_Params_getPropTree_depth_const(instance: *const c_void) -> i32;
	// tree_depth /usr/include/opencv2/face/facemarkLBF.hpp:74
	pub fn cv_face_FacemarkLBF_Params_setPropTree_depth_int(instance: *mut c_void, val: i32);
	// bagging_overlap /usr/include/opencv2/face/facemarkLBF.hpp:76
	pub fn cv_face_FacemarkLBF_Params_getPropBagging_overlap_const(instance: *const c_void) -> f64;
	// bagging_overlap /usr/include/opencv2/face/facemarkLBF.hpp:76
	pub fn cv_face_FacemarkLBF_Params_setPropBagging_overlap_double(instance: *mut c_void, val: f64);
	// model_filename /usr/include/opencv2/face/facemarkLBF.hpp:79
	pub fn cv_face_FacemarkLBF_Params_getPropModel_filename_const(instance: *const c_void) -> *mut c_void;
	// model_filename /usr/include/opencv2/face/facemarkLBF.hpp:79
	pub fn cv_face_FacemarkLBF_Params_setPropModel_filename_string(instance: *mut c_void, val: *mut c_char);
	// save_model /usr/include/opencv2/face/facemarkLBF.hpp:81
	pub fn cv_face_FacemarkLBF_Params_getPropSave_model_const(instance: *const c_void) -> bool;
	// save_model /usr/include/opencv2/face/facemarkLBF.hpp:81
	pub fn cv_face_FacemarkLBF_Params_setPropSave_model_bool(instance: *mut c_void, val: bool);
	// seed /usr/include/opencv2/face/facemarkLBF.hpp:82
	pub fn cv_face_FacemarkLBF_Params_getPropSeed_const(instance: *const c_void) -> u32;
	// seed /usr/include/opencv2/face/facemarkLBF.hpp:82
	pub fn cv_face_FacemarkLBF_Params_setPropSeed_unsigned_int(instance: *mut c_void, val: u32);
	// feats_m /usr/include/opencv2/face/facemarkLBF.hpp:84
	pub fn cv_face_FacemarkLBF_Params_getPropFeats_m_const(instance: *const c_void) -> *mut c_void;
	// feats_m /usr/include/opencv2/face/facemarkLBF.hpp:84
	pub fn cv_face_FacemarkLBF_Params_setPropFeats_m_vector_int_(instance: *mut c_void, val: *mut c_void);
	// radius_m /usr/include/opencv2/face/facemarkLBF.hpp:85
	pub fn cv_face_FacemarkLBF_Params_getPropRadius_m_const(instance: *const c_void) -> *mut c_void;
	// radius_m /usr/include/opencv2/face/facemarkLBF.hpp:85
	pub fn cv_face_FacemarkLBF_Params_setPropRadius_m_vector_double_(instance: *mut c_void, val: *mut c_void);
	// detectROI /usr/include/opencv2/face/facemarkLBF.hpp:89
	pub fn cv_face_FacemarkLBF_Params_getPropDetectROI_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
	// detectROI /usr/include/opencv2/face/facemarkLBF.hpp:89
	pub fn cv_face_FacemarkLBF_Params_setPropDetectROI_Rect(instance: *mut c_void, val: *const core::Rect);
	// Params() /usr/include/opencv2/face/facemarkLBF.hpp:56
	pub fn cv_face_FacemarkLBF_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
	// read(const cv::FileNode &) /usr/include/opencv2/face/facemarkLBF.hpp:91
	pub fn cv_face_FacemarkLBF_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/face/facemarkLBF.hpp:92
	pub fn cv_face_FacemarkLBF_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result_void);
	// addTrainingSample(cv::InputArray, cv::InputArray) /usr/include/opencv2/face/facemark_train.hpp:308
	pub fn cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(instance: *mut c_void, image: *const c_void, landmarks: *const c_void, ocvrs_return: *mut Result<bool>);
	// training(void *) /usr/include/opencv2/face/facemark_train.hpp:328
	pub fn cv_face_FacemarkTrain_training_voidX(instance: *mut c_void, parameters: *mut c_void, ocvrs_return: *mut Result_void);
	// setFaceDetector(cv::face::FN_FaceDetector, void *) /usr/include/opencv2/face/facemark_train.hpp:351
	pub fn cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(instance: *mut c_void, detector: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> bool>, user_data: *mut c_void, ocvrs_return: *mut Result<bool>);
	// getFaces(cv::InputArray, cv::OutputArray) /usr/include/opencv2/face/facemark_train.hpp:368
	pub fn cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<bool>);
	// getData(void *) /usr/include/opencv2/face/facemark_train.hpp:386
	pub fn cv_face_FacemarkTrain_getData_voidX(instance: *mut c_void, items: *mut c_void, ocvrs_return: *mut Result<bool>);
	// create(int, double) /usr/include/opencv2/face/facerec.hpp:122
	pub fn cv_face_FisherFaceRecognizer_create_int_double(num_components: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// getGridX() /usr/include/opencv2/face/facerec.hpp:130
	pub fn cv_face_LBPHFaceRecognizer_getGridX_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setGridX(int) /usr/include/opencv2/face/facerec.hpp:132
	pub fn cv_face_LBPHFaceRecognizer_setGridX_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getGridY() /usr/include/opencv2/face/facerec.hpp:134
	pub fn cv_face_LBPHFaceRecognizer_getGridY_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setGridY(int) /usr/include/opencv2/face/facerec.hpp:136
	pub fn cv_face_LBPHFaceRecognizer_setGridY_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getRadius() /usr/include/opencv2/face/facerec.hpp:138
	pub fn cv_face_LBPHFaceRecognizer_getRadius_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setRadius(int) /usr/include/opencv2/face/facerec.hpp:140
	pub fn cv_face_LBPHFaceRecognizer_setRadius_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getNeighbors() /usr/include/opencv2/face/facerec.hpp:142
	pub fn cv_face_LBPHFaceRecognizer_getNeighbors_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// setNeighbors(int) /usr/include/opencv2/face/facerec.hpp:144
	pub fn cv_face_LBPHFaceRecognizer_setNeighbors_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result_void);
	// getThreshold() /usr/include/opencv2/face/facerec.hpp:146
	pub fn cv_face_LBPHFaceRecognizer_getThreshold_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setThreshold(double) /usr/include/opencv2/face/facerec.hpp:148
	pub fn cv_face_LBPHFaceRecognizer_setThreshold_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// getHistograms() /usr/include/opencv2/face/facerec.hpp:149
	pub fn cv_face_LBPHFaceRecognizer_getHistograms_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getLabels() /usr/include/opencv2/face/facerec.hpp:150
	pub fn cv_face_LBPHFaceRecognizer_getLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, int, double) /usr/include/opencv2/face/facerec.hpp:184
	pub fn cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// salt(const cv::String &) /usr/include/opencv2/face/mace.hpp:78
	pub fn cv_face_MACE_salt_const_StringR(instance: *mut c_void, passphrase: *const c_char, ocvrs_return: *mut Result_void);
	// train(cv::InputArrayOfArrays) /usr/include/opencv2/face/mace.hpp:86
	pub fn cv_face_MACE_train_const__InputArrayR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result_void);
	// same(cv::InputArray) /usr/include/opencv2/face/mace.hpp:92
	pub fn cv_face_MACE_same_const_const__InputArrayR(instance: *const c_void, query: *const c_void, ocvrs_return: *mut Result<bool>);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/face/mace.hpp:100
	pub fn cv_face_MACE_load_const_StringR_const_StringR(filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// create(int) /usr/include/opencv2/face/mace.hpp:106
	pub fn cv_face_MACE_create_int(imgsize: i32, ocvrs_return: *mut Result<*mut c_void>);
	// init(size_t) /usr/include/opencv2/face/predict_collector.hpp:69
	pub fn cv_face_PredictCollector_init_size_t(instance: *mut c_void, size: size_t, ocvrs_return: *mut Result_void);
	// collect(int, double) /usr/include/opencv2/face/predict_collector.hpp:75
	pub fn cv_face_PredictCollector_collect_int_double(instance: *mut c_void, label: i32, dist: f64, ocvrs_return: *mut Result<bool>);
	// StandardCollector(double) /usr/include/opencv2/face/predict_collector.hpp:99
	pub fn cv_face_StandardCollector_StandardCollector_double(threshold_: f64, ocvrs_return: *mut Result<*mut c_void>);
	// init(size_t) /usr/include/opencv2/face/predict_collector.hpp:101
	pub fn cv_face_StandardCollector_init_size_t(instance: *mut c_void, size: size_t, ocvrs_return: *mut Result_void);
	// collect(int, double) /usr/include/opencv2/face/predict_collector.hpp:103
	pub fn cv_face_StandardCollector_collect_int_double(instance: *mut c_void, label: i32, dist: f64, ocvrs_return: *mut Result<bool>);
	// getMinLabel() /usr/include/opencv2/face/predict_collector.hpp:105
	pub fn cv_face_StandardCollector_getMinLabel_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getMinDist() /usr/include/opencv2/face/predict_collector.hpp:107
	pub fn cv_face_StandardCollector_getMinDist_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// create(double) /usr/include/opencv2/face/predict_collector.hpp:120
	pub fn cv_face_StandardCollector_create_double(threshold: f64, ocvrs_return: *mut Result<*mut c_void>);
	// PredictResult(int, double) /usr/include/opencv2/face/predict_collector.hpp:89
	pub fn cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_: i32, distance_: f64, ocvrs_return: *mut Result<crate::face::StandardCollector_PredictResult>);
}
