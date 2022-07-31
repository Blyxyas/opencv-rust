extern "C" {
	cv::Ptr<cv::dnn::RegionLayer>* cv_PtrOfRegionLayer_new(cv::dnn::RegionLayer* val) {
		return new cv::Ptr<cv::dnn::RegionLayer>(val);
	}
	
	void cv_PtrOfRegionLayer_delete(cv::Ptr<cv::dnn::RegionLayer>* instance) {
		delete instance;
	}

	const cv::dnn::RegionLayer* cv_PtrOfRegionLayer_get_inner_ptr(const cv::Ptr<cv::dnn::RegionLayer>* instance) {
		return instance->get();
	}

	cv::dnn::RegionLayer* cv_PtrOfRegionLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::RegionLayer>* instance) {
		return instance->get();
	}
}

