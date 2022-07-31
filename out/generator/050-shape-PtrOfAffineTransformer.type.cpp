extern "C" {
	void cv_PtrOfAffineTransformer_delete(cv::Ptr<cv::AffineTransformer>* instance) {
		delete instance;
	}

	const cv::AffineTransformer* cv_PtrOfAffineTransformer_get_inner_ptr(const cv::Ptr<cv::AffineTransformer>* instance) {
		return instance->get();
	}

	cv::AffineTransformer* cv_PtrOfAffineTransformer_get_inner_ptr_mut(cv::Ptr<cv::AffineTransformer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrOfAffineTransformer_to_PtrOfShapeTransformer(cv::Ptr<cv::AffineTransformer>* instance) {
		return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
}

