extern "C" {
	cv::Ptr<cv::dnn::SigmoidLayer>* cv_PtrOfSigmoidLayer_new(cv::dnn::SigmoidLayer* val) {
		return new cv::Ptr<cv::dnn::SigmoidLayer>(val);
	}
	
	void cv_PtrOfSigmoidLayer_delete(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SigmoidLayer* cv_PtrOfSigmoidLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SigmoidLayer* cv_PtrOfSigmoidLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
		return instance->get();
	}
}

