extern "C" {
	cv::Ptr<cv::videostab::WeightingDeblurer>* cv_PtrOfWeightingDeblurer_new(cv::videostab::WeightingDeblurer* val) {
		return new cv::Ptr<cv::videostab::WeightingDeblurer>(val);
	}
	
	void cv_PtrOfWeightingDeblurer_delete(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		delete instance;
	}

	const cv::videostab::WeightingDeblurer* cv_PtrOfWeightingDeblurer_get_inner_ptr(const cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return instance->get();
	}

	cv::videostab::WeightingDeblurer* cv_PtrOfWeightingDeblurer_get_inner_ptr_mut(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrOfWeightingDeblurer_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
		return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
}

