<script setup lang="ts">
defineProps<{
    modelValue: string;
    placeholder?: string;
}>();

const emit = defineEmits<{
    "update:modelValue": [value: string];
}>();

function onInput(event: Event) {
    emit("update:modelValue", (event.target as HTMLInputElement).value);
}
</script>

<template>
    <div class="search-bar">
        <input
            :value="modelValue"
            type="text"
            :placeholder="placeholder"
            class="search-input"
            @input="onInput"
        />
    </div>
</template>

<style scoped>
.search-bar {
    margin-bottom: 1.5rem;
}

.search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.search-input:focus {
    outline: none;
    border-color: var(--color-primary);
}
</style>
