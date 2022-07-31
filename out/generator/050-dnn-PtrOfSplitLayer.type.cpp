extern "C" {
	cv::Ptr<cv::dnn::SplitLayer>* cv_PtrOfSplitLayer_new(cv::dnn::SplitLayer* val) {
		return new cv::Ptr<cv::dnn::SplitLayer>(val);
	}
	
	void cv_PtrOfSplitLayer_delete(cv::Ptr<cv::dnn::SplitLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SplitLayer* cv_PtrOfSplitLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SplitLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SplitLayer* cv_PtrOfSplitLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SplitLayer>* instance) {
		return instance->get();
	}
}

