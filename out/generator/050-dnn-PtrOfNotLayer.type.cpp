extern "C" {
	cv::Ptr<cv::dnn::NotLayer>* cv_PtrOfNotLayer_new(cv::dnn::NotLayer* val) {
		return new cv::Ptr<cv::dnn::NotLayer>(val);
	}
	
	void cv_PtrOfNotLayer_delete(cv::Ptr<cv::dnn::NotLayer>* instance) {
		delete instance;
	}

	const cv::dnn::NotLayer* cv_PtrOfNotLayer_get_inner_ptr(const cv::Ptr<cv::dnn::NotLayer>* instance) {
		return instance->get();
	}

	cv::dnn::NotLayer* cv_PtrOfNotLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::NotLayer>* instance) {
		return instance->get();
	}
}

