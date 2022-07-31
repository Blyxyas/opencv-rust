extern "C" {
	void cv_PtrOfEMDL1HistogramCostExtractor_delete(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		delete instance;
	}

	const cv::EMDL1HistogramCostExtractor* cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr(const cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return instance->get();
	}

	cv::EMDL1HistogramCostExtractor* cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr_mut(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrOfEMDL1HistogramCostExtractor_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
		return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
}

