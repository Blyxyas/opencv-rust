extern "C" {
	cv::Ptr<cv::dnn::DetectionOutputLayer>* cv_PtrOfDetectionOutputLayer_new(cv::dnn::DetectionOutputLayer* val) {
		return new cv::Ptr<cv::dnn::DetectionOutputLayer>(val);
	}
	
	void cv_PtrOfDetectionOutputLayer_delete(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		delete instance;
	}

	const cv::dnn::DetectionOutputLayer* cv_PtrOfDetectionOutputLayer_get_inner_ptr(const cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		return instance->get();
	}

	cv::dnn::DetectionOutputLayer* cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::DetectionOutputLayer>* instance) {
		return instance->get();
	}
}

