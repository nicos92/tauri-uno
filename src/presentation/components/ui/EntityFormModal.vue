<script setup lang="ts">
import Modal from "./Modal.vue";

defineProps<{
    modelValue: boolean;
    title: string;
    error?: string | null;
    submitLabel: string;
    disableSubmit?: boolean;
    maxWidth?: string;
}>();

const emit = defineEmits<{
    "update:modelValue": [value: boolean];
    submit: [];
}>();

function onClose() {
    emit("update:modelValue", false);
}
</script>

<template>
    <Modal
        :model-value="modelValue"
        :title="title"
        :max-width="maxWidth"
        @update:model-value="onClose"
    >
        <form @submit.prevent="emit('submit')">
            <slot />
            <slot name="extra" />
            <div v-if="error" class="error-message">{{ error }}</div>
            <div class="modal-actions">
                <button type="button" class="btn-secondary" @click="onClose">
                    Cancelar
                </button>
                <button type="submit" class="btn-primary" :disabled="disableSubmit">
                    {{ submitLabel }}
                </button>
            </div>
        </form>
    </Modal>
</template>

<style scoped>
.modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
}
</style>
