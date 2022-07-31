extern "C" {
	void cv_PtrOfShapeContextDistanceExtractor_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		delete instance;
	}

	const cv::ShapeContextDistanceExtractor* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(const cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		return instance->get();
	}

	cv::ShapeContextDistanceExtractor* cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
		return instance->get();
	}
}

