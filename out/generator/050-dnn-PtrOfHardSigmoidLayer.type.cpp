extern "C" {
	cv::Ptr<cv::dnn::HardSigmoidLayer>* cv_PtrOfHardSigmoidLayer_new(cv::dnn::HardSigmoidLayer* val) {
		return new cv::Ptr<cv::dnn::HardSigmoidLayer>(val);
	}
	
	void cv_PtrOfHardSigmoidLayer_delete(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
		delete instance;
	}

	const cv::dnn::HardSigmoidLayer* cv_PtrOfHardSigmoidLayer_get_inner_ptr(const cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
		return instance->get();
	}

	cv::dnn::HardSigmoidLayer* cv_PtrOfHardSigmoidLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
		return instance->get();
	}
}

