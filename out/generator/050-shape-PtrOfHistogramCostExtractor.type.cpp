extern "C" {
	void cv_PtrOfHistogramCostExtractor_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::HistogramCostExtractor* cv_PtrOfHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::HistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::HistogramCostExtractor* cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::HistogramCostExtractor>* instance) {
		return instance->get();
	}
}

