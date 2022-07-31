extern "C" {
	void cv_PtrOfShapeTransformer_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
		delete instance;
	}

	const cv::ShapeTransformer* cv_PtrOfShapeTransformer_get_inner_ptr(const cv::Ptr<cv::ShapeTransformer>* instance) {
		return instance->get();
	}

	cv::ShapeTransformer* cv_PtrOfShapeTransformer_get_inner_ptr_mut(cv::Ptr<cv::ShapeTransformer>* instance) {
		return instance->get();
	}
}

