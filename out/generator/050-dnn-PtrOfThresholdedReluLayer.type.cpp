extern "C" {
	cv::Ptr<cv::dnn::ThresholdedReluLayer>* cv_PtrOfThresholdedReluLayer_new(cv::dnn::ThresholdedReluLayer* val) {
		return new cv::Ptr<cv::dnn::ThresholdedReluLayer>(val);
	}
	
	void cv_PtrOfThresholdedReluLayer_delete(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ThresholdedReluLayer* cv_PtrOfThresholdedReluLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ThresholdedReluLayer* cv_PtrOfThresholdedReluLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
		return instance->get();
	}
}

