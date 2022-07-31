extern "C" {
	void cv_VectorOfPtrOfLinemod_Modality_delete(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::linemod::Modality>>* cv_VectorOfPtrOfLinemod_Modality_new() {
		return new std::vector<cv::Ptr<cv::linemod::Modality>>();
	}

	size_t cv_VectorOfPtrOfLinemod_Modality_len(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfLinemod_Modality_is_empty(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfLinemod_Modality_capacity(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfLinemod_Modality_reserve(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfLinemod_Modality_remove(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfLinemod_Modality_swap(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfLinemod_Modality_clear(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfLinemod_Modality_push(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, cv::Ptr<cv::linemod::Modality>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfLinemod_Modality_insert(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPtrOfLinemod_Modality_get(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>** ocvrs_return) {
		*ocvrs_return = new cv::Ptr<cv::linemod::Modality>((*instance)[index]);
	}

	void cv_VectorOfPtrOfLinemod_Modality_set(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>* val) {
		(*instance)[index] = *val;
	}

}


