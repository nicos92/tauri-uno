<script setup lang="ts">
import type { Cliente } from "../../../domain/entities";
import type { CartItem } from "../../composables/useCart";
import { formatMoney } from "../../utils/format";
import { clienteLabel } from "../../utils/cliente";

defineProps<{
  fecha: string;
  cliente: Cliente | null;
  items: CartItem[];
  subtotal: number;
  descuento: number;
  descuentoMonto: number;
  total: number;
  observacion: string;
}>();
</script>

<template>
    <Teleport to="body">
        <div class="print-area" id="print-area">
            <h1>Presupuesto</h1>
            <p>Fecha: {{ fecha }}</p>
            <p v-if="cliente">
                Cliente: {{ clienteLabel(cliente) }}
            </p>
            <div class="print-summary">
                <p class="print-line">Subtotal: {{ formatMoney(subtotal) }}</p>
                <p v-if="descuento > 0" class="print-line">
                    Descuento ({{ descuento }}%): −{{ formatMoney(descuentoMonto) }}
                </p>
                <p class="print-total">Total: {{ formatMoney(total) }}</p>
                <p v-if="observacion" class="print-obs">
                    Observación: {{ observacion }}
                </p>
            </div>
            <table>
                <thead>
                    <tr>
                        <th>Código</th>
                        <th>Artículo</th>
                        <th>Cantidad</th>
                        <th>Precio</th>
                        <th>Subtotal</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in items" :key="item.id_articulo">
                        <td>{{ item.cod_articulo }}</td>
                        <td>{{ item.articulo }}</td>
                        <td>{{ item.cantidad }}</td>
                        <td>{{ formatMoney(item.precio) }}</td>
                        <td>{{ formatMoney(item.subtotal) }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </Teleport>
</template>
