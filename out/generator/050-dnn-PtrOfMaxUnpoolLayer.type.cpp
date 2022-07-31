extern "C" {
	cv::Ptr<cv::dnn::MaxUnpoolLayer>* cv_PtrOfMaxUnpoolLayer_new(cv::dnn::MaxUnpoolLayer* val) {
		return new cv::Ptr<cv::dnn::MaxUnpoolLayer>(val);
	}
	
	void cv_PtrOfMaxUnpoolLayer_delete(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		delete instance;
	}

	const cv::dnn::MaxUnpoolLayer* cv_PtrOfMaxUnpoolLayer_get_inner_ptr(const cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		return instance->get();
	}

	cv::dnn::MaxUnpoolLayer* cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::MaxUnpoolLayer>* instance) {
		return instance->get();
	}
}

