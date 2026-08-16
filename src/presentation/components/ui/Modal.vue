<script setup lang="ts">
defineProps<{
    modelValue: boolean;
    title: string;
    maxWidth?: string;
}>();

const emit = defineEmits<{
    "update:modelValue": [value: boolean];
}>();

function close() {
    emit("update:modelValue", false);
}
</script>

<template>
    <Teleport to="body">
        <div v-if="modelValue" class="modal-overlay" @click.self="close">
            <div class="modal" :style="{ maxWidth: maxWidth || '500px' }">
                <h2>{{ title }}</h2>
                <slot />
            </div>
        </div>
    </Teleport>
</template>

<style scoped>
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background: var(--color-surface);
    padding: 2rem;
    border-radius: 12px;
    width: 100%;
    max-height: 90vh;
    overflow-y: auto;
}

.modal h2 {
    margin: 0 0 1.5rem;
}
</style>
