<script setup lang="ts">
import { ref, watch, nextTick, onBeforeUnmount } from "vue";
import { useConfirm } from "../composables/useConfirm";

const { active, resolveActive } = useConfirm();

const confirmButton = ref<HTMLButtonElement | null>(null);

watch(active, async (pending) => {
  if (pending) {
    await nextTick();
    confirmButton.value?.focus();
  }
});

function onCancel() {
  resolveActive(false);
}

function onConfirm() {
  resolveActive(true);
}

onBeforeUnmount(() => {
  resolveActive(false);
});
</script>

<template>
  <Teleport to="body">
    <div
      v-if="active"
      class="confirm-overlay"
      role="dialog"
      aria-modal="true"
      @click.self="onCancel"
      @keydown.esc="onCancel"
    >
      <div class="confirm-modal">
        <h2>{{ active.options.title }}</h2>
        <p class="confirm-message">{{ active.options.message }}</p>
        <div class="confirm-actions">
          <button type="button" class="btn-secondary" @click="onCancel">
            {{ active.options.cancelText }}
          </button>
          <button
            ref="confirmButton"
            type="button"
            :class="[
              'btn-primary',
              active.options.variant === 'danger' ? 'btn-danger' : '',
            ]"
            @click="onConfirm"
          >
            {{ active.options.confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1500;
}

.confirm-modal {
  background: var(--color-surface);
  padding: 2rem;
  border-radius: 12px;
  width: 100%;
  max-width: 420px;
}

.confirm-modal h2 {
  margin: 0 0 1rem;
}

.confirm-message {
  margin: 0;
}

.confirm-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.btn-primary {
  background: #3F2281;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
}

.btn-primary:hover {
  background: #5568d3;
}

.btn-primary.btn-danger {
  background: transparent;
  border: 1px solid #c53030;
}

.btn-primary.btn-danger:hover {
  background: #c53030;
}

.btn-secondary {
  background: var(--color-surface);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
}

.btn-secondary:hover {
  background: var(--color-border);
}
</style>
