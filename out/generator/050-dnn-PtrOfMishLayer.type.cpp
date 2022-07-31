extern "C" {
	cv::Ptr<cv::dnn::MishLayer>* cv_PtrOfMishLayer_new(cv::dnn::MishLayer* val) {
		return new cv::Ptr<cv::dnn::MishLayer>(val);
	}
	
	void cv_PtrOfMishLayer_delete(cv::Ptr<cv::dnn::MishLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MishLayer* cv_PtrOfMishLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MishLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MishLayer* cv_PtrOfMishLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MishLayer>* instance) {
		return instance->get();
	}
}

