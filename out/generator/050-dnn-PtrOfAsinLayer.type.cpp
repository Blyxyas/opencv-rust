extern "C" {
	cv::Ptr<cv::dnn::AsinLayer>* cv_PtrOfAsinLayer_new(cv::dnn::AsinLayer* val) {
		return new cv::Ptr<cv::dnn::AsinLayer>(val);
	}
	
	void cv_PtrOfAsinLayer_delete(cv::Ptr<cv::dnn::AsinLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AsinLayer* cv_PtrOfAsinLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AsinLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AsinLayer* cv_PtrOfAsinLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AsinLayer>* instance) {
		return instance->get();
	}
}

