extern "C" {
	void cv_PtrOfRNNLayer_delete(cv::Ptr<cv::dnn::RNNLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RNNLayer* cv_PtrOfRNNLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RNNLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RNNLayer* cv_PtrOfRNNLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RNNLayer>* instance) {
		return instance->get();
	}
}

