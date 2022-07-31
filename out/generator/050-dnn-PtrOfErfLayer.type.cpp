extern "C" {
	cv::Ptr<cv::dnn::ErfLayer>* cv_PtrOfErfLayer_new(cv::dnn::ErfLayer* val) {
		return new cv::Ptr<cv::dnn::ErfLayer>(val);
	}
	
	void cv_PtrOfErfLayer_delete(cv::Ptr<cv::dnn::ErfLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ErfLayer* cv_PtrOfErfLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ErfLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ErfLayer* cv_PtrOfErfLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ErfLayer>* instance) {
		return instance->get();
	}
}

