extern "C" {
	cv::Ptr<cv::dnn::CumSumLayer>* cv_PtrOfCumSumLayer_new(cv::dnn::CumSumLayer* val) {
		return new cv::Ptr<cv::dnn::CumSumLayer>(val);
	}
	
	void cv_PtrOfCumSumLayer_delete(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CumSumLayer* cv_PtrOfCumSumLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CumSumLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CumSumLayer* cv_PtrOfCumSumLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CumSumLayer>* instance) {
		return instance->get();
	}
}

