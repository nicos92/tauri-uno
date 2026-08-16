<script setup lang="ts">
import { useConfirm } from "../../composables/useConfirm";

const props = defineProps<{
    message: string;
}>();

const emit = defineEmits<{
    confirmed: [];
}>();

const { confirm } = useConfirm();

async function onClick() {
    if (await confirm({ message: props.message })) {
        emit("confirmed");
    }
}
</script>

<template>
    <button class="btn-icon btn-danger" title="Eliminar" @click="onClick">
        <img src="/svg/trash.svg" alt="Eliminar" />
    </button>
</template>
