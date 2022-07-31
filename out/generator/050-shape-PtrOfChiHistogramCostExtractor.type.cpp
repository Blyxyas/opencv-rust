extern "C" {
	void cv_PtrOfChiHistogramCostExtractor_delete(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::ChiHistogramCostExtractor* cv_PtrOfChiHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::ChiHistogramCostExtractor* cv_PtrOfChiHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfChiHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

