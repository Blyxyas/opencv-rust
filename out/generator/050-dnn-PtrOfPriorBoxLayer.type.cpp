extern "C" {
	cv::Ptr<cv::dnn::PriorBoxLayer>* cv_PtrOfPriorBoxLayer_new(cv::dnn::PriorBoxLayer* val) {
		return new cv::Ptr<cv::dnn::PriorBoxLayer>(val);
	}
	
	void cv_PtrOfPriorBoxLayer_delete(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PriorBoxLayer* cv_PtrOfPriorBoxLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PriorBoxLayer* cv_PtrOfPriorBoxLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
		return instance->get();
	}
}

