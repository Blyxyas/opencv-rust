extern "C" {
	cv::Ptr<cv::face::StandardCollector>* cv_PtrOfStandardCollector_new(cv::face::StandardCollector* val) {
		return new cv::Ptr<cv::face::StandardCollector>(val);
	}
	
	void cv_PtrOfStandardCollector_delete(cv::Ptr<cv::face::StandardCollector>* instance) {
		delete instance;
	}

	const cv::face::StandardCollector* cv_PtrOfStandardCollector_get_inner_ptr(const cv::Ptr<cv::face::StandardCollector>* instance) {
		return instance->get();
	}

	cv::face::StandardCollector* cv_PtrOfStandardCollector_get_inner_ptr_mut(cv::Ptr<cv::face::StandardCollector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::face::PredictCollector>* cv_PtrOfStandardCollector_to_PtrOfPredictCollector(cv::Ptr<cv::face::StandardCollector>* instance) {
		return new cv::Ptr<cv::face::PredictCollector>(instance->dynamicCast<cv::face::PredictCollector>());
	}
}

