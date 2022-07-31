extern "C" {
	cv::Ptr<cv::dnn::ReduceLayer>* cv_PtrOfReduceLayer_new(cv::dnn::ReduceLayer* val) {
		return new cv::Ptr<cv::dnn::ReduceLayer>(val);
	}
	
	void cv_PtrOfReduceLayer_delete(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReduceLayer* cv_PtrOfReduceLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReduceLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReduceLayer* cv_PtrOfReduceLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReduceLayer>* instance) {
		return instance->get();
	}
}

