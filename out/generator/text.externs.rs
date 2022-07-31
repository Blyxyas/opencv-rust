extern "C" {
	// MSERsToERStats(cv::InputArray, std::vector<std::vector<Point>> &, std::vector<std::vector<ERStat>> &) /usr/include/opencv2/text/erfilter.hpp:347
	pub fn cv_text_MSERsToERStats_const__InputArrayR_vector_vector_Point__R_vector_vector_ERStat__R(image: *const c_void, contours: *mut c_void, regions: *mut c_void, ocvrs_return: *mut Result_void);
	// computeNMChannels(cv::InputArray, cv::OutputArrayOfArrays, int) /usr/include/opencv2/text/erfilter.hpp:262
	pub fn cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR_int(_src: *const c_void, _channels: *const c_void, _mode: i32, ocvrs_return: *mut Result_void);
	// createERFilterNM1(const Ptr<ERFilter::Callback> &, int, float, float, float, bool, float) /usr/include/opencv2/text/erfilter.hpp:187
	pub fn cv_text_createERFilterNM1_const_Ptr_Callback_R_int_float_float_float_bool_float(cb: *const c_void, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createERFilterNM1(const cv::String &, int, float, float, float, bool, float) /usr/include/opencv2/text/erfilter.hpp:212
	pub fn cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float(filename: *const c_char, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createERFilterNM2(const Ptr<ERFilter::Callback> &, float) /usr/include/opencv2/text/erfilter.hpp:204
	pub fn cv_text_createERFilterNM2_const_Ptr_Callback_R_float(cb: *const c_void, min_probability: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createERFilterNM2(const cv::String &, float) /usr/include/opencv2/text/erfilter.hpp:223
	pub fn cv_text_createERFilterNM2_const_StringR_float(filename: *const c_char, min_probability: f32, ocvrs_return: *mut Result<*mut c_void>);
	// createOCRHMMTransitionsTable(const cv::String &, std::vector<cv::String> &) /usr/include/opencv2/text/ocr.hpp:381
	pub fn cv_text_createOCRHMMTransitionsTable_const_StringR_vector_String_R(vocabulary: *const c_char, lexicon: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createOCRHMMTransitionsTable(std::string &, std::vector<std::string> &, cv::OutputArray) /usr/include/opencv2/text/ocr.hpp:379
	pub fn cv_text_createOCRHMMTransitionsTable_stringR_vector_string_R_const__OutputArrayR(vocabulary: *mut *mut c_void, lexicon: *mut c_void, transition_probabilities_table: *const c_void, ocvrs_return: *mut Result_void);
	// detectRegions(cv::InputArray, const Ptr<cv::text::ERFilter> &, const Ptr<cv::text::ERFilter> &, std::vector<Rect> &, int, const cv::String &, float) /usr/include/opencv2/text/erfilter.hpp:366
	pub fn cv_text_detectRegions_const__InputArrayR_const_Ptr_ERFilter_R_const_Ptr_ERFilter_R_vector_Rect_R_int_const_StringR_float(image: *const c_void, er_filter1: *const c_void, er_filter2: *const c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probability: f32, ocvrs_return: *mut Result_void);
	// detectRegions(cv::InputArray, const Ptr<cv::text::ERFilter> &, const Ptr<cv::text::ERFilter> &, std::vector<std::vector<Point>> &) /usr/include/opencv2/text/erfilter.hpp:351
	pub fn cv_text_detectRegions_const__InputArrayR_const_Ptr_ERFilter_R_const_Ptr_ERFilter_R_vector_vector_Point__R(image: *const c_void, er_filter1: *const c_void, er_filter2: *const c_void, regions: *mut c_void, ocvrs_return: *mut Result_void);
	// detectTextSWT(cv::InputArray, std::vector<cv::Rect> &, bool, const cv::_OutputArray &, const cv::_OutputArray &) /usr/include/opencv2/text/swt_text_detection.hpp:20
	pub fn cv_text_detectTextSWT_const__InputArrayR_vector_Rect_R_bool_const__OutputArrayR_const__OutputArrayR(input: *const c_void, result: *mut c_void, dark_on_light: bool, draw: *const c_void, chain_b_bs: *const c_void, ocvrs_return: *mut Result_void);
	// erGrouping(cv::InputArray, cv::InputArrayOfArrays, std::vector<std::vector<ERStat>> &, std::vector<std::vector<Vec2i>> &, std::vector<Rect> &, int, const std::string &, float) /usr/include/opencv2/text/erfilter.hpp:316
	pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vector_vector_ERStat__R_vector_vector_Vec2i__R_vector_Rect_R_int_const_stringR_float(img: *const c_void, channels: *const c_void, regions: *mut c_void, groups: *mut c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probablity: f32, ocvrs_return: *mut Result_void);
	// erGrouping(cv::InputArray, cv::InputArray, std::vector<std::vector<Point>>, std::vector<Rect> &, int, const cv::String &, float) /usr/include/opencv2/text/erfilter.hpp:324
	pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vector_vector_Point___vector_Rect_R_int_const_StringR_float(image: *const c_void, channel: *const c_void, regions: *mut c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probablity: f32, ocvrs_return: *mut Result_void);
	// loadClassifierNM1(const cv::String &) /usr/include/opencv2/text/erfilter.hpp:232
	pub fn cv_text_loadClassifierNM1_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// loadClassifierNM2(const cv::String &) /usr/include/opencv2/text/erfilter.hpp:240
	pub fn cv_text_loadClassifierNM2_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// loadOCRBeamSearchClassifierCNN(const cv::String &) /usr/include/opencv2/text/ocr.hpp:525
	pub fn cv_text_loadOCRBeamSearchClassifierCNN_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// loadOCRHMMClassifierCNN(const cv::String &) /usr/include/opencv2/text/ocr.hpp:354
	pub fn cv_text_loadOCRHMMClassifierCNN_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// loadOCRHMMClassifierNM(const cv::String &) /usr/include/opencv2/text/ocr.hpp:341
	pub fn cv_text_loadOCRHMMClassifierNM_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// loadOCRHMMClassifier(const cv::String &, int) /usr/include/opencv2/text/ocr.hpp:363
	pub fn cv_text_loadOCRHMMClassifier_const_StringR_int(filename: *const c_char, classifier: i32, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:96
	pub fn cv_text_BaseOCR_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:99
	pub fn cv_text_BaseOCR_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, std::vector<ERStat> &) /usr/include/opencv2/text/erfilter.hpp:151
	pub fn cv_text_ERFilter_run_const__InputArrayR_vector_ERStat_R(instance: *mut c_void, image: *const c_void, regions: *mut c_void, ocvrs_return: *mut Result_void);
	// setCallback(const Ptr<ERFilter::Callback> &) /usr/include/opencv2/text/erfilter.hpp:155
	pub fn cv_text_ERFilter_setCallback_const_Ptr_Callback_R(instance: *mut c_void, cb: *const c_void, ocvrs_return: *mut Result_void);
	// setThresholdDelta(int) /usr/include/opencv2/text/erfilter.hpp:156
	pub fn cv_text_ERFilter_setThresholdDelta_int(instance: *mut c_void, threshold_delta: i32, ocvrs_return: *mut Result_void);
	// setMinArea(float) /usr/include/opencv2/text/erfilter.hpp:157
	pub fn cv_text_ERFilter_setMinArea_float(instance: *mut c_void, min_area: f32, ocvrs_return: *mut Result_void);
	// setMaxArea(float) /usr/include/opencv2/text/erfilter.hpp:158
	pub fn cv_text_ERFilter_setMaxArea_float(instance: *mut c_void, max_area: f32, ocvrs_return: *mut Result_void);
	// setMinProbability(float) /usr/include/opencv2/text/erfilter.hpp:159
	pub fn cv_text_ERFilter_setMinProbability_float(instance: *mut c_void, min_probability: f32, ocvrs_return: *mut Result_void);
	// setMinProbabilityDiff(float) /usr/include/opencv2/text/erfilter.hpp:160
	pub fn cv_text_ERFilter_setMinProbabilityDiff_float(instance: *mut c_void, min_probability_diff: f32, ocvrs_return: *mut Result_void);
	// setNonMaxSuppression(bool) /usr/include/opencv2/text/erfilter.hpp:161
	pub fn cv_text_ERFilter_setNonMaxSuppression_bool(instance: *mut c_void, non_max_suppression: bool, ocvrs_return: *mut Result_void);
	// getNumRejected() /usr/include/opencv2/text/erfilter.hpp:162
	pub fn cv_text_ERFilter_getNumRejected_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// eval(const cv::text::ERStat &) /usr/include/opencv2/text/erfilter.hpp:135
	pub fn cv_text_ERFilter_Callback_eval_const_ERStatR(instance: *mut c_void, stat: *const c_void, ocvrs_return: *mut Result<f64>);
	// pixel /usr/include/opencv2/text/erfilter.hpp:75
	pub fn cv_text_ERStat_getPropPixel_const(instance: *const c_void) -> i32;
	// pixel /usr/include/opencv2/text/erfilter.hpp:75
	pub fn cv_text_ERStat_setPropPixel_int(instance: *mut c_void, val: i32);
	// level /usr/include/opencv2/text/erfilter.hpp:76
	pub fn cv_text_ERStat_getPropLevel_const(instance: *const c_void) -> i32;
	// level /usr/include/opencv2/text/erfilter.hpp:76
	pub fn cv_text_ERStat_setPropLevel_int(instance: *mut c_void, val: i32);
	// area /usr/include/opencv2/text/erfilter.hpp:79
	pub fn cv_text_ERStat_getPropArea_const(instance: *const c_void) -> i32;
	// area /usr/include/opencv2/text/erfilter.hpp:79
	pub fn cv_text_ERStat_setPropArea_int(instance: *mut c_void, val: i32);
	// perimeter /usr/include/opencv2/text/erfilter.hpp:80
	pub fn cv_text_ERStat_getPropPerimeter_const(instance: *const c_void) -> i32;
	// perimeter /usr/include/opencv2/text/erfilter.hpp:80
	pub fn cv_text_ERStat_setPropPerimeter_int(instance: *mut c_void, val: i32);
	// euler /usr/include/opencv2/text/erfilter.hpp:81
	pub fn cv_text_ERStat_getPropEuler_const(instance: *const c_void) -> i32;
	// euler /usr/include/opencv2/text/erfilter.hpp:81
	pub fn cv_text_ERStat_setPropEuler_int(instance: *mut c_void, val: i32);
	// rect /usr/include/opencv2/text/erfilter.hpp:82
	pub fn cv_text_ERStat_getPropRect_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
	// rect /usr/include/opencv2/text/erfilter.hpp:82
	pub fn cv_text_ERStat_setPropRect_Rect(instance: *mut c_void, val: *const core::Rect);
	// raw_moments /usr/include/opencv2/text/erfilter.hpp:83
	pub fn cv_text_ERStat_getPropRaw_moments(instance: *mut c_void) -> *mut [f64; 2];
	// central_moments /usr/include/opencv2/text/erfilter.hpp:84
	pub fn cv_text_ERStat_getPropCentral_moments(instance: *mut c_void) -> *mut [f64; 3];
	// med_crossings /usr/include/opencv2/text/erfilter.hpp:86
	pub fn cv_text_ERStat_getPropMed_crossings_const(instance: *const c_void) -> f32;
	// med_crossings /usr/include/opencv2/text/erfilter.hpp:86
	pub fn cv_text_ERStat_setPropMed_crossings_float(instance: *mut c_void, val: f32);
	// hole_area_ratio /usr/include/opencv2/text/erfilter.hpp:89
	pub fn cv_text_ERStat_getPropHole_area_ratio_const(instance: *const c_void) -> f32;
	// hole_area_ratio /usr/include/opencv2/text/erfilter.hpp:89
	pub fn cv_text_ERStat_setPropHole_area_ratio_float(instance: *mut c_void, val: f32);
	// convex_hull_ratio /usr/include/opencv2/text/erfilter.hpp:90
	pub fn cv_text_ERStat_getPropConvex_hull_ratio_const(instance: *const c_void) -> f32;
	// convex_hull_ratio /usr/include/opencv2/text/erfilter.hpp:90
	pub fn cv_text_ERStat_setPropConvex_hull_ratio_float(instance: *mut c_void, val: f32);
	// num_inflexion_points /usr/include/opencv2/text/erfilter.hpp:91
	pub fn cv_text_ERStat_getPropNum_inflexion_points_const(instance: *const c_void) -> f32;
	// num_inflexion_points /usr/include/opencv2/text/erfilter.hpp:91
	pub fn cv_text_ERStat_setPropNum_inflexion_points_float(instance: *mut c_void, val: f32);
	// probability /usr/include/opencv2/text/erfilter.hpp:100
	pub fn cv_text_ERStat_getPropProbability_const(instance: *const c_void) -> f64;
	// probability /usr/include/opencv2/text/erfilter.hpp:100
	pub fn cv_text_ERStat_setPropProbability_double(instance: *mut c_void, val: f64);
	// parent /usr/include/opencv2/text/erfilter.hpp:103
	pub fn cv_text_ERStat_getPropParent(instance: *mut c_void) -> *mut c_void;
	// parent /usr/include/opencv2/text/erfilter.hpp:103
	pub fn cv_text_ERStat_setPropParent_ERStatX(instance: *mut c_void, val: *mut c_void);
	// child /usr/include/opencv2/text/erfilter.hpp:104
	pub fn cv_text_ERStat_getPropChild(instance: *mut c_void) -> *mut c_void;
	// child /usr/include/opencv2/text/erfilter.hpp:104
	pub fn cv_text_ERStat_setPropChild_ERStatX(instance: *mut c_void, val: *mut c_void);
	// next /usr/include/opencv2/text/erfilter.hpp:105
	pub fn cv_text_ERStat_getPropNext(instance: *mut c_void) -> *mut c_void;
	// next /usr/include/opencv2/text/erfilter.hpp:105
	pub fn cv_text_ERStat_setPropNext_ERStatX(instance: *mut c_void, val: *mut c_void);
	// prev /usr/include/opencv2/text/erfilter.hpp:106
	pub fn cv_text_ERStat_getPropPrev(instance: *mut c_void) -> *mut c_void;
	// prev /usr/include/opencv2/text/erfilter.hpp:106
	pub fn cv_text_ERStat_setPropPrev_ERStatX(instance: *mut c_void, val: *mut c_void);
	// local_maxima /usr/include/opencv2/text/erfilter.hpp:109
	pub fn cv_text_ERStat_getPropLocal_maxima_const(instance: *const c_void) -> bool;
	// local_maxima /usr/include/opencv2/text/erfilter.hpp:109
	pub fn cv_text_ERStat_setPropLocal_maxima_bool(instance: *mut c_void, val: bool);
	// max_probability_ancestor /usr/include/opencv2/text/erfilter.hpp:110
	pub fn cv_text_ERStat_getPropMax_probability_ancestor(instance: *mut c_void) -> *mut c_void;
	// max_probability_ancestor /usr/include/opencv2/text/erfilter.hpp:110
	pub fn cv_text_ERStat_setPropMax_probability_ancestor_ERStatX(instance: *mut c_void, val: *mut c_void);
	// min_probability_ancestor /usr/include/opencv2/text/erfilter.hpp:111
	pub fn cv_text_ERStat_getPropMin_probability_ancestor(instance: *mut c_void) -> *mut c_void;
	// min_probability_ancestor /usr/include/opencv2/text/erfilter.hpp:111
	pub fn cv_text_ERStat_setPropMin_probability_ancestor_ERStatX(instance: *mut c_void, val: *mut c_void);
	// ERStat(int, int, int, int) /usr/include/opencv2/text/erfilter.hpp:70
	pub fn cv_text_ERStat_ERStat_int_int_int_int(level: i32, pixel: i32, x: i32, y: i32, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:446
	pub fn cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:450
	pub fn cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:455
	pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:457
	pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const Ptr<OCRBeamSearchDecoder::ClassifierCallback>, const std::string &, cv::InputArray, cv::InputArray, text::decoder_mode, int) /usr/include/opencv2/text/ocr.hpp:478
	pub fn cv_text_OCRBeamSearchDecoder_create_const_Ptr_ClassifierCallback__const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: crate::text::decoder_mode, beam_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::String &, const cv::String &, cv::InputArray, cv::InputArray, text::decoder_mode, int) /usr/include/opencv2/text/ocr.hpp:495
	pub fn cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: crate::text::decoder_mode, beam_size: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eval(cv::InputArray, std::vector<std::vector<double>> &, std::vector<int> &) /usr/include/opencv2/text/ocr.hpp:418
	pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vector_vector_double__R_vector_int_R(instance: *mut c_void, image: *const c_void, recognition_probabilities: *mut c_void, oversegmentation: *mut c_void, ocvrs_return: *mut Result_void);
	// getWindowSize() /usr/include/opencv2/text/ocr.hpp:420
	pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// getStepSize() /usr/include/opencv2/text/ocr.hpp:421
	pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// run(cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:241
	pub fn cv_text_OCRHMMDecoder_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:268
	pub fn cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:273
	pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:275
	pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const Ptr<OCRHMMDecoder::ClassifierCallback>, const cv::String &, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/text/ocr.hpp:294
	pub fn cv_text_OCRHMMDecoder_create_const_Ptr_ClassifierCallback__const_StringR_const__InputArrayR_const__InputArrayR_int(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::String &, const cv::String &, cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:307
	pub fn cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, classifier: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eval(cv::InputArray, std::vector<int> &, std::vector<double> &) /usr/include/opencv2/text/ocr.hpp:216
	pub fn cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vector_int_R_vector_double_R(instance: *mut c_void, image: *const c_void, out_class: *mut c_void, out_confidence: *mut c_void, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:539
	pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:569
	pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// create(const std::string &, const std::string &, const std::string &) /usr/include/opencv2/text/ocr.hpp:579
	pub fn cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(arch_filename: *const c_char, weights_filename: *const c_char, words_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:135
	pub fn cv_text_OCRTesseract_run_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::Mat &, cv::Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int) /usr/include/opencv2/text/ocr.hpp:139
	pub fn cv_text_OCRTesseract_run_MatR_MatR_stringR_vector_Rect_X_vector_string_X_vector_float_X_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result_void);
	// run(cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:144
	pub fn cv_text_OCRTesseract_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// run(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/text/ocr.hpp:146
	pub fn cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
	// setWhiteList(const cv::String &) /usr/include/opencv2/text/ocr.hpp:148
	pub fn cv_text_OCRTesseract_setWhiteList_const_StringR(instance: *mut c_void, char_whitelist: *const c_char, ocvrs_return: *mut Result_void);
	// create(const char *, const char *, const char *, int, int) /usr/include/opencv2/text/ocr.hpp:165
	pub fn cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(datapath: *const c_char, language: *const c_char, char_whitelist: *const c_char, oem: i32, psmode: i32, ocvrs_return: *mut Result<*mut c_void>);
	// detect(cv::InputArray, std::vector<Rect> &, std::vector<float> &) /usr/include/opencv2/text/textDetector.hpp:30
	pub fn cv_text_TextDetector_detect_const__InputArrayR_vector_Rect_R_vector_float_R(instance: *mut c_void, input_image: *const c_void, bbox: *mut c_void, confidence: *mut c_void, ocvrs_return: *mut Result_void);
	// detect(cv::InputArray, std::vector<Rect> &, std::vector<float> &) /usr/include/opencv2/text/textDetector.hpp:51
	pub fn cv_text_TextDetectorCNN_detect_const__InputArrayR_vector_Rect_R_vector_float_R(instance: *mut c_void, input_image: *const c_void, bbox: *mut c_void, confidence: *mut c_void, ocvrs_return: *mut Result_void);
	// create(const cv::String &, const cv::String &, std::vector<Size>) /usr/include/opencv2/text/textDetector.hpp:60
	pub fn cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vector_Size_(model_arch_filename: *const c_char, model_weights_filename: *const c_char, detection_sizes: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::String &, const cv::String &) /usr/include/opencv2/text/textDetector.hpp:65
	pub fn cv_text_TextDetectorCNN_create_const_StringR_const_StringR(model_arch_filename: *const c_char, model_weights_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
}
