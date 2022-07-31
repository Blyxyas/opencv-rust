extern "C" {
	void cv_PtrOfNormHistogramCostExtractor_delete(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::NormHistogramCostExtractor* cv_PtrOfNormHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::NormHistogramCostExtractor* cv_PtrOfNormHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfNormHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

