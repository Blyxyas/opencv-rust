extern "C" {
	cv::Ptr<cv::dnn::CeluLayer>* cv_PtrOfCeluLayer_new(cv::dnn::CeluLayer* val) {
		return new cv::Ptr<cv::dnn::CeluLayer>(val);
	}
	
	void cv_PtrOfCeluLayer_delete(cv::Ptr<cv::dnn::CeluLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CeluLayer* cv_PtrOfCeluLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CeluLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CeluLayer* cv_PtrOfCeluLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CeluLayer>* instance) {
		return instance->get();
	}
}

