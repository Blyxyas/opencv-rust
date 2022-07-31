extern "C" {
	void cv_PtrOfLSTMLayer_delete(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		delete instance;
	}

	const cv::dnn::LSTMLayer* cv_PtrOfLSTMLayer_get_inner_ptr(const cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		return instance->get();
	}

	cv::dnn::LSTMLayer* cv_PtrOfLSTMLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
		return instance->get();
	}
}

