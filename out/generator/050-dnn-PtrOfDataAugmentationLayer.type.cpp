extern "C" {
	cv::Ptr<cv::dnn::DataAugmentationLayer>* cv_PtrOfDataAugmentationLayer_new(cv::dnn::DataAugmentationLayer* val) {
		return new cv::Ptr<cv::dnn::DataAugmentationLayer>(val);
	}
	
	void cv_PtrOfDataAugmentationLayer_delete(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::DataAugmentationLayer* cv_PtrOfDataAugmentationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::DataAugmentationLayer* cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::DataAugmentationLayer>* instance) {
		return instance->get();
	}
}

