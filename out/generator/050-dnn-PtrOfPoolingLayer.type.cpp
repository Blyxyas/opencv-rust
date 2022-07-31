extern "C" {
	cv::Ptr<cv::dnn::PoolingLayer>* cv_PtrOfPoolingLayer_new(cv::dnn::PoolingLayer* val) {
		return new cv::Ptr<cv::dnn::PoolingLayer>(val);
	}
	
	void cv_PtrOfPoolingLayer_delete(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PoolingLayer* cv_PtrOfPoolingLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PoolingLayer* cv_PtrOfPoolingLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
		return instance->get();
	}
}

