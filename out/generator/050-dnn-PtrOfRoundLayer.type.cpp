extern "C" {
	cv::Ptr<cv::dnn::RoundLayer>* cv_PtrOfRoundLayer_new(cv::dnn::RoundLayer* val) {
		return new cv::Ptr<cv::dnn::RoundLayer>(val);
	}
	
	void cv_PtrOfRoundLayer_delete(cv::Ptr<cv::dnn::RoundLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RoundLayer* cv_PtrOfRoundLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RoundLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RoundLayer* cv_PtrOfRoundLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RoundLayer>* instance) {
		return instance->get();
	}
}

