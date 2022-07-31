extern "C" {
	void cv_PtrOfPredictCollector_delete(cv::Ptr<cv::face::PredictCollector>* instance) {
		delete instance;
	}

	const cv::face::PredictCollector* cv_PtrOfPredictCollector_get_inner_ptr(const cv::Ptr<cv::face::PredictCollector>* instance) {
		return instance->get();
	}

	cv::face::PredictCollector* cv_PtrOfPredictCollector_get_inner_ptr_mut(cv::Ptr<cv::face::PredictCollector>* instance) {
		return instance->get();
	}
}

