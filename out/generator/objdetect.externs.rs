extern "C" {
	// createFaceDetectionMaskGenerator() /usr/include/opencv2/objdetect.hpp:342
	pub fn cv_createFaceDetectionMaskGenerator(ocvrs_return: *mut Result<*mut c_void>);
	// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, cv::Size) /usr/include/opencv2/objdetect.hpp:166
	pub fn cv_groupRectangles_meanshift_vector_Rect_R_vector_double_R_vector_double_R_double_Size(rect_list: *mut c_void, found_weights: *mut c_void, found_scales: *mut c_void, detect_threshold: f64, win_det_size: *const core::Size, ocvrs_return: *mut Result_void);
	// groupRectangles(std::vector<Rect> &, int, double) /usr/include/opencv2/objdetect.hpp:155
	pub fn cv_groupRectangles_vector_Rect_R_int_double(rect_list: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result_void);
	// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *) /usr/include/opencv2/objdetect.hpp:160
	pub fn cv_groupRectangles_vector_Rect_R_int_double_vector_int_X_vector_double_X(rect_list: *mut c_void, group_threshold: i32, eps: f64, weights: *mut c_void, level_weights: *mut c_void, ocvrs_return: *mut Result_void);
	// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double) /usr/include/opencv2/objdetect.hpp:157
	pub fn cv_groupRectangles_vector_Rect_R_vector_int_R_int_double(rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result_void);
	// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double) /usr/include/opencv2/objdetect.hpp:163
	pub fn cv_groupRectangles_vector_Rect_R_vector_int_R_vector_double_R_int_double(rect_list: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/objdetect.hpp:186
	pub fn cv_BaseCascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// load(const cv::String &) /usr/include/opencv2/objdetect.hpp:187
	pub fn cv_BaseCascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:188
	pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:194
	pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, cv::Size, cv::Size, bool) /usr/include/opencv2/objdetect.hpp:201
	pub fn cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result_void);
	// isOldFormatCascade() /usr/include/opencv2/objdetect.hpp:210
	pub fn cv_BaseCascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getOriginalWindowSize() /usr/include/opencv2/objdetect.hpp:211
	pub fn cv_BaseCascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getFeatureType() /usr/include/opencv2/objdetect.hpp:212
	pub fn cv_BaseCascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getOldCascade() /usr/include/opencv2/objdetect.hpp:213
	pub fn cv_BaseCascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setMaskGenerator(const Ptr<cv::BaseCascadeClassifier::MaskGenerator> &) /usr/include/opencv2/objdetect.hpp:222
	pub fn cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result_void);
	// getMaskGenerator() /usr/include/opencv2/objdetect.hpp:223
	pub fn cv_BaseCascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// generateMask(const cv::Mat &) /usr/include/opencv2/objdetect.hpp:219
	pub fn cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// initializeMask(const cv::Mat &) /usr/include/opencv2/objdetect.hpp:220
	pub fn cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result_void);
	// cc /usr/include/opencv2/objdetect.hpp:339
	pub fn cv_CascadeClassifier_getPropCc(instance: *mut c_void) -> *mut c_void;
	// cc /usr/include/opencv2/objdetect.hpp:339
	pub fn cv_CascadeClassifier_setPropCc_Ptr_BaseCascadeClassifier_(instance: *mut c_void, val: *mut c_void);
	// CascadeClassifier() /usr/include/opencv2/objdetect.hpp:235
	pub fn cv_CascadeClassifier_CascadeClassifier(ocvrs_return: *mut Result<*mut c_void>);
	// CascadeClassifier(const cv::String &) /usr/include/opencv2/objdetect.hpp:240
	pub fn cv_CascadeClassifier_CascadeClassifier_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/objdetect.hpp:244
	pub fn cv_CascadeClassifier_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// load(const cv::String &) /usr/include/opencv2/objdetect.hpp:251
	pub fn cv_CascadeClassifier_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result<bool>);
	// read(const cv::FileNode &) /usr/include/opencv2/objdetect.hpp:256
	pub fn cv_CascadeClassifier_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<bool>);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:272
	pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:294
	pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(instance: *mut c_void, image: *const c_void, objects: *mut c_void, num_detections: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, cv::Size, cv::Size, bool) /usr/include/opencv2/objdetect.hpp:319
	pub fn cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(instance: *mut c_void, image: *const c_void, objects: *mut c_void, reject_levels: *mut c_void, level_weights: *mut c_void, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: *const core::Size, max_size: *const core::Size, output_reject_levels: bool, ocvrs_return: *mut Result_void);
	// isOldFormatCascade() /usr/include/opencv2/objdetect.hpp:329
	pub fn cv_CascadeClassifier_isOldFormatCascade_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getOriginalWindowSize() /usr/include/opencv2/objdetect.hpp:330
	pub fn cv_CascadeClassifier_getOriginalWindowSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getFeatureType() /usr/include/opencv2/objdetect.hpp:331
	pub fn cv_CascadeClassifier_getFeatureType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getOldCascade() /usr/include/opencv2/objdetect.hpp:332
	pub fn cv_CascadeClassifier_getOldCascade(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// convert(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:334
	pub fn cv_CascadeClassifier_convert_const_StringR_const_StringR(oldcascade: *const c_char, newcascade: *const c_char, ocvrs_return: *mut Result<bool>);
	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &) /usr/include/opencv2/objdetect.hpp:336
	pub fn cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(instance: *mut c_void, mask_generator: *const c_void, ocvrs_return: *mut Result_void);
	// getMaskGenerator() /usr/include/opencv2/objdetect.hpp:337
	pub fn cv_CascadeClassifier_getMaskGenerator(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const cv::DetectionBasedTracker::Parameters &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:121
	pub fn cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersR(main_detector: *mut c_void, tracking_detector: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// run() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:124
	pub fn cv_DetectionBasedTracker_run(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// stop() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:125
	pub fn cv_DetectionBasedTracker_stop(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// resetTracking() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:126
	pub fn cv_DetectionBasedTracker_resetTracking(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// process(const cv::Mat &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:128
	pub fn cv_DetectionBasedTracker_process_const_MatR(instance: *mut c_void, image_gray: *const c_void, ocvrs_return: *mut Result_void);
	// setParameters(const cv::DetectionBasedTracker::Parameters &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:130
	pub fn cv_DetectionBasedTracker_setParameters_const_ParametersR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
	// getParameters() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:131
	pub fn cv_DetectionBasedTracker_getParameters_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getObjects(std::vector<cv::Rect> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:135
	pub fn cv_DetectionBasedTracker_getObjects_const_vector_Rect_R(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result_void);
	// getObjects(std::vector<ExtObject> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:155
	pub fn cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_R(instance: *const c_void, result: *mut c_void, ocvrs_return: *mut Result_void);
	// addObject(const cv::Rect &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:158
	pub fn cv_DetectionBasedTracker_addObject_const_RectR(instance: *mut c_void, location: *const core::Rect, ocvrs_return: *mut Result<i32>);
	// id /usr/include/opencv2/objdetect/detection_based_tracker.hpp:147
	pub fn cv_DetectionBasedTracker_ExtObject_getPropId_const(instance: *const c_void) -> i32;
	// id /usr/include/opencv2/objdetect/detection_based_tracker.hpp:147
	pub fn cv_DetectionBasedTracker_ExtObject_setPropId_int(instance: *mut c_void, val: i32);
	// location /usr/include/opencv2/objdetect/detection_based_tracker.hpp:148
	pub fn cv_DetectionBasedTracker_ExtObject_getPropLocation_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
	// location /usr/include/opencv2/objdetect/detection_based_tracker.hpp:148
	pub fn cv_DetectionBasedTracker_ExtObject_setPropLocation_Rect(instance: *mut c_void, val: *const core::Rect);
	// status /usr/include/opencv2/objdetect/detection_based_tracker.hpp:149
	pub fn cv_DetectionBasedTracker_ExtObject_getPropStatus_const(instance: *const c_void, ocvrs_return: *mut crate::objdetect::DetectionBasedTracker_ObjectStatus);
	// status /usr/include/opencv2/objdetect/detection_based_tracker.hpp:149
	pub fn cv_DetectionBasedTracker_ExtObject_setPropStatus_ObjectStatus(instance: *mut c_void, val: crate::objdetect::DetectionBasedTracker_ObjectStatus);
	// ExtObject(int, cv::Rect, cv::DetectionBasedTracker::ObjectStatus) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:150
	pub fn cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id: i32, _location: *const core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus, ocvrs_return: *mut Result<*mut c_void>);
	// detect(const cv::Mat &, std::vector<cv::Rect> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:78
	pub fn cv_DetectionBasedTracker_IDetector_detect_const_MatR_vector_Rect_R(instance: *mut c_void, image: *const c_void, objects: *mut c_void, ocvrs_return: *mut Result_void);
	// setMinObjectSize(const cv::Size &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:80
	pub fn cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(instance: *mut c_void, min: *const core::Size, ocvrs_return: *mut Result_void);
	// setMaxObjectSize(const cv::Size &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:84
	pub fn cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(instance: *mut c_void, max: *const core::Size, ocvrs_return: *mut Result_void);
	// getMinObjectSize() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:88
	pub fn cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getMaxObjectSize() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:92
	pub fn cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// getScaleFactor() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:96
	pub fn cv_DetectionBasedTracker_IDetector_getScaleFactor(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setScaleFactor(float) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:100
	pub fn cv_DetectionBasedTracker_IDetector_setScaleFactor_float(instance: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// getMinNeighbours() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:104
	pub fn cv_DetectionBasedTracker_IDetector_getMinNeighbours(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setMinNeighbours(int) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:108
	pub fn cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// maxTrackLifetime /usr/include/opencv2/objdetect/detection_based_tracker.hpp:62
	pub fn cv_DetectionBasedTracker_Parameters_getPropMaxTrackLifetime_const(instance: *const c_void) -> i32;
	// maxTrackLifetime /usr/include/opencv2/objdetect/detection_based_tracker.hpp:62
	pub fn cv_DetectionBasedTracker_Parameters_setPropMaxTrackLifetime_int(instance: *mut c_void, val: i32);
	// minDetectionPeriod /usr/include/opencv2/objdetect/detection_based_tracker.hpp:63
	pub fn cv_DetectionBasedTracker_Parameters_getPropMinDetectionPeriod_const(instance: *const c_void) -> i32;
	// minDetectionPeriod /usr/include/opencv2/objdetect/detection_based_tracker.hpp:63
	pub fn cv_DetectionBasedTracker_Parameters_setPropMinDetectionPeriod_int(instance: *mut c_void, val: i32);
	// Parameters() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:65
	pub fn cv_DetectionBasedTracker_Parameters_Parameters(ocvrs_return: *mut Result<*mut c_void>);
	// scale /usr/include/opencv2/objdetect.hpp:353
	pub fn cv_DetectionROI_getPropScale_const(instance: *const c_void) -> f64;
	// scale /usr/include/opencv2/objdetect.hpp:353
	pub fn cv_DetectionROI_setPropScale_double(instance: *mut c_void, val: f64);
	// locations /usr/include/opencv2/objdetect.hpp:355
	pub fn cv_DetectionROI_getPropLocations_const(instance: *const c_void) -> *mut c_void;
	// locations /usr/include/opencv2/objdetect.hpp:355
	pub fn cv_DetectionROI_setPropLocations_vector_Point_(instance: *mut c_void, val: *mut c_void);
	// confidences /usr/include/opencv2/objdetect.hpp:357
	pub fn cv_DetectionROI_getPropConfidences_const(instance: *const c_void) -> *mut c_void;
	// confidences /usr/include/opencv2/objdetect.hpp:357
	pub fn cv_DetectionROI_setPropConfidences_vector_double_(instance: *mut c_void, val: *mut c_void);
	// setInputSize(const cv::Size &) /usr/include/opencv2/objdetect/face.hpp:29
	pub fn cv_FaceDetectorYN_setInputSize_const_SizeR(instance: *mut c_void, input_size: *const core::Size, ocvrs_return: *mut Result_void);
	// getInputSize() /usr/include/opencv2/objdetect/face.hpp:31
	pub fn cv_FaceDetectorYN_getInputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
	// setScoreThreshold(float) /usr/include/opencv2/objdetect/face.hpp:37
	pub fn cv_FaceDetectorYN_setScoreThreshold_float(instance: *mut c_void, score_threshold: f32, ocvrs_return: *mut Result_void);
	// getScoreThreshold() /usr/include/opencv2/objdetect/face.hpp:39
	pub fn cv_FaceDetectorYN_getScoreThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setNMSThreshold(float) /usr/include/opencv2/objdetect/face.hpp:45
	pub fn cv_FaceDetectorYN_setNMSThreshold_float(instance: *mut c_void, nms_threshold: f32, ocvrs_return: *mut Result_void);
	// getNMSThreshold() /usr/include/opencv2/objdetect/face.hpp:47
	pub fn cv_FaceDetectorYN_getNMSThreshold(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setTopK(int) /usr/include/opencv2/objdetect/face.hpp:53
	pub fn cv_FaceDetectorYN_setTopK_int(instance: *mut c_void, top_k: i32, ocvrs_return: *mut Result_void);
	// getTopK() /usr/include/opencv2/objdetect/face.hpp:55
	pub fn cv_FaceDetectorYN_getTopK(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:62
	pub fn cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, image: *const c_void, faces: *const c_void, ocvrs_return: *mut Result<i32>);
	// create(const cv::String &, const cv::String &, const cv::Size &, float, float, int, int, int) /usr/include/opencv2/objdetect/face.hpp:75
	pub fn cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(model: *const c_char, config: *const c_char, input_size: *const core::Size, score_threshold: f32, nms_threshold: f32, top_k: i32, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// alignCrop(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:103
	pub fn cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *const c_void, src_img: *const c_void, face_box: *const c_void, aligned_img: *const c_void, ocvrs_return: *mut Result_void);
	// feature(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:109
	pub fn cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, aligned_img: *const c_void, face_feature: *const c_void, ocvrs_return: *mut Result_void);
	// match(cv::InputArray, cv::InputArray, int) /usr/include/opencv2/objdetect/face.hpp:116
	pub fn cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(instance: *const c_void, face_feature1: *const c_void, face_feature2: *const c_void, dis_type: i32, ocvrs_return: *mut Result<f64>);
	// create(const cv::String &, const cv::String &, int, int) /usr/include/opencv2/objdetect/face.hpp:124
	pub fn cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(model: *const c_char, config: *const c_char, backend_id: i32, target_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// winSize /usr/include/opencv2/objdetect.hpp:596
	pub fn cv_HOGDescriptor_getPropWinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// winSize /usr/include/opencv2/objdetect.hpp:596
	pub fn cv_HOGDescriptor_setPropWinSize_Size(instance: *mut c_void, val: *const core::Size);
	// blockSize /usr/include/opencv2/objdetect.hpp:599
	pub fn cv_HOGDescriptor_getPropBlockSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// blockSize /usr/include/opencv2/objdetect.hpp:599
	pub fn cv_HOGDescriptor_setPropBlockSize_Size(instance: *mut c_void, val: *const core::Size);
	// blockStride /usr/include/opencv2/objdetect.hpp:602
	pub fn cv_HOGDescriptor_getPropBlockStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// blockStride /usr/include/opencv2/objdetect.hpp:602
	pub fn cv_HOGDescriptor_setPropBlockStride_Size(instance: *mut c_void, val: *const core::Size);
	// cellSize /usr/include/opencv2/objdetect.hpp:605
	pub fn cv_HOGDescriptor_getPropCellSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// cellSize /usr/include/opencv2/objdetect.hpp:605
	pub fn cv_HOGDescriptor_setPropCellSize_Size(instance: *mut c_void, val: *const core::Size);
	// nbins /usr/include/opencv2/objdetect.hpp:608
	pub fn cv_HOGDescriptor_getPropNbins_const(instance: *const c_void) -> i32;
	// nbins /usr/include/opencv2/objdetect.hpp:608
	pub fn cv_HOGDescriptor_setPropNbins_int(instance: *mut c_void, val: i32);
	// derivAperture /usr/include/opencv2/objdetect.hpp:611
	pub fn cv_HOGDescriptor_getPropDerivAperture_const(instance: *const c_void) -> i32;
	// derivAperture /usr/include/opencv2/objdetect.hpp:611
	pub fn cv_HOGDescriptor_setPropDerivAperture_int(instance: *mut c_void, val: i32);
	// winSigma /usr/include/opencv2/objdetect.hpp:614
	pub fn cv_HOGDescriptor_getPropWinSigma_const(instance: *const c_void) -> f64;
	// winSigma /usr/include/opencv2/objdetect.hpp:614
	pub fn cv_HOGDescriptor_setPropWinSigma_double(instance: *mut c_void, val: f64);
	// histogramNormType /usr/include/opencv2/objdetect.hpp:617
	pub fn cv_HOGDescriptor_getPropHistogramNormType_const(instance: *const c_void, ocvrs_return: *mut crate::objdetect::HOGDescriptor_HistogramNormType);
	// histogramNormType /usr/include/opencv2/objdetect.hpp:617
	pub fn cv_HOGDescriptor_setPropHistogramNormType_HistogramNormType(instance: *mut c_void, val: crate::objdetect::HOGDescriptor_HistogramNormType);
	// L2HysThreshold /usr/include/opencv2/objdetect.hpp:620
	pub fn cv_HOGDescriptor_getPropL2HysThreshold_const(instance: *const c_void) -> f64;
	// L2HysThreshold /usr/include/opencv2/objdetect.hpp:620
	pub fn cv_HOGDescriptor_setPropL2HysThreshold_double(instance: *mut c_void, val: f64);
	// gammaCorrection /usr/include/opencv2/objdetect.hpp:623
	pub fn cv_HOGDescriptor_getPropGammaCorrection_const(instance: *const c_void) -> bool;
	// gammaCorrection /usr/include/opencv2/objdetect.hpp:623
	pub fn cv_HOGDescriptor_setPropGammaCorrection_bool(instance: *mut c_void, val: bool);
	// svmDetector /usr/include/opencv2/objdetect.hpp:626
	pub fn cv_HOGDescriptor_getPropSvmDetector_const(instance: *const c_void) -> *mut c_void;
	// svmDetector /usr/include/opencv2/objdetect.hpp:626
	pub fn cv_HOGDescriptor_setPropSvmDetector_vector_float_(instance: *mut c_void, val: *mut c_void);
	// oclSvmDetector /usr/include/opencv2/objdetect.hpp:629
	pub fn cv_HOGDescriptor_getPropOclSvmDetector_const(instance: *const c_void) -> *mut c_void;
	// oclSvmDetector /usr/include/opencv2/objdetect.hpp:629
	pub fn cv_HOGDescriptor_setPropOclSvmDetector_UMat(instance: *mut c_void, val: *mut c_void);
	// free_coef /usr/include/opencv2/objdetect.hpp:632
	pub fn cv_HOGDescriptor_getPropFree_coef_const(instance: *const c_void) -> f32;
	// free_coef /usr/include/opencv2/objdetect.hpp:632
	pub fn cv_HOGDescriptor_setPropFree_coef_float(instance: *mut c_void, val: f32);
	// nlevels /usr/include/opencv2/objdetect.hpp:635
	pub fn cv_HOGDescriptor_getPropNlevels_const(instance: *const c_void) -> i32;
	// nlevels /usr/include/opencv2/objdetect.hpp:635
	pub fn cv_HOGDescriptor_setPropNlevels_int(instance: *mut c_void, val: i32);
	// signedGradient /usr/include/opencv2/objdetect.hpp:638
	pub fn cv_HOGDescriptor_getPropSignedGradient_const(instance: *const c_void) -> bool;
	// signedGradient /usr/include/opencv2/objdetect.hpp:638
	pub fn cv_HOGDescriptor_setPropSignedGradient_bool(instance: *mut c_void, val: bool);
	// HOGDescriptor() /usr/include/opencv2/objdetect.hpp:390
	pub fn cv_HOGDescriptor_HOGDescriptor(ocvrs_return: *mut Result<*mut c_void>);
	// HOGDescriptor(cv::Size, cv::Size, cv::Size, cv::Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool) /usr/include/opencv2/objdetect.hpp:410
	pub fn cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(_win_size: *const core::Size, _block_size: *const core::Size, _block_stride: *const core::Size, _cell_size: *const core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: crate::objdetect::HOGDescriptor_HistogramNormType, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
	// HOGDescriptor(const cv::String &) /usr/include/opencv2/objdetect.hpp:426
	pub fn cv_HOGDescriptor_HOGDescriptor_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// HOGDescriptor(const cv::HOGDescriptor &) /usr/include/opencv2/objdetect.hpp:434
	pub fn cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDescriptorSize() /usr/include/opencv2/objdetect.hpp:445
	pub fn cv_HOGDescriptor_getDescriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// checkDetectorSize() /usr/include/opencv2/objdetect.hpp:449
	pub fn cv_HOGDescriptor_checkDetectorSize_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getWinSigma() /usr/include/opencv2/objdetect.hpp:453
	pub fn cv_HOGDescriptor_getWinSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setSVMDetector(cv::InputArray) /usr/include/opencv2/objdetect.hpp:460
	pub fn cv_HOGDescriptor_setSVMDetector_const__InputArrayR(instance: *mut c_void, svmdetector: *const c_void, ocvrs_return: *mut Result_void);
	// read(cv::FileNode &) /usr/include/opencv2/objdetect.hpp:465
	pub fn cv_HOGDescriptor_read_FileNodeR(instance: *mut c_void, fn_: *mut c_void, ocvrs_return: *mut Result<bool>);
	// write(cv::FileStorage &, const cv::String &) /usr/include/opencv2/objdetect.hpp:471
	pub fn cv_HOGDescriptor_write_const_FileStorageR_const_StringR(instance: *const c_void, fs: *mut c_void, objname: *const c_char, ocvrs_return: *mut Result_void);
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:477
	pub fn cv_HOGDescriptor_load_const_StringR_const_StringR(instance: *mut c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result<bool>);
	// save(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:483
	pub fn cv_HOGDescriptor_save_const_const_StringR_const_StringR(instance: *const c_void, filename: *const c_char, objname: *const c_char, ocvrs_return: *mut Result_void);
	// copyTo(cv::HOGDescriptor &) /usr/include/opencv2/objdetect.hpp:488
	pub fn cv_HOGDescriptor_copyTo_const_HOGDescriptorR(instance: *const c_void, c: *mut c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, std::vector<float> &, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:499
	pub fn cv_HOGDescriptor_compute_const_const__InputArrayR_vector_float_R_Size_Size_const_vector_Point_R(instance: *const c_void, img: *const c_void, descriptors: *mut c_void, win_stride: *const core::Size, padding: *const core::Size, locations: *const c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, std::vector<Point> &, std::vector<double> &, double, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:515
	pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, std::vector<Point> &, double, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:531
	pub fn cv_HOGDescriptor_detect_const_const__InputArrayR_vector_Point_R_double_Size_Size_const_vector_Point_R(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, search_locations: *const c_void, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<double> &, double, cv::Size, cv::Size, double, double, bool) /usr/include/opencv2/objdetect.hpp:551
	pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_vector_double_R_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, found_weights: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result_void);
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, cv::Size, cv::Size, double, double, bool) /usr/include/opencv2/objdetect.hpp:570
	pub fn cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_double_Size_Size_double_double_bool(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, scale: f64, group_threshold: f64, use_meanshift_grouping: bool, ocvrs_return: *mut Result_void);
	// computeGradient(cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:582
	pub fn cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(instance: *const c_void, img: *const c_void, grad: *const c_void, angle_ofs: *const c_void, padding_tl: *const core::Size, padding_br: *const core::Size, ocvrs_return: *mut Result_void);
	// getDefaultPeopleDetector() /usr/include/opencv2/objdetect.hpp:587
	pub fn cv_HOGDescriptor_getDefaultPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
	// getDaimlerPeopleDetector() /usr/include/opencv2/objdetect.hpp:593
	pub fn cv_HOGDescriptor_getDaimlerPeopleDetector(ocvrs_return: *mut Result<*mut c_void>);
	// detectROI(cv::InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:651
	pub fn cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vector_Point_R_vector_Point_R_vector_double_R_double_Size_Size(instance: *const c_void, img: *const c_void, locations: *const c_void, found_locations: *mut c_void, confidences: *mut c_void, hit_threshold: f64, win_stride: *const core::Size, padding: *const core::Size, ocvrs_return: *mut Result_void);
	// detectMultiScaleROI(cv::InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int) /usr/include/opencv2/objdetect.hpp:664
	pub fn cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vector_Rect_R_vector_DetectionROI_R_double_int(instance: *const c_void, img: *const c_void, found_locations: *mut c_void, locations: *mut c_void, hit_threshold: f64, group_threshold: i32, ocvrs_return: *mut Result_void);
	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double) /usr/include/opencv2/objdetect.hpp:676
	pub fn cv_HOGDescriptor_groupRectangles_const_vector_Rect_R_vector_double_R_int_double(instance: *const c_void, rect_list: *mut c_void, weights: *mut c_void, group_threshold: i32, eps: f64, ocvrs_return: *mut Result_void);
	// QRCodeDetector() /usr/include/opencv2/objdetect.hpp:749
	pub fn cv_QRCodeDetector_QRCodeDetector(ocvrs_return: *mut Result<*mut c_void>);
	// setEpsX(double) /usr/include/opencv2/objdetect.hpp:756
	pub fn cv_QRCodeDetector_setEpsX_double(instance: *mut c_void, eps_x: f64, ocvrs_return: *mut Result_void);
	// setEpsY(double) /usr/include/opencv2/objdetect.hpp:761
	pub fn cv_QRCodeDetector_setEpsY_double(instance: *mut c_void, eps_y: f64, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:767
	pub fn cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
	// decode(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:776
	pub fn cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// decodeCurved(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:785
	pub fn cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detectAndDecode(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:793
	pub fn cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detectAndDecodeCurved(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:802
	pub fn cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// detectMulti(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:810
	pub fn cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<bool>);
	// decodeMulti(cv::InputArray, cv::InputArray, std::vector<std::string> &, cv::OutputArrayOfArrays) /usr/include/opencv2/objdetect.hpp:819
	pub fn cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_string_R_const__OutputArrayR(instance: *const c_void, img: *const c_void, points: *const c_void, decoded_info: *mut c_void, straight_qrcode: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(const QRCodeEncoder::Params &) /usr/include/opencv2/objdetect.hpp:730
	pub fn cv_QRCodeEncoder_create_const_ParamsR(parameters: *const crate::objdetect::QRCodeEncoder_Params, ocvrs_return: *mut Result<*mut c_void>);
	// encode(const cv::String &, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:736
	pub fn cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcode: *const c_void, ocvrs_return: *mut Result_void);
	// encodeStructuredAppend(const cv::String &, cv::OutputArrayOfArrays) /usr/include/opencv2/objdetect.hpp:742
	pub fn cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(instance: *mut c_void, encoded_info: *const c_char, qrcodes: *const c_void, ocvrs_return: *mut Result_void);
	// Params() /usr/include/opencv2/objdetect.hpp:719
	pub fn cv_QRCodeEncoder_Params_Params(ocvrs_return: *mut Result<crate::objdetect::QRCodeEncoder_Params>);
	// eps /usr/include/opencv2/objdetect.hpp:137
	pub fn cv_SimilarRects_getPropEps_const(instance: *const c_void) -> f64;
	// eps /usr/include/opencv2/objdetect.hpp:137
	pub fn cv_SimilarRects_setPropEps_double(instance: *mut c_void, val: f64);
	// SimilarRects(double) /usr/include/opencv2/objdetect.hpp:128
	pub fn cv_SimilarRects_SimilarRects_double(_eps: f64, ocvrs_return: *mut Result<*mut c_void>);
}
