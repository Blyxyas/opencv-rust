extern "C" {
	cv::Ptr<cv::dnn::SwishLayer>* cv_PtrOfSwishLayer_new(cv::dnn::SwishLayer* val) {
		return new cv::Ptr<cv::dnn::SwishLayer>(val);
	}
	
	void cv_PtrOfSwishLayer_delete(cv::Ptr<cv::dnn::SwishLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SwishLayer* cv_PtrOfSwishLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SwishLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SwishLayer* cv_PtrOfSwishLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SwishLayer>* instance) {
		return instance->get();
	}
}

