extern "C" {
	cv::Ptr<cv::dnn::AsinhLayer>* cv_PtrOfAsinhLayer_new(cv::dnn::AsinhLayer* val) {
		return new cv::Ptr<cv::dnn::AsinhLayer>(val);
	}
	
	void cv_PtrOfAsinhLayer_delete(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AsinhLayer* cv_PtrOfAsinhLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AsinhLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AsinhLayer* cv_PtrOfAsinhLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
		return instance->get();
	}
}

