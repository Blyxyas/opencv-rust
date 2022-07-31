extern "C" {
	void cv_VectorOfGPCPatchDescriptor_delete(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		delete instance;
	}

	std::vector<cv::optflow::GPCPatchDescriptor>* cv_VectorOfGPCPatchDescriptor_new() {
		return new std::vector<cv::optflow::GPCPatchDescriptor>();
	}

	size_t cv_VectorOfGPCPatchDescriptor_len(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGPCPatchDescriptor_is_empty(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGPCPatchDescriptor_capacity(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfGPCPatchDescriptor_shrink_to_fit(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGPCPatchDescriptor_reserve(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGPCPatchDescriptor_remove(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGPCPatchDescriptor_swap(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGPCPatchDescriptor_clear(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		instance->clear();
	}

	void cv_VectorOfGPCPatchDescriptor_push(std::vector<cv::optflow::GPCPatchDescriptor>* instance, cv::optflow::GPCPatchDescriptor* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGPCPatchDescriptor_insert(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfGPCPatchDescriptor_get(const std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor** ocvrs_return) {
		*ocvrs_return = new cv::optflow::GPCPatchDescriptor((*instance)[index]);
	}

	void cv_VectorOfGPCPatchDescriptor_set(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor* val) {
		(*instance)[index] = *val;
	}

}


