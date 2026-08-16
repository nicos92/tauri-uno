<script setup lang="ts">
defineProps<{
    title: string;
}>();
</script>

<template>
    <div class="page-header">
        <h1>{{ title }}</h1>
        <div class="page-header-actions">
            <slot />
        </div>
    </div>
</template>

<style scoped>
.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.page-header h1 {
    margin: 0;
}

.page-header-actions {
    display: flex;
    gap: 0.5rem;
}
</style>
