extern "C" {
	cv::Ptr<cv::dnn::ShrinkLayer>* cv_PtrOfShrinkLayer_new(cv::dnn::ShrinkLayer* val) {
		return new cv::Ptr<cv::dnn::ShrinkLayer>(val);
	}
	
	void cv_PtrOfShrinkLayer_delete(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ShrinkLayer* cv_PtrOfShrinkLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ShrinkLayer* cv_PtrOfShrinkLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ShrinkLayer>* instance) {
		return instance->get();
	}
}

