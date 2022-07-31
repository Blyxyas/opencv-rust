extern "C" {
	cv::Ptr<cv::dnn::PermuteLayer>* cv_PtrOfPermuteLayer_new(cv::dnn::PermuteLayer* val) {
		return new cv::Ptr<cv::dnn::PermuteLayer>(val);
	}
	
	void cv_PtrOfPermuteLayer_delete(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PermuteLayer* cv_PtrOfPermuteLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PermuteLayer* cv_PtrOfPermuteLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PermuteLayer>* instance) {
		return instance->get();
	}
}

