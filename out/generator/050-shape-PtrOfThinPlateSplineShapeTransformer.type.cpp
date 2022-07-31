extern "C" {
	void cv_PtrOfThinPlateSplineShapeTransformer_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		delete instance;
	}

	const cv::ThinPlateSplineShapeTransformer* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(const cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return instance->get();
	}

	cv::ThinPlateSplineShapeTransformer* cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::ShapeTransformer>* cv_PtrOfThinPlateSplineShapeTransformer_to_PtrOfShapeTransformer(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
		return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}
}

