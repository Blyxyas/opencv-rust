extern "C" {
	cv::Ptr<cv::dnn::FloorLayer>* cv_PtrOfFloorLayer_new(cv::dnn::FloorLayer* val) {
		return new cv::Ptr<cv::dnn::FloorLayer>(val);
	}
	
	void cv_PtrOfFloorLayer_delete(cv::Ptr<cv::dnn::FloorLayer>* instance) {
		delete instance;
	}

	const cv::dnn::FloorLayer* cv_PtrOfFloorLayer_get_inner_ptr(const cv::Ptr<cv::dnn::FloorLayer>* instance) {
		return instance->get();
	}

	cv::dnn::FloorLayer* cv_PtrOfFloorLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::FloorLayer>* instance) {
		return instance->get();
	}
}

