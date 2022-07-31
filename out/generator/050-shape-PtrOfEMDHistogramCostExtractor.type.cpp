extern "C" {
	void cv_PtrOfEMDHistogramCostExtractor_delete(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::EMDHistogramCostExtractor* cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::EMDHistogramCostExtractor* cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfEMDHistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

